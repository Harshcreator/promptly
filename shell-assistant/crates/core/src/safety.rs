use std::collections::HashSet;

/// CommandSafetyChecker evaluates shell commands for potential security risks.
pub struct CommandSafetyChecker {
    high_risk_commands: HashSet<String>,
    high_risk_patterns: Vec<String>,
}

impl Default for CommandSafetyChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl CommandSafetyChecker {
    /// Creates a new CommandSafetyChecker with default high-risk commands and patterns.
    pub fn new() -> Self {
        let mut high_risk_commands = HashSet::new();
        
        // Add potentially dangerous commands
        for cmd in [
            "rm", "rmdir", "del", "deltree", "format",
            "fdisk", "mkfs", "dd", "chmod", "chown",
            "sudo", "su", ">", "|", "2>",
            "mv", "remove-item", "rd", "erase",
        ] {
            high_risk_commands.insert(cmd.to_string());
        }
        
        // Add PowerShell specific dangerous cmdlets
        for cmd in [
            "remove-item", "rm", "rmdir", "del", "rd",
            "set-executionpolicy", "invoke-expression", "iex",
            "invoke-command", "invoke-webrequest", "start-process",
            "restart-computer", "stop-computer", "stop-service", 
            "reset-service", "remove-service", "remove-module",
            "remove-psdrive", "remove-variable"
        ] {
            high_risk_commands.insert(cmd.to_string());
        }
        
        // Patterns that might indicate dangerous operations
        let high_risk_patterns = vec![
            "-rf".to_string(),
            "-r -f".to_string(),
            "-force".to_string(),
            "-confirm:$false".to_string(),
            "-recursive".to_string(),
            "force=true".to_string(),
            "recurse".to_string(),
            "/s /q".to_string(),  // Windows silent and quiet delete
            "/y".to_string(),     // Windows suppress confirmation
            "> /dev/null".to_string(),
            "2>&1".to_string(),
        ];
        
        Self {
            high_risk_commands,
            high_risk_patterns,
        }
    }
    
    /// Checks if a command contains any high-risk operations.
    /// Returns a tuple of (is_high_risk, reason) where reason explains 
    /// why the command is considered high risk if applicable.
    pub fn check_command(&self, command: &str) -> (bool, Option<String>) {
        let command_lower = command.to_lowercase();
        let words: Vec<&str> = command_lower.split_whitespace().collect();
        
        // Check if the command contains any high-risk commands
        if let Some(first_word) = words.first() {
            if self.high_risk_commands.contains(&first_word.to_string()) {
                return (true, Some(format!("Command '{}' can be destructive", first_word)));
            }
        }
        
        // Check for special PowerShell operators
        for word in &words {
            // Remove any punctuation to check the core command
            let clean_word = word.trim_matches(|c: char| !c.is_alphanumeric());
            if self.high_risk_commands.contains(&clean_word.to_string()) {
                return (true, Some(format!("Command '{}' can be destructive", clean_word)));
            }
        }
        
        // Check if the command contains any high-risk patterns
        for pattern in &self.high_risk_patterns {
            if command_lower.contains(pattern) {
                return (true, Some(format!("Pattern '{}' often used in destructive operations", pattern)));
            }
        }
        
        // Special checks for specific command combinations
        if (command_lower.contains("rm") || command_lower.contains("remove-item") || 
            command_lower.contains("del") || command_lower.contains("rd")) && 
            (command_lower.contains("-r") || command_lower.contains("-recurse") || 
             command_lower.contains("/s") || command_lower.contains("-force") || 
             command_lower.contains("/q") || command_lower.contains("/f")) {
            return (true, Some("Recursive or forced deletion can be dangerous".to_string()));
        }
        
        // Check for file redirections that could overwrite files
        if command_lower.contains(" > ") && !command_lower.contains(" >> ") {
            return (true, Some("File redirection (>) will overwrite existing files".to_string()));
        }
        
        // Not high risk
        (false, None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safe_commands() {
        let checker = CommandSafetyChecker::new();
        
        // These should be considered safe
        assert!(!checker.check_command("ls").0);
        assert!(!checker.check_command("dir").0);
        assert!(!checker.check_command("echo hello").0);
        assert!(!checker.check_command("get-childitem").0);
        assert!(!checker.check_command("ping 8.8.8.8").0);
    }

    #[test]
    fn test_dangerous_commands() {
        let checker = CommandSafetyChecker::new();
        
        // These should be flagged as dangerous
        assert!(checker.check_command("rm -rf /").0);
        assert!(checker.check_command("sudo rm -rf *").0);
        assert!(checker.check_command("Remove-Item -Force -Recurse C:\\Windows").0);
        assert!(checker.check_command("del /s /q C:\\Important").0);
        assert!(checker.check_command("chmod -R 777 /").0);
        assert!(checker.check_command("fdisk /dev/sda").0);
    }
}
