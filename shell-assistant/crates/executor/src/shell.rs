use std::process::Command;
use std::io::{self, Write};

pub enum UserAction {
    Run,
    Copy,
    Abort,
}

pub struct ShellExecutor;

impl ShellExecutor {
    pub fn new() -> Self {
        ShellExecutor
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
        println!("\n[r] Run [c] Copy [a] Abort");
        print!("Choose an action: ");
        io::stdout().flush()?;

        let mut response = String::new();
        io::stdin().read_line(&mut response)?;

        match response.trim().to_lowercase().as_str() {
            "r" | "run" => Ok(UserAction::Run),
            "c" | "copy" => Ok(UserAction::Copy),
            "a" | "abort" | _ => Ok(UserAction::Abort),
        }
    }
}