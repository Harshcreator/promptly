use crate::traits::Plugin;
use std::error::Error;

pub struct GitPlugin;

impl GitPlugin {
    pub fn new() -> Self {
        GitPlugin
    }
}

impl Plugin for GitPlugin {
    fn name(&self) -> &str {
        "git"
    }
    
    fn description(&self) -> &str {
        "Provides Git command functionality"
    }
    
    fn can_handle(&self, command: &str) -> bool {
        command.starts_with("git ")
    }
    
    fn execute(&self, command: &str) -> Result<String, Box<dyn Error>> {
        if !self.can_handle(command) {
            return Err("Not a Git command".into());
        }
        
        match command {
            "git status" => Ok("On branch main\nYour branch is up to date with 'origin/main'.".to_string()),
            "git log" => Ok("commit 1234567890abcdef\nAuthor: User <user@example.com>\nDate: Thu Jun 13 10:00:00 2025 -0700\n\nInitial commit".to_string()),
            "git branch" => Ok("* main\n  develop".to_string()),
            "git commit -m \"message\"" => Ok("Created commit abcdef: message".to_string()),
            "git push" => Ok("Pushing to origin...".to_string()),
            _ => Ok(format!("Executing: {}", command)),
        }
    }
}