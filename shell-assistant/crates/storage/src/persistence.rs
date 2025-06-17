use std::fs::File;
use std::io::{self, Write, Read};
use std::path::Path;
use serde::{Serialize, Deserialize};

/// Feedback type for command execution
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
pub enum FeedbackType {
    /// User found the command helpful
    Helpful,
    /// User did not find the command helpful
    NotHelpful,
    /// User edited the command
    Edited,
    /// No feedback provided
    None,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CommandEntry {
    /// Natural language input
    pub input: String,
    /// Generated or edited command
    pub command: String,
    /// Explanation for the command
    pub explanation: Option<String>,
    /// Timestamp when the entry was created
    pub timestamp: u64,
    /// User feedback on the command
    pub feedback: FeedbackType,
    /// Original command if edited
    pub original_command: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CommandHistory {
    pub entries: Vec<CommandEntry>,
}

impl CommandHistory {
    pub fn new() -> Self {
        CommandHistory {
            entries: Vec::new(),
        }
    }

    /// Get the default history file path, platform-independent
    pub fn default_history_path() -> io::Result<String> {
        let home = if cfg!(windows) {
            std::env::var("USERPROFILE").map_err(|_| {
                io::Error::new(io::ErrorKind::NotFound, "Could not find USERPROFILE environment variable")
            })?
        } else {
            std::env::var("HOME").map_err(|_| {
                io::Error::new(io::ErrorKind::NotFound, "Could not find HOME environment variable")
            })?
        };
        
        let app_dir = Path::new(&home).join(".shell-assistant");
        let history_file = app_dir.join("history.json");
        
        Ok(history_file.to_string_lossy().into_owned())
    }

    pub fn add_entry(&mut self, input: String, command: String, explanation: Option<String>) {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
            
        self.entries.push(CommandEntry {
            input,
            command,
            explanation,
            timestamp,
            feedback: FeedbackType::None,
            original_command: None,
        });
    }
    
    /// Add a command entry with feedback
    pub fn add_entry_with_feedback(&mut self, input: String, command: String, explanation: Option<String>, 
                                   feedback: FeedbackType, original_command: Option<String>) {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
            
        self.entries.push(CommandEntry {
            input,
            command,
            explanation,
            timestamp,
            feedback,
            original_command,
        });
    }
    
    /// Update feedback for the most recent entry
    pub fn update_last_entry_feedback(&mut self, feedback: FeedbackType, edited_command: Option<String>) {
        if let Some(last_entry) = self.entries.last_mut() {
            if feedback == FeedbackType::Edited {
                // If the command was edited, store the original command
                last_entry.original_command = Some(last_entry.command.clone());
                // Update the command with the edited version
                if let Some(cmd) = edited_command {
                    last_entry.command = cmd;
                }
            }
            last_entry.feedback = feedback;
        }
    }

    pub fn save_to_file(&self, file_path: &str) -> io::Result<()> {
        // Create parent directories if they don't exist
        if let Some(parent) = Path::new(file_path).parent() {
            std::fs::create_dir_all(parent)?;
        }
        
        let file = File::create(file_path)?;
        serde_json::to_writer(file, &self)?;
        Ok(())
    }

    pub fn load_from_file(file_path: &str) -> io::Result<Self> {
        let file = File::open(file_path)?;
        let history: CommandHistory = serde_json::from_reader(file)?;
        Ok(history)
    }
}