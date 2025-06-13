use std::fs::File;
use std::io::{self, Write, Read};
use std::path::Path;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CommandEntry {
    pub input: String,
    pub command: String,
    pub timestamp: u64,
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

    pub fn add_entry(&mut self, input: String, command: String) {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
            
        self.entries.push(CommandEntry {
            input,
            command,
            timestamp,
        });
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