use clap::Parser;
use cli::{CliArgs, copy_to_clipboard};
use core::{construct_prompt, mock_llm_call, parse_response};
use executor::shell::{ShellExecutor, UserAction};
use std::io::{self, Write};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = CliArgs::parse();
    let executor = ShellExecutor::new();
    
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
    
    // Mock LLM call (always returns the same response for testing)
    let prompt = construct_prompt(&user_input);
    let llm_response = mock_llm_call(&prompt).await?;
    
    // Parse LLM response
    let (command, explanation) = parse_response(&llm_response)?;
    
    // Display command and explanation
    println!("\nI'll help you with that!");
    
    // Prompt user for action
    let action = executor.prompt_for_action(&command, &explanation)?;
    
    match action {
        UserAction::Run => {
            // Execute the command
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
            copy_to_clipboard(&command)?;
        }
        UserAction::Abort => {
            println!("\nCommand execution aborted.");
        }
    }
    
    Ok(())
}