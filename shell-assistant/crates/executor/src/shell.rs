use std::process::Command;
use std::io::{self, Write};
use core::safety::CommandSafetyChecker;

pub enum UserAction {
    Run,
    Copy,
    Abort,
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
            return Ok(format!("Dry run: {}", command));
        }

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

    pub fn prompt_for_action(&self, command: &str, explanation: &str) -> io::Result<UserAction> {
        println!("Command: {}", command);
        println!("Explanation: {}", explanation);
        
        // Check if the command is potentially unsafe
        let (is_unsafe, reason) = self.safety_checker.check_command(command);
        if is_unsafe {
            println!("\n⚠️ WARNING: This command may be destructive! ⚠️");
            if let Some(reason) = reason {
                println!("Reason: {}", reason);
            }
            println!("Please confirm you understand the risks.");
        }
        
        println!("\n[r] Run [c] Copy [a] Abort");
        print!("Choose an action: ");
        io::stdout().flush()?;

        let mut response = String::new();
        io::stdin().read_line(&mut response)?;
        let response = response.trim().to_lowercase();

        // If the command is unsafe, require double confirmation
        if is_unsafe && (response == "r" || response == "run") {
            println!("\n⚠️ DOUBLE-CHECK: This command is potentially unsafe! ⚠️");
            print!("Type 'confirm' to proceed anyway or anything else to abort: ");
            io::stdout().flush()?;
            
            let mut confirm = String::new();
            io::stdin().read_line(&mut confirm)?;
            
            if confirm.trim().to_lowercase() != "confirm" {
                println!("Command execution aborted for safety.");
                return Ok(UserAction::Abort);
            }
        }

        match response.as_str() {
            "r" | "run" => Ok(UserAction::Run),
            "c" | "copy" => Ok(UserAction::Copy),
            "a" | "abort" | _ => Ok(UserAction::Abort),
        }
    }
}