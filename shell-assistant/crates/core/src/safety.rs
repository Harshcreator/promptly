use std::collections::HashSet;

/// Safety assessment result
#[derive(Debug, Clone, PartialEq)]
pub enum SafetyLevel {
    Safe,
    Warning,
    Dangerous,
    Blocked,
}

/// Result of safety check
#[derive(Debug, Clone)]
pub struct SafetyCheckResult {
    pub level: SafetyLevel,
    pub reason: Option<String>,
}

/// CommandSafetyChecker evaluates shell commands for potential security risks.
pub struct CommandSafetyChecker {
    high_risk_commands: HashSet<String>,
    high_risk_patterns: Vec<String>,
    safe_command_patterns: Vec<String>,
    // Enterprise features
    allowed_commands: Vec<String>,
    blocked_commands: Vec<String>,
    compliance_mode: bool,
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
            "rm",
            "rmdir",
            "del",
            "deltree",
            "format",
            "fdisk",
            "mkfs",
            "dd",
            "chmod",
            "chown",
            "sudo",
            "su",
            ">",
            "|",
            "2>",
            "mv",
            "remove-item",
            "rd",
            "erase",
        ] {
            high_risk_commands.insert(cmd.to_string());
        }

        // Add PowerShell specific dangerous cmdlets
        for cmd in [
            "remove-item",
            "rm",
            "rmdir",
            "del",
            "rd",
            "set-executionpolicy",
            "invoke-expression",
            "iex",
            "invoke-command",
            "invoke-webrequest",
            "start-process",
            "restart-computer",
            "stop-computer",
            "stop-service",
            "reset-service",
            "remove-service",
            "remove-module",
            "remove-psdrive",
            "remove-variable",
        ] {
            high_risk_commands.insert(cmd.to_string());
        }

        // Patterns that might indicate dangerous operations
        let high_risk_patterns = vec![
            "-rf".to_string(),
            "-r -f".to_string(),
            "-confirm:$false".to_string(),
            "force=true".to_string(),
            "/s /q".to_string(), // Windows silent and quiet delete
            "/y".to_string(),    // Windows suppress confirmation
        ];

        // Safe command patterns that should not trigger warnings
        let safe_command_patterns = vec![
            "get-childitem".to_string(),
            "gci".to_string(),
            "dir".to_string(),
            "ls".to_string(),
            "select-string".to_string(),
            "findstr".to_string(),
            "find-string".to_string(),
            "where-object".to_string(),
            "foreach-object".to_string(),
            "measure-object".to_string(),
        ];

        Self {
            high_risk_commands,
            high_risk_patterns,
            safe_command_patterns,
            allowed_commands: Vec::new(),
            blocked_commands: Vec::new(),
            compliance_mode: false,
        }
    }
    
    /// Create a new safety checker with enterprise configuration
    pub fn with_enterprise_config(
        allowed_commands: Vec<String>,
        blocked_commands: Vec<String>,
        compliance_mode: bool,
    ) -> Self {
        let mut checker = Self::new();
        checker.allowed_commands = allowed_commands;
        checker.blocked_commands = blocked_commands;
        checker.compliance_mode = compliance_mode;
        checker
    }
    
    /// Set allowed commands (whitelist)
    pub fn set_allowed_commands(&mut self, allowed: Vec<String>) {
        self.allowed_commands = allowed;
    }
    
    /// Set blocked commands (blacklist)
    pub fn set_blocked_commands(&mut self, blocked: Vec<String>) {
        self.blocked_commands = blocked;
    }
    
    /// Enable or disable compliance mode
    pub fn set_compliance_mode(&mut self, enabled: bool) {
        self.compliance_mode = enabled;
    }

    /// Checks if a command contains any high-risk operations.
    /// Returns a SafetyCheckResult with level and reason
    pub fn check_command_detailed(&self, command: &str) -> SafetyCheckResult {
        let command_lower = command.to_lowercase();
        
        // First check enterprise blacklist
        for pattern in &self.blocked_commands {
            if command.contains(pattern) || command_lower.contains(&pattern.to_lowercase()) {
                return SafetyCheckResult {
                    level: SafetyLevel::Blocked,
                    reason: Some(format!("Command blocked by enterprise policy: contains '{}'", pattern)),
                };
            }
        }
        
        // Check enterprise whitelist (if configured)
        if !self.allowed_commands.is_empty() {
            let mut allowed = false;
            for pattern in &self.allowed_commands {
                if command.starts_with(pattern) || command_lower.starts_with(&pattern.to_lowercase()) {
                    allowed = true;
                    break;
                }
            }
            
            if !allowed {
                return SafetyCheckResult {
                    level: SafetyLevel::Blocked,
                    reason: Some("Command not in allowed list (enterprise whitelist active)".to_string()),
                };
            }
        }

        // Check if the command starts with any safe command pattern
        for safe_pattern in &self.safe_command_patterns {
            if command_lower.starts_with(safe_pattern)
                || command_lower.split_whitespace().next() == Some(safe_pattern)
            {
                return SafetyCheckResult {
                    level: SafetyLevel::Safe,
                    reason: None,
                };
            }
        }

        let words: Vec<&str> = command_lower.split_whitespace().collect();

        // Check if the command contains any high-risk commands
        if let Some(first_word) = words.first() {
            if self.high_risk_commands.contains(&first_word.to_string()) {
                let level = if self.compliance_mode {
                    SafetyLevel::Dangerous
                } else {
                    SafetyLevel::Warning
                };
                return SafetyCheckResult {
                    level,
                    reason: Some(format!("Command '{}' can be destructive", first_word)),
                };
            }
        }

        // Check for special PowerShell operators
        for word in &words {
            let clean_word = word.trim_matches(|c: char| !c.is_alphanumeric());
            if self.high_risk_commands.contains(&clean_word.to_string()) {
                let level = if self.compliance_mode {
                    SafetyLevel::Dangerous
                } else {
                    SafetyLevel::Warning
                };
                return SafetyCheckResult {
                    level,
                    reason: Some(format!("Command '{}' can be destructive", clean_word)),
                };
            }
        }

        // Check if the command contains any high-risk patterns
        for pattern in &self.high_risk_patterns {
            if command_lower.contains(pattern) {
                return SafetyCheckResult {
                    level: SafetyLevel::Dangerous,
                    reason: Some(format!("Pattern '{}' often used in destructive operations", pattern)),
                };
            }
        }

        // Special checks for specific command combinations
        if (command_lower.contains("rm")
            || command_lower.contains("remove-item")
            || command_lower.contains("del")
            || command_lower.contains("rd"))
            && (command_lower.contains("-r")
                || command_lower.contains("-recurse")
                || command_lower.contains("/s")
                || command_lower.contains("-force")
                || command_lower.contains("/q")
                || command_lower.contains("/f"))
        {
            return SafetyCheckResult {
                level: SafetyLevel::Dangerous,
                reason: Some("Recursive or forced deletion can be dangerous".to_string()),
            };
        }

        // Check for file redirections that could overwrite files
        if command_lower.contains(" > ") && !command_lower.contains(" >> ") {
            return SafetyCheckResult {
                level: SafetyLevel::Warning,
                reason: Some("File redirection (>) will overwrite existing files".to_string()),
            };
        }

        // Not high risk
        SafetyCheckResult {
            level: SafetyLevel::Safe,
            reason: None,
        }
    }
    
    /// Legacy method for backward compatibility
    /// Returns a tuple of (is_high_risk, reason)
    pub fn check_command(&self, command: &str) -> (bool, Option<String>) {
        let result = self.check_command_detailed(command);
        let is_high_risk = matches!(result.level, SafetyLevel::Warning | SafetyLevel::Dangerous | SafetyLevel::Blocked);
        (is_high_risk, result.reason)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safe_commands() {
        let checker = CommandSafetyChecker::new();

        let result = checker.check_command_detailed("ls");
        assert_eq!(result.level, SafetyLevel::Safe);
        
        let result = checker.check_command_detailed("dir");
        assert_eq!(result.level, SafetyLevel::Safe);
        
        let result = checker.check_command_detailed("get-childitem -recurse");
        assert_eq!(result.level, SafetyLevel::Safe);
    }

    #[test]
    fn test_dangerous_commands() {
        let checker = CommandSafetyChecker::new();

        let result = checker.check_command_detailed("rm -rf /");
        assert!(matches!(result.level, SafetyLevel::Dangerous));
        
        let result = checker.check_command_detailed("Remove-Item -Force -Recurse C:\\Windows");
        assert!(matches!(result.level, SafetyLevel::Dangerous | SafetyLevel::Warning));
    }
    
    #[test]
    fn test_enterprise_blacklist() {
        let checker = CommandSafetyChecker::with_enterprise_config(
            Vec::new(),
            vec!["rm -rf /".to_string(), "format".to_string()],
            true,
        );
        
        let result = checker.check_command_detailed("rm -rf /home");
        assert_eq!(result.level, SafetyLevel::Blocked);
        
        let result = checker.check_command_detailed("format c:");
        assert_eq!(result.level, SafetyLevel::Blocked);
    }
    
    #[test]
    fn test_enterprise_whitelist() {
        let checker = CommandSafetyChecker::with_enterprise_config(
            vec!["git".to_string(), "ls".to_string(), "cd".to_string()],
            Vec::new(),
            true,
        );
        
        let result = checker.check_command_detailed("git status");
        assert_eq!(result.level, SafetyLevel::Safe);
        
        let result = checker.check_command_detailed("ls -la");
        assert_eq!(result.level, SafetyLevel::Safe);
        
        let result = checker.check_command_detailed("rm file.txt");
        assert_eq!(result.level, SafetyLevel::Blocked);
    }
}
