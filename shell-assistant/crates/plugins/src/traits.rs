use std::error::Error;

/// Represents the result of a command execution by a plugin
#[derive(Debug, Clone)]
pub struct CommandResult {
    /// The actual shell command to be executed
    pub command: String,
    
    /// An explanation of what the command does
    pub explanation: String,
    
    /// Whether the command was directly executed by the plugin
    pub executed: bool,
    
    /// The output of the command if it was executed
    pub output: Option<String>,
}

pub trait Plugin {
    /// Returns the name of the plugin
    fn name(&self) -> &str;
    
    /// Returns a description of the plugin's functionality
    fn description(&self) -> &str;
    
    /// Checks if this plugin can handle the given natural language input
    fn can_handle(&self, input: &str) -> bool;
    
    /// Processes the natural language input and returns a command result if applicable
    fn handle(&self, input: &str) -> Option<CommandResult>;
    
    /// Process method that calls handle if can_handle returns true
    fn process(&self, input: &str) -> Option<CommandResult> {
        if self.can_handle(input) {
            self.handle(input)
        } else {
            None
        }
    }
}