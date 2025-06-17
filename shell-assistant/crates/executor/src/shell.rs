use std::process::Command;
use std::io::{self, Write};
use core::safety::CommandSafetyChecker;
use storage::persistence::FeedbackType;
use colored::*;
use dialoguer::{theme::ColorfulTheme, Select, Input, Confirm};
use console::{Term, style};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UserAction {
    Run,
    Copy,
    Abort,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FeedbackAction {
    Helpful,
    NotHelpful,
    Edit,
    Skip,
}

pub struct ShellExecutor {
    safety_checker: CommandSafetyChecker,
}

impl ShellExecutor {
    pub fn new() -> Self {
        ShellExecutor {
            safety_checker: CommandSafetyChecker::new(),
        }
    }

    pub async fn execute_command(&self, command: &str, dry_run: bool) -> io::Result<String> {
        if dry_run {
            return Ok(format!("{} {}", "🔍 Dry run:".bright_blue(), command));
        }

        println!("{} {}", "🚀 Executing:".bright_green(), command);
        
        // Use cmd.exe on Windows
        #[cfg(target_os = "windows")]
        let output = Command::new("powershell.exe")
            .args(["-Command", command])
            .output()?;

        // Use sh on Unix-like systems
        #[cfg(not(target_os = "windows"))]
        let output = Command::new("sh")
            .arg("-c")
            .arg(command)
            .output()?;

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();

        if output.status.success() {
            Ok(stdout)
        } else {
            Err(io::Error::new(io::ErrorKind::Other, stderr))
        }
    }

    pub fn prompt_for_action(&self, command: &str, explanation: &str, force: bool) -> io::Result<UserAction> {
        println!("{}: {}", "Command".bright_green(), command);
        println!("{}: {}", "Explanation".bright_green(), explanation);
        
        // Check if the command is potentially unsafe
        let (is_unsafe, reason) = self.safety_checker.check_command(command);
        if is_unsafe {
            println!("\n{} {}", " ⚠️ WARNING:".on_yellow().black(), "This command may be destructive!".yellow());
            if let Some(reason) = reason {
                println!("{}: {}", "Reason".yellow(), reason);
            }
            println!("{}", "Please confirm you understand the risks.".yellow());
        }
        
        if force && !is_unsafe {
            // If force is enabled and the command is safe, execute without prompting
            println!("{}", "🚀 Force mode enabled - executing without confirmation".bright_blue());
            return Ok(UserAction::Run);
        }
        
        let options = vec!["▶️  Run", "📋 Copy", "❌ Abort"];
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Choose an action")
            .default(0)
            .items(&options)
            .interact()
            .unwrap_or(2); // Default to Abort if interaction fails
            
        // If the command is unsafe, require double confirmation
        if is_unsafe && selection == 0 && !force {
            println!("\n{} {}", " ⚠️ DOUBLE-CHECK:".on_red().black(), "This command is potentially unsafe!".red());
            
            let confirm = Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt("Proceed anyway?")
                .default(false)
                .interact()
                .unwrap_or(false);
                
            if !confirm {
                println!("{}", "Command execution aborted for safety.".bright_red());
                return Ok(UserAction::Abort);
            }
        }

        match selection {
            0 => Ok(UserAction::Run),
            1 => Ok(UserAction::Copy),
            _ => Ok(UserAction::Abort),
        }
    }

    pub fn prompt_for_feedback(&self, _command: &str) -> io::Result<(FeedbackAction, Option<String>)> {
        println!("\n{}", "Was this command helpful?".bright_cyan());
        
        let options = vec!["👍 Yes", "👎 No", "✏️  Edit", "⏭️  Skip"];
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Your feedback")
            .default(0)
            .items(&options)
            .interact()
            .unwrap_or(3); // Default to Skip if interaction fails
        
        match selection {
            0 => Ok((FeedbackAction::Helpful, None)),
            1 => Ok((FeedbackAction::NotHelpful, None)),
            2 => {
                println!("\n{}", "Please enter your corrected command:".bright_cyan());
                
                let edited = Input::<String>::with_theme(&ColorfulTheme::default())
                    .with_prompt(">")
                    .allow_empty(true)
                    .interact()
                    .unwrap_or_default();
                
                if edited.is_empty() {
                    println!("{}", "No changes made.".yellow());
                    Ok((FeedbackAction::Skip, None))
                } else {
                    println!("{}: {}", "Command updated".bright_green(), edited);
                    Ok((FeedbackAction::Edit, Some(edited)))
                }
            },
            _ => Ok((FeedbackAction::Skip, None)),
        }
    }
}

impl FeedbackAction {
    /// Convert FeedbackAction to storage's FeedbackType
    pub fn to_feedback_type(&self) -> FeedbackType {
        match self {
            FeedbackAction::Helpful => FeedbackType::Helpful,
            FeedbackAction::NotHelpful => FeedbackType::NotHelpful,
            FeedbackAction::Edit => FeedbackType::Edited,
            FeedbackAction::Skip => FeedbackType::None,
        }
    }
}
