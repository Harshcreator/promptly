use chrono;
use clap::Parser;
use cli::{copy_to_clipboard, CliArgs};
use colored::*;
use console::Term;
use core::llm::{LLMEngine, LlmRsProvider, OllamaProvider, OpenAIProvider};
use core::{construct_prompt, generate_command, LLMError, LLMProvider};
use executor::shell::{FeedbackAction, ShellExecutor, UserAction};
use plugins::{DockerPlugin, GitPlugin, PluginManager};
use std::io::{self, Write};
use storage::persistence::FeedbackType;
use storage::CommandHistory;

#[tokio::main]
async fn main() -> Result<(), io::Error> {
    let args = CliArgs::parse();
    let executor = ShellExecutor::new();
    let _term = Term::stdout();

    // Initialize command history with persistence
    let mut history = if let Some(custom_path) = &args.history_file {
        CommandHistory::with_persistence(custom_path.clone())
    } else {
        match CommandHistory::default_history_path() {
            Ok(path) => CommandHistory::with_persistence(path),
            Err(e) => {
                eprintln!(
                    "{} {}",
                    "⚠️ Warning:".yellow(),
                    format!("Could not determine history file path: {}", e).yellow()
                );
                CommandHistory::new()
            }
        }
    };

    // Initialize plugin manager and register plugins
    let mut plugin_manager = PluginManager::new();
    plugin_manager.register_plugin(GitPlugin::new());
    plugin_manager.register_plugin(DockerPlugin::new());

    // Print debug info if requested
    if args.debug {
        println!(
            "{} {}",
            "🔍 Debug:".bright_blue(),
            format!("Command line arguments: {:?}", args).bright_blue()
        );

        if let Some(path) = history.get_file_path() {
            println!(
                "{} {}",
                "🔍 Debug:".bright_blue(),
                format!("History path: {}", path).bright_blue()
            );
        }
    }

    let plugins_list =
        plugin_manager.list_plugins().iter().map(|(name, _)| *name).collect::<Vec<&str>>();
    println!(
        "{} {} {}",
        "✅ Initialized".green(),
        plugin_manager.plugin_count().to_string().green(),
        format!("plugins: {:?}", plugins_list).green()
    );

    // Handle list plugins command
    if args.list_plugins {
        println!("\n{}", "🔌 Available Plugins:".bright_cyan());
        println!("{}", "-------------------".bright_cyan());

        for (name, description) in plugin_manager.list_plugins() {
            println!("{}  {}", name.bright_green(), description);
        }

        return Ok(());
    }

    // Handle history display if requested
    if args.history {
        display_history(&history);
        return Ok(());
    }

    // Initialize the appropriate LLM provider based on arguments
    let provider = match create_llm_provider(&args) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("{} {}", "❌ Error:".bright_red(), e.to_string().bright_red());
            return Err(io::Error::new(io::ErrorKind::Other, e.to_string()));
        }
    };

    // Get user input
    let user_input = match args.input {
        Some(input) => input,
        None => {
            print!("{} ", "Enter your request:".bright_cyan());
            io::stdout().flush()?;
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            input.trim().to_string()
        }
    };

    if user_input.is_empty() {
        println!("{}", "No input provided. Exiting.".red());
        return Ok(());
    }

    println!("\n{} {}", "💬 Processing:".bright_blue(), user_input);

    // Try to process with plugins
    let plugin_result = if let Some(plugin_name) = &args.plugin {
        // If a specific plugin is requested, use only that plugin
        let plugin_name = plugin_name.to_lowercase();

        // Find the requested plugin
        if let Some(plugin) = plugin_manager.get_plugin(&plugin_name) {
            // Process with the specified plugin
            if plugin.can_handle(&user_input) {
                if let Some(result) = plugin.handle(&user_input) {
                    println!("{} {}", "🔌 Using plugin:".bright_green(), plugin_name);

                    if args.debug {
                        println!(
                            "{} {}",
                            "🔍 Debug - Plugin:".bright_blue(),
                            format!("Plugin '{}' matched input", plugin_name).bright_blue()
                        );
                    }

                    Some(result)
                } else {
                    println!(
                        "{} {} {}",
                        "⚠️ Warning:".yellow(),
                        format!("Plugin '{}' couldn't process the request", plugin_name).yellow(),
                        "Falling back to LLM.".yellow()
                    );
                    None
                }
            } else {
                println!(
                    "{} {} {}",
                    "⚠️ Warning:".yellow(),
                    format!("Plugin '{}' can't handle this request", plugin_name).yellow(),
                    "Falling back to LLM.".yellow()
                );
                None
            }
        } else {
            println!(
                "{} {}",
                "⚠️ Warning:".yellow(),
                format!("Plugin '{}' not found", plugin_name).yellow()
            );
            None
        }
    } else {
        // Try all plugins
        let mut result = None;

        for (name, _) in &plugin_manager.list_plugins() {
            if let Some(plugin) = plugin_manager.get_plugin(name) {
                if plugin.can_handle(&user_input) {
                    if let Some(cmd_result) = plugin.handle(&user_input) {
                        println!("{} {}", "🔌 Using plugin:".bright_green(), name);

                        if args.debug {
                            println!(
                                "{} {}",
                                "🔍 Debug - Plugin:".bright_blue(),
                                format!("Plugin '{}' automatically selected", name).bright_blue()
                            );
                        }

                        result = Some(cmd_result);
                        break;
                    }
                }
            }
        }

        result
    };

    // Process with plugin if we have a result
    if let Some(plugin_result) = plugin_result {
        println!("\n{}", "🤖 I'll help you with that!".bright_green());
        println!("{}: {}", "Command".bright_green(), plugin_result.command);
        println!("{}: {}", "Explanation".bright_green(), plugin_result.explanation);

        // If the plugin has already executed the command, just display the output
        if plugin_result.executed {
            if let Some(output) = plugin_result.output {
                println!("\n{}", "🚀 Command executed by plugin:".bright_green());
                println!("{}", output);
                // Add command to history
                history.add_entry(
                    user_input,
                    plugin_result.command,
                    Some(plugin_result.explanation.clone()),
                );
                return Ok(());
            }
        }

        // Otherwise, prompt user for action
        let action = executor.prompt_for_action(
            &plugin_result.command,
            &plugin_result.explanation,
            args.force,
        )?;

        match action {
            UserAction::Run => {
                // Execute the command
                match executor.execute_command(&plugin_result.command, args.dry_run).await {
                    Ok(output) => {
                        println!("\n{}", "✅ Command executed successfully:".bright_green());
                        println!("{}", output);

                        // Add command to history
                        history.add_entry(
                            user_input.clone(),
                            plugin_result.command.clone(),
                            Some(plugin_result.explanation.clone()),
                        );

                        // Prompt for feedback if not disabled
                        if !args.no_feedback {
                            handle_feedback(&mut history, &executor, &plugin_result.command)?;
                        }
                    }
                    Err(e) => {
                        eprintln!(
                            "\n{} {}",
                            "❌ Error executing command:".bright_red(),
                            e.to_string().bright_red()
                        );
                    }
                }
            }
            UserAction::Copy => {
                match copy_to_clipboard(&plugin_result.command) {
                    Ok(_) => {
                        println!("\n{}", "📋 Command copied to clipboard!".bright_green());

                        // Add to history when copied too
                        history.add_entry(
                            user_input.clone(),
                            plugin_result.command.clone(),
                            Some(plugin_result.explanation.clone()),
                        );

                        // Prompt for feedback if not disabled
                        if !args.no_feedback {
                            handle_feedback(&mut history, &executor, &plugin_result.command)?;
                        }
                    }
                    Err(e) => eprintln!(
                        "{} {}",
                        "❌ Error copying to clipboard:".bright_red(),
                        e.to_string().bright_red()
                    ),
                }
            }
            UserAction::Abort => {
                println!("\n{}", "🛑 Command execution aborted.".yellow());
            }
        }

        return Ok(());
    }

    // If no plugin can handle it, use the LLM
    println!("{} {}", "🧠 Using LLM backend:".bright_blue(), provider.name());

    // Skip LLM if in offline mode and the LLM is online-only
    if args.offline && provider.is_online() {
        println!("{}", "❌ Cannot use online LLM in offline mode. Exiting.".bright_red());
        return Ok(());
    }

    // Generate the shell command using the LLM
    let prompt = construct_prompt(&user_input);

    if args.debug {
        println!("{} {}", "🔍 Debug - Prompt:".bright_blue(), prompt.bright_blue());
    }

    let (command, explanation) = match generate_command(&provider, &prompt).await {
        Ok((cmd, exp)) => (cmd, exp),
        Err(e) => {
            eprintln!(
                "{} {}",
                "❌ Error generating command:".bright_red(),
                e.to_string().bright_red()
            );
            return Err(io::Error::new(io::ErrorKind::Other, e.to_string()));
        }
    };

    // Display command and explanation
    println!("\n{}", "🤖 I'll help you with that!".bright_green());

    // Prompt user for action
    let action = executor.prompt_for_action(&command, &explanation, args.force)?;

    match action {
        UserAction::Run => {
            // Execute the command directly without the helper function
            match executor.execute_command(&command, args.dry_run).await {
                Ok(output) => {
                    println!("\n{}", "✅ Command executed successfully:".bright_green());
                    println!("{}", output);

                    // Add command to history
                    history.add_entry(
                        user_input.clone(),
                        command.clone(),
                        Some(explanation.clone()),
                    );

                    // Prompt for feedback if not disabled
                    if !args.no_feedback {
                        handle_feedback(&mut history, &executor, &command)?;
                    }
                }
                Err(e) => {
                    eprintln!(
                        "\n{} {}",
                        "❌ Error executing command:".bright_red(),
                        e.to_string().bright_red()
                    );
                }
            }
        }
        UserAction::Copy => {
            match copy_to_clipboard(&command) {
                Ok(_) => {
                    println!("\n{}", "📋 Command copied to clipboard!".bright_green());

                    // Add to history when copied too
                    history.add_entry(
                        user_input.clone(),
                        command.clone(),
                        Some(explanation.clone()),
                    );

                    // Prompt for feedback if not disabled
                    if !args.no_feedback {
                        handle_feedback(&mut history, &executor, &command)?;
                    }
                }
                Err(e) => eprintln!(
                    "{} {}",
                    "❌ Error copying to clipboard:".bright_red(),
                    e.to_string().bright_red()
                ),
            }
        }
        UserAction::Abort => {
            println!("\n{}", "🛑 Command execution aborted.".yellow());
        }
    }

    Ok(())
}

// Helper function to handle feedback
fn handle_feedback(
    history: &mut CommandHistory,
    executor: &ShellExecutor,
    command: &str,
) -> io::Result<()> {
    let (feedback, edited_cmd) = executor.prompt_for_feedback(command)?;

    // Process feedback
    match feedback {
        FeedbackAction::Helpful => {
            history.update_last_entry_feedback(FeedbackType::Helpful, None);
            println!("{}", "👍 Thanks for your feedback!".bright_green());
        }
        FeedbackAction::NotHelpful => {
            history.update_last_entry_feedback(FeedbackType::NotHelpful, None);
            println!(
                "{}",
                "👎 Sorry to hear that. We'll try to do better next time!".bright_yellow()
            );
        }
        FeedbackAction::Edit => {
            if let Some(cmd) = edited_cmd {
                history.update_last_entry_feedback(FeedbackType::Edited, Some(cmd));
                println!(
                    "{}",
                    "✏️ Thanks for your correction! We'll learn from this.".bright_green()
                );
            }
        }
        FeedbackAction::Skip => {
            println!("{}", "⏭️ Feedback skipped.".bright_blue());
        }
    }

    Ok(())
}

// Create the appropriate LLM provider based on CLI arguments
fn create_llm_provider(args: &CliArgs) -> Result<LLMProvider, LLMError> {
    // If offline mode is enabled, ensure we don't use online providers
    if args.offline {
        match args.backend.to_lowercase().as_str() {
            "openai" => {
                println!(
                    "{}",
                    "⚠️ OpenAI backend requires internet. Using local LLM instead.".yellow()
                );
                return Ok(LLMProvider::LlmRs(LlmRsProvider::new(
                    &args.model_path.clone().unwrap_or_else(|| "models/tinyllama.gguf".to_string()),
                )));
            }
            "ollama" if args.online => {
                println!(
                    "{}",
                    "⚠️ Online Ollama mode requires internet. Using local model instead.".yellow()
                );
                return Ok(LLMProvider::Ollama(OllamaProvider::new("codellama")));
            }
            _ => {}
        }
    }

    match args.backend.to_lowercase().as_str() {
        "ollama" => {
            // Choose codellama or wizardcoder model
            let model = if args.online { "wizardcoder" } else { "codellama" };
            Ok(LLMProvider::Ollama(OllamaProvider::new(model)))
        }
        "llm-rs" => {
            let model_path = args.model_path.clone().unwrap_or_else(|| {
                println!("{}", "ℹ️ No model path specified, using default model path".blue());
                "models/tinyllama.gguf".to_string()
            });
            Ok(LLMProvider::LlmRs(LlmRsProvider::new(&model_path)))
        }
        "openai" => {
            if args.offline {
                return Err(LLMError::ApiKeyError(
                    "OpenAI backend cannot be used in offline mode".into(),
                ));
            }

            let model = args.openai_model.as_deref().unwrap_or("gpt-3.5-turbo");
            match OpenAIProvider::new_with_model(model) {
                Ok(provider) => {
                    println!(
                        "{} {}",
                        "✅ OpenAI backend initialized successfully with model:".green(),
                        model.green()
                    );
                    Ok(LLMProvider::OpenAI(provider))
                }
                Err(LLMError::ApiKeyError(msg)) => {
                    eprintln!("{} {}", "❌ OpenAI Configuration Error:".red(), msg.red());
                    eprintln!("{}", "💡 To use OpenAI backend:".yellow());
                    eprintln!(
                        "   {}",
                        "1. Set your API key: export OPENAI_API_KEY=sk-your-key-here".yellow()
                    );
                    eprintln!(
                        "   {}",
                        "2. Or create a .env file with: OPENAI_API_KEY=sk-your-key-here".yellow()
                    );
                    eprintln!(
                        "   {}",
                        "3. Get your API key from: https://platform.openai.com/api-keys".yellow()
                    );
                    Err(LLMError::ApiKeyError(msg))
                }
                Err(e) => Err(e),
            }
        }
        _ => {
            println!(
                "{} {}",
                "⚠️ Unknown backend:".yellow(),
                format!("{}. Using default (Ollama)", args.backend).yellow()
            );
            Ok(LLMProvider::default())
        }
    }
}

// Display the command history
fn display_history(history: &CommandHistory) {
    let entries = history.get_history();

    if entries.is_empty() {
        println!("{}", "No command history found.".yellow());
        return;
    }

    println!("\n{}", "📜 Command History:".bright_cyan());
    println!("{}", "---------------".bright_cyan());

    for (i, entry) in entries.iter().enumerate() {
        let local_time = chrono::DateTime::<chrono::Local>::from(
            std::time::UNIX_EPOCH + std::time::Duration::from_secs(entry.timestamp),
        );
        let formatted_time = local_time.format("%Y-%m-%d %H:%M:%S");

        // Get feedback indicator
        let feedback_indicator = match entry.feedback {
            FeedbackType::Helpful => "👍",
            FeedbackType::NotHelpful => "👎",
            FeedbackType::Edited => "✏️",
            FeedbackType::None => "  ",
        };

        println!(
            "{}. [{}] {} \"{}\" => \"{}\"",
            (i + 1).to_string().bright_blue(),
            formatted_time.to_string().cyan(),
            feedback_indicator,
            entry.input.bright_green(),
            entry.command.yellow()
        );

        // Show explanation if available
        if let Some(explanation) = &entry.explanation {
            println!("   {}: {}", "Explanation".bright_cyan(), explanation);
        }

        // Show original command if edited
        if let Some(original) = &entry.original_command {
            println!("   {}: {}", "Original command".bright_red(), original);
        }

        println!("");
    }
}
