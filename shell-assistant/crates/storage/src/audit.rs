use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::{BufWriter, Write};
use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AuditError {
    #[error("Failed to write audit log: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("Failed to serialize audit entry: {0}")]
    SerializationError(#[from] serde_json::Error),
}

/// Safety level of a command
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum SafetyLevel {
    Safe,
    Warning,
    Dangerous,
    Blocked,
}

/// A single audit log entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEntry {
    /// Timestamp of the event
    pub timestamp: DateTime<Utc>,
    
    /// Username executing the command
    pub user: String,
    
    /// Organization name (from config)
    pub organization: Option<String>,
    
    /// Department name (from config)
    pub department: Option<String>,
    
    /// User's natural language input
    pub input: String,
    
    /// Generated command
    pub generated_command: String,
    
    /// Whether the command was actually executed
    pub executed: bool,
    
    /// Exit code if executed (None if not executed or still running)
    pub exit_code: Option<i32>,
    
    /// Safety level assessment
    pub safety_level: SafetyLevel,
    
    /// Additional notes or warnings
    pub notes: Option<String>,
    
    /// LLM backend used
    pub llm_backend: String,
    
    /// Session ID for tracking related commands
    pub session_id: Option<String>,
}

/// Audit logger for tracking command execution
pub struct AuditLogger {
    log_path: PathBuf,
    organization: Option<String>,
    department: Option<String>,
}

impl AuditLogger {
    /// Create a new audit logger
    pub fn new(log_path: PathBuf, organization: Option<String>, department: Option<String>) -> Self {
        // Create parent directory if it doesn't exist
        if let Some(parent) = log_path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        
        Self {
            log_path,
            organization,
            department,
        }
    }
    
    /// Log a command execution event
    pub fn log_command(
        &self,
        input: String,
        generated_command: String,
        executed: bool,
        exit_code: Option<i32>,
        safety_level: SafetyLevel,
        llm_backend: String,
        notes: Option<String>,
        session_id: Option<String>,
    ) -> Result<(), AuditError> {
        let entry = AuditEntry {
            timestamp: Utc::now(),
            user: Self::get_current_user(),
            organization: self.organization.clone(),
            department: self.department.clone(),
            input,
            generated_command,
            executed,
            exit_code,
            safety_level,
            notes,
            llm_backend,
            session_id,
        };
        
        self.write_entry(&entry)
    }
    
    /// Write an audit entry to the log file
    fn write_entry(&self, entry: &AuditEntry) -> Result<(), AuditError> {
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.log_path)?;
        
        let mut writer = BufWriter::new(file);
        let json = serde_json::to_string(entry)?;
        writeln!(writer, "{}", json)?;
        writer.flush()?;
        
        Ok(())
    }
    
    /// Get the current system user
    fn get_current_user() -> String {
        std::env::var("USER")
            .or_else(|_| std::env::var("USERNAME"))
            .unwrap_or_else(|_| "unknown".to_string())
    }
    
    /// Read all audit entries from the log file
    pub fn read_entries(&self) -> Result<Vec<AuditEntry>, AuditError> {
        if !self.log_path.exists() {
            return Ok(Vec::new());
        }
        
        let contents = std::fs::read_to_string(&self.log_path)?;
        let mut entries = Vec::new();
        
        for line in contents.lines() {
            if line.trim().is_empty() {
                continue;
            }
            
            match serde_json::from_str::<AuditEntry>(line) {
                Ok(entry) => entries.push(entry),
                Err(e) => {
                    eprintln!("Failed to parse audit entry: {}", e);
                    continue;
                }
            }
        }
        
        Ok(entries)
    }
    
    /// Get entries for a specific user
    pub fn get_user_entries(&self, username: &str) -> Result<Vec<AuditEntry>, AuditError> {
        let entries = self.read_entries()?;
        Ok(entries.into_iter().filter(|e| e.user == username).collect())
    }
    
    /// Get entries with specific safety level
    pub fn get_entries_by_safety(&self, safety_level: SafetyLevel) -> Result<Vec<AuditEntry>, AuditError> {
        let entries = self.read_entries()?;
        Ok(entries.into_iter().filter(|e| e.safety_level == safety_level).collect())
    }
    
    /// Get entries within a time range
    pub fn get_entries_in_range(
        &self,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Result<Vec<AuditEntry>, AuditError> {
        let entries = self.read_entries()?;
        Ok(entries
            .into_iter()
            .filter(|e| e.timestamp >= start && e.timestamp <= end)
            .collect())
    }
    
    /// Get statistics from audit log
    pub fn get_statistics(&self) -> Result<AuditStats, AuditError> {
        let entries = self.read_entries()?;
        
        let total_commands = entries.len();
        let executed_commands = entries.iter().filter(|e| e.executed).count();
        let failed_commands = entries.iter()
            .filter(|e| e.exit_code.map(|c| c != 0).unwrap_or(false))
            .count();
        let dangerous_commands = entries.iter()
            .filter(|e| matches!(e.safety_level, SafetyLevel::Dangerous | SafetyLevel::Blocked))
            .count();
        
        Ok(AuditStats {
            total_commands,
            executed_commands,
            failed_commands,
            dangerous_commands,
        })
    }
}

/// Statistics from audit log
#[derive(Debug, Serialize, Deserialize)]
pub struct AuditStats {
    pub total_commands: usize,
    pub executed_commands: usize,
    pub failed_commands: usize,
    pub dangerous_commands: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    
    #[test]
    fn test_audit_logger() {
        let temp_dir = env::temp_dir();
        let log_path = temp_dir.join("test_audit.log");
        
        // Clean up from previous test
        let _ = std::fs::remove_file(&log_path);
        
        let logger = AuditLogger::new(
            log_path.clone(),
            Some("Test Corp".to_string()),
            Some("Engineering".to_string()),
        );
        
        // Log a command
        logger.log_command(
            "list files".to_string(),
            "ls -la".to_string(),
            true,
            Some(0),
            SafetyLevel::Safe,
            "ollama".to_string(),
            None,
            Some("session-123".to_string()),
        ).unwrap();
        
        // Read entries
        let entries = logger.read_entries().unwrap();
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].input, "list files");
        assert_eq!(entries[0].generated_command, "ls -la");
        assert_eq!(entries[0].organization, Some("Test Corp".to_string()));
        
        // Clean up
        let _ = std::fs::remove_file(&log_path);
    }
    
    #[test]
    fn test_audit_statistics() {
        let temp_dir = env::temp_dir();
        let log_path = temp_dir.join("test_audit_stats.log");
        
        // Clean up from previous test
        let _ = std::fs::remove_file(&log_path);
        
        let logger = AuditLogger::new(log_path.clone(), None, None);
        
        // Log multiple commands
        logger.log_command(
            "safe command".to_string(),
            "ls".to_string(),
            true,
            Some(0),
            SafetyLevel::Safe,
            "ollama".to_string(),
            None,
            None,
        ).unwrap();
        
        logger.log_command(
            "dangerous command".to_string(),
            "rm -rf /".to_string(),
            false,
            None,
            SafetyLevel::Blocked,
            "ollama".to_string(),
            Some("Command blocked by safety checker".to_string()),
            None,
        ).unwrap();
        
        let stats = logger.get_statistics().unwrap();
        assert_eq!(stats.total_commands, 2);
        assert_eq!(stats.executed_commands, 1);
        assert_eq!(stats.dangerous_commands, 1);
        
        // Clean up
        let _ = std::fs::remove_file(&log_path);
    }
}
