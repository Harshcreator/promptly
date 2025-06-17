use clap::Parser;
use cli::{CliArgs, copy_to_clipboard};
use core::{construct_prompt, generate_command, LLMProvider, LLMError};
use core::llm::{OllamaProvider, LlmRsProvider, LLMEngine};
use executor::shell::{ShellExecutor, UserAction};
use storage::CommandHistory;
use plugins::{PluginManager, GitPlugin, DockerPlugin};
use std::io::{self, Write};
use chrono;

#[tokio::main]
async fn main() -> Result<(), io::Error> {
    let args = CliArgs::parse();
    let executor = ShellExecutor::new();
    
    // Initialize command history with persistence
    let mut history = if let Some(custom_path) = &args.history_file {
        CommandHistory::with_persistence(custom_path.clone())
    } else {
        match CommandHistory::default_history_path() {
            Ok(path) => CommandHistory::with_persistence(path),
            Err(e) => {
                eprintln!("Warning: Could not determine history file path: {}", e);
                CommandHistory::new()
            }
        }
    };
    
    // Initialize plugin manager and register plugins
    let mut plugin_manager = PluginManager::new();
    plugin_manager.register_plugin(GitPlugin::new());
    plugin_manager.register_plugin(DockerPlugin::new());
    
    println!("Initialized {} plugins: {:?}", 
        plugin_manager.plugin_count(),
        plugin_manager.list_plugins().iter().map(|(name, _)| *name).collect::<Vec<&str>>()
    );
    
    // Handle history display if requested
    if args.history {
        display_history(&history);
        return Ok(());
    }
    
    // Initialize the appropriate LLM provider based on arguments
    let provider = match create_llm_provider(&args) {
        Ok(p) => p,
        Err(e) => return Err(io::Error::new(io::ErrorKind::Other, e.to_string()))
    };
    
    // Get user input
    let user_input = match args.input {
        Some(input) => input,
        None => {
            print!("Enter your request: ");
            io::stdout().flush()?;
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            input.trim().to_string()
        }
    };
    
    if user_input.is_empty() {
        println!("No input provided. Exiting.");
        return Ok(());
    }
    
    println!("\nProcessing: {}", user_input);
    
    // Try to process with plugins first
    if let Some(plugin_result) = plugin_manager.process(&user_input) {
        println!("\nI'll help you with that!");
        println!("Command: {}", plugin_result.command);
        println!("Explanation: {}", plugin_result.explanation);
        
        // If the plugin has already executed the command, just display the output
        if plugin_result.executed {
            if let Some(output) = plugin_result.output {
                println!("\nCommand executed by plugin:");
                println!("{}", output);
                // Add command to history
                history.add_entry(user_input, plugin_result.command);
                return Ok(());
            }
        }
        
        // Otherwise, prompt user for action
        let action = executor.prompt_for_action(&plugin_result.command, &plugin_result.explanation)?;
        
        match action {
            UserAction::Run => {
                // Execute the command
                match executor.execute_command(&plugin_result.command, args.dry_run).await {
                    Ok(output) => {
                        println!("\nCommand executed successfully:");
                        println!("{}", output);
                        
                        // Add command to history
                        history.add_entry(user_input, plugin_result.command);
                    }
                    Err(e) => {
                        eprintln!("\nError executing command: {}", e);
                    }
                }
            }
            UserAction::Copy => {
                match copy_to_clipboard(&plugin_result.command) {
                    Ok(_) => {
                        // Add to history when copied too
                        history.add_entry(user_input, plugin_result.command);
                    },
                    Err(e) => eprintln!("Error copying to clipboard: {}", e)
                }
            }
            UserAction::Abort => {
                println!("\nCommand execution aborted.");
            }
        }
        
        return Ok(());
    }
    
    // If no plugin can handle it, use the LLM
    println!("Using LLM backend: {}", provider.name());
    
    // Generate the shell command using the LLM
    let prompt = construct_prompt(&user_input);
    
    let (command, explanation) = match generate_command(&provider, &prompt).await {
        Ok((cmd, exp)) => (cmd, exp),
        Err(e) => {
            eprintln!("Error generating command: {}", e);
            return Err(io::Error::new(io::ErrorKind::Other, e.to_string()));
        }
    };
    
    // Display command and explanation
    println!("\nI'll help you with that!");
    
    // Prompt user for action
    let action = executor.prompt_for_action(&command, &explanation)?;
    
    match action {
        UserAction::Run => {
            // Execute the command directly without the helper function
            match executor.execute_command(&command, args.dry_run).await {
                Ok(output) => {
                    println!("\nCommand executed successfully:");
                    println!("{}", output);
                    
                    // Add command to history
                    history.add_entry(user_input, command);
                }
                Err(e) => {
                    eprintln!("\nError executing command: {}", e);
                }
            }
        }
        UserAction::Copy => {
            match copy_to_clipboard(&command) {
                Ok(_) => {
                    // Add to history when copied too
                    history.add_entry(user_input, command);
                },
                Err(e) => eprintln!("Error copying to clipboard: {}", e)
            }
        }
        UserAction::Abort => {
            println!("\nCommand execution aborted.");
        }
    }
    
    Ok(())
}

// Create the appropriate LLM provider based on CLI arguments
fn create_llm_provider(args: &CliArgs) -> Result<LLMProvider, LLMError> {
    match args.backend.to_lowercase().as_str() {
        "ollama" => {
            // Choose codellama or wizardcoder model
            let model = if args.online {
                "wizardcoder"
            } else {
                "codellama"
            };
            Ok(LLMProvider::Ollama(OllamaProvider::new(model)))
        },
        "llm-rs" => {
            let model_path = args.model_path.clone().unwrap_or_else(|| {
                println!("No model path specified, using default model path");
                "models/tinyllama.gguf".to_string()
            });
            Ok(LLMProvider::LlmRs(LlmRsProvider::new(&model_path)))
        },
        "openai" => {
            println!("OpenAI backend is currently disabled as an experimental feature.");
            Err(LLMError::ApiKeyError("OpenAI backend is currently disabled".into()))
            // Commented out for now
            /*
            match OpenAIProvider::new() {
                Ok(provider) => Ok(LLMProvider::OpenAI(provider)),
                Err(e) => Err(e)
            }
            */
        },
        _ => {
            println!("Unknown backend: {}. Using default (Ollama)", args.backend);
            Ok(LLMProvider::default())
        }
    }
}

// Display the command history
fn display_history(history: &CommandHistory) {
    let entries = history.get_history();
    
    if entries.is_empty() {
        println!("No command history found.");
        return;
    }
    
    println!("\nCommand History:");
    println!("---------------");
    
    for (i, entry) in entries.iter().enumerate() {
        let local_time = chrono::DateTime::<chrono::Local>::from(
            std::time::UNIX_EPOCH + std::time::Duration::from_secs(entry.timestamp)
        );
        let formatted_time = local_time.format("%Y-%m-%d %H:%M:%S");
        
        println!("{}. [{}] \"{}\" => \"{}\"", 
            i + 1,
            formatted_time,
            entry.input,
            entry.command
        );
    }
}