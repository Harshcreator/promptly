use std::collections::VecDeque;
use crate::persistence::CommandEntry;

const DEFAULT_HISTORY_SIZE: usize = 100;

#[derive(Debug, Clone)]
pub struct CommandHistory {
    history: VecDeque<CommandEntry>,
    max_size: usize,
}

impl CommandHistory {
    pub fn new() -> Self {
        CommandHistory {
            history: VecDeque::with_capacity(DEFAULT_HISTORY_SIZE),
            max_size: DEFAULT_HISTORY_SIZE,
        }
    }

    pub fn with_capacity(max_size: usize) -> Self {
        CommandHistory {
            history: VecDeque::with_capacity(max_size),
            max_size,
        }
    }

    pub fn add_entry(&mut self, input: String, command: String) {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
            
        if self.history.len() == self.max_size {
            self.history.pop_front();
        }
        
        self.history.push_back(CommandEntry {
            input,
            command,
            timestamp,
        });
    }

    pub fn get_history(&self) -> Vec<CommandEntry> {
        self.history.iter().cloned().collect()
    }

    pub fn clear(&mut self) {
        self.history.clear();
    }
    
    pub fn get_recent(&self, count: usize) -> Vec<CommandEntry> {
        self.history.iter()
            .rev()
            .take(count)
            .cloned()
            .collect()
    }
}