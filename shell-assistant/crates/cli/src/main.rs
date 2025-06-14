use clap::Parser;
use cli::{CliArgs, copy_to_clipboard};
use core::{construct_prompt, generate_command, LLMProvider, LLMEngine, LLMError};
use core::llm::{OllamaProvider, LlmRsProvider};
use executor::shell::{ShellExecutor, UserAction};
use std::io::{self, Write};

#[tokio::main]
async fn main() -> Result<(), io::Error> {
    let args = CliArgs::parse();
    let executor = ShellExecutor::new();
    
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
                }
                Err(e) => {
                    eprintln!("\nError executing command: {}", e);
                }
            }
        }
        UserAction::Copy => {
            match copy_to_clipboard(&command) {
                Ok(_) => {},
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