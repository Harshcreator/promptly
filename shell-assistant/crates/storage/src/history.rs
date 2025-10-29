use crate::persistence::{CommandEntry, CommandHistory as PersistentHistory, FeedbackType};
use std::collections::VecDeque;
use std::io;

const DEFAULT_HISTORY_SIZE: usize = 100;

#[derive(Debug, Clone)]
pub struct CommandHistory {
    history: VecDeque<CommandEntry>,
    max_size: usize,
    file_path: Option<String>,
}

impl Default for CommandHistory {
    fn default() -> Self {
        Self::new()
    }
}

impl CommandHistory {
    pub fn new() -> Self {
        CommandHistory {
            history: VecDeque::with_capacity(DEFAULT_HISTORY_SIZE),
            max_size: DEFAULT_HISTORY_SIZE,
            file_path: None,
        }
    }

    /// Create a new CommandHistory with file persistence
    pub fn with_persistence(file_path: String) -> Self {
        let mut history = CommandHistory {
            history: VecDeque::with_capacity(DEFAULT_HISTORY_SIZE),
            max_size: DEFAULT_HISTORY_SIZE,
            file_path: Some(file_path),
        };

        // Try to load existing history
        if let Err(e) = history.load_from_file() {
            if e.kind() != io::ErrorKind::NotFound {
                eprintln!("Warning: Could not load history file: {}", e);
            }
        }

        history
    }

    pub fn with_capacity(max_size: usize) -> Self {
        CommandHistory { history: VecDeque::with_capacity(max_size), max_size, file_path: None }
    }

    pub fn set_max_size(&mut self, max_size: usize) {
        self.max_size = max_size;
        while self.history.len() > max_size {
            self.history.pop_front();
        }
    }

    pub fn add_entry(&mut self, input: String, command: String, explanation: Option<String>) {
        let timestamp =
            std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();

        if self.history.len() == self.max_size {
            self.history.pop_front();
        }

        self.history.push_back(CommandEntry {
            input,
            command,
            explanation,
            timestamp,
            feedback: FeedbackType::None,
            original_command: None,
        });

        // Save to file if persistence is enabled
        if let Some(_file_path) = &self.file_path {
            if let Err(e) = self.save_to_file() {
                eprintln!("Warning: Could not save history to file: {}", e);
            }
        }
    }

    /// Add entry with feedback and possibly edited command
    pub fn add_entry_with_feedback(
        &mut self,
        input: String,
        command: String,
        explanation: Option<String>,
        feedback: FeedbackType,
        original_command: Option<String>,
    ) {
        let timestamp =
            std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();

        if self.history.len() == self.max_size {
            self.history.pop_front();
        }

        self.history.push_back(CommandEntry {
            input,
            command,
            explanation,
            timestamp,
            feedback,
            original_command,
        });

        // Save to file if persistence is enabled
        if let Some(_file_path) = &self.file_path {
            if let Err(e) = self.save_to_file() {
                eprintln!("Warning: Could not save history to file: {}", e);
            }
        }
    }

    /// Update the last entry with feedback
    pub fn update_last_entry_feedback(
        &mut self,
        feedback: FeedbackType,
        edited_command: Option<String>,
    ) -> bool {
        if let Some(last_entry) = self.history.back_mut() {
            if feedback == FeedbackType::Edited {
                // Store original command if edited
                last_entry.original_command = Some(last_entry.command.clone());
                // Update with edited version
                if let Some(cmd) = edited_command {
                    last_entry.command = cmd;
                }
            }
            last_entry.feedback = feedback;

            // Save to file if persistence is enabled
            if let Some(_file_path) = &self.file_path {
                if let Err(e) = self.save_to_file() {
                    eprintln!("Warning: Could not save history file after feedback update: {}", e);
                }
            }

            true
        } else {
            false
        }
    }

    pub fn get_history(&self) -> Vec<CommandEntry> {
        self.history.iter().cloned().collect()
    }

    /// Get a reference to the internal history entries
    pub fn entries(&self) -> &VecDeque<CommandEntry> {
        &self.history
    }

    /// Get the file path for history storage
    pub fn get_file_path(&self) -> Option<&String> {
        self.file_path.as_ref()
    }

    pub fn clear(&mut self) {
        self.history.clear();
    }

    pub fn get_recent(&self, count: usize) -> Vec<CommandEntry> {
        self.history.iter().rev().take(count).cloned().collect()
    }

    /// Set the file path for history persistence
    pub fn set_file_path(&mut self, file_path: String) {
        self.file_path = Some(file_path);
    }

    /// Save history to file
    pub fn save_to_file(&self) -> io::Result<()> {
        if let Some(file_path) = &self.file_path {
            let persistent = PersistentHistory { entries: self.history.iter().cloned().collect() };
            persistent.save_to_file(file_path)?;
        }
        Ok(())
    }

    /// Load history from file
    pub fn load_from_file(&mut self) -> io::Result<()> {
        if let Some(file_path) = &self.file_path {
            let persistent = PersistentHistory::load_from_file(file_path)?;

            // Clear current history and load from file
            self.history.clear();

            // Only load up to max_size entries, most recent first
            for entry in persistent.entries.into_iter().rev().take(self.max_size).rev() {
                self.history.push_back(entry);
            }
        }
        Ok(())
    }

    /// Get default history file path
    pub fn default_history_path() -> io::Result<String> {
        PersistentHistory::default_history_path()
    }
}
