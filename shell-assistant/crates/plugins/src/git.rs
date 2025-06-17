use crate::traits::{Plugin, CommandResult};

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
    
    fn can_handle(&self, input: &str) -> bool {
        input.to_lowercase().contains("git") || 
        input.to_lowercase().contains("commit") || 
        input.to_lowercase().contains("repository") ||
        input.to_lowercase().contains("branch") ||
        input.to_lowercase().contains("push") ||
        input.to_lowercase().contains("pull") ||
        input.to_lowercase().contains("clone")
    }
    
    fn handle(&self, input: &str) -> Option<CommandResult> {
        let input_lower = input.to_lowercase();
        
        // Pattern matching for common Git operations
        if input_lower.contains("status") || input_lower.contains("what changed") {
            return Some(CommandResult {
                command: "git status".to_string(),
                explanation: "Shows the working tree status, including tracked and untracked files.".to_string(),
                executed: false,
                output: None,
            });
        }
        
        if input_lower.contains("commit") {
            if input_lower.contains("message") && input_lower.contains("\"") {
                // Extract message between quotes if present
                if let Some(message) = extract_quoted_text(input) {
                    return Some(CommandResult {
                        command: format!("git commit -m \"{}\"", message),
                        explanation: "Commits changes with the specified message.".to_string(),
                        executed: false,
                        output: None,
                    });
                }
            }
            
            return Some(CommandResult {
                command: "git commit -m \"\"".to_string(),
                explanation: "Commits the staged changes. You'll need to provide a commit message.".to_string(),
                executed: false,
                output: None,
            });
        }
        
        if input_lower.contains("add") || input_lower.contains("stage") {
            if input_lower.contains("all") || input_lower.contains("everything") {
                return Some(CommandResult {
                    command: "git add .".to_string(),
                    explanation: "Stages all changes in the working directory.".to_string(),
                    executed: false,
                    output: None,
                });
            }
            
            // Try to extract specific files
            if let Some(file) = extract_file_reference(input) {
                return Some(CommandResult {
                    command: format!("git add {}", file),
                    explanation: format!("Stages changes to the file '{}'.", file),
                    executed: false,
                    output: None,
                });
            }
            
            return Some(CommandResult {
                command: "git add ".to_string(),
                explanation: "Stages changes. You'll need to specify which files to stage.".to_string(),
                executed: false,
                output: None,
            });
        }
        
        if input_lower.contains("log") || input_lower.contains("history") {
            return Some(CommandResult {
                command: "git log".to_string(),
                explanation: "Shows the commit history.".to_string(),
                executed: false,
                output: None,
            });
        }
        
        if input_lower.contains("branch") {
            if input_lower.contains("list") || input_lower.contains("show") {
                return Some(CommandResult {
                    command: "git branch".to_string(),
                    explanation: "Lists all local branches.".to_string(),
                    executed: false,
                    output: None,
                });
            }
            
            if input_lower.contains("create") || input_lower.contains("new") {
                if let Some(branch_name) = extract_branch_name(input) {
                    return Some(CommandResult {
                        command: format!("git branch {}", branch_name),
                        explanation: format!("Creates a new branch named '{}'.", branch_name),
                        executed: false,
                        output: None,
                    });
                }
            }
            
            if input_lower.contains("switch") || input_lower.contains("checkout") {
                if let Some(branch_name) = extract_branch_name(input) {
                    return Some(CommandResult {
                        command: format!("git checkout {}", branch_name),
                        explanation: format!("Switches to the branch named '{}'.", branch_name),
                        executed: false,
                        output: None,
                    });
                }
            }
        }
        
        if input_lower.contains("push") {
            return Some(CommandResult {
                command: "git push".to_string(),
                explanation: "Pushes commits to the remote repository.".to_string(),
                executed: false,
                output: None,
            });
        }
        
        if input_lower.contains("pull") {
            return Some(CommandResult {
                command: "git pull".to_string(),
                explanation: "Fetches changes from the remote repository and merges them into the current branch.".to_string(),
                executed: false,
                output: None,
            });
        }
        
        if input_lower.contains("clone") {
            if let Some(url) = extract_url(input) {
                return Some(CommandResult {
                    command: format!("git clone {}", url),
                    explanation: format!("Clones the repository from '{}'.", url),
                    executed: false,
                    output: None,
                });
            }
            
            return Some(CommandResult {
                command: "git clone ".to_string(),
                explanation: "Clones a repository. You'll need to specify the repository URL.".to_string(),
                executed: false,
                output: None,
            });
        }
        
        // Default fallback for other git commands
        Some(CommandResult {
            command: "git ".to_string(),
            explanation: "Git is a distributed version control system.".to_string(),
            executed: false,
            output: None,
        })
    }
}

// Helper functions for extracting information from input
fn extract_quoted_text(input: &str) -> Option<String> {
    let parts: Vec<&str> = input.split('"').collect();
    if parts.len() >= 3 {
        Some(parts[1].to_string())
    } else {
        None
    }
}

fn extract_file_reference(input: &str) -> Option<String> {
    // Very simple extraction - would need to be more sophisticated in a real implementation
    let words: Vec<&str> = input.split_whitespace().collect();
    let idx = words.iter().position(|&w| w.to_lowercase() == "file" || w.to_lowercase() == "files")?;
    
    if idx + 1 < words.len() {
        Some(words[idx + 1].trim_matches(|c: char| !c.is_alphanumeric() && c != '.' && c != '_').to_string())
    } else {
        None
    }
}

fn extract_branch_name(input: &str) -> Option<String> {
    let words: Vec<&str> = input.split_whitespace().collect();
    let idx = words.iter().position(|&w| 
        w.to_lowercase() == "branch" || 
        w.to_lowercase() == "to" || 
        w.to_lowercase() == "named" ||
        w.to_lowercase() == "called"
    )?;
    
    if idx + 1 < words.len() {
        Some(words[idx + 1].trim_matches(|c: char| !c.is_alphanumeric() && c != '-' && c != '_').to_string())
    } else {
        None
    }
}

fn extract_url(input: &str) -> Option<String> {
    // Simple URL extraction - a more robust implementation would use regex
    let words: Vec<&str> = input.split_whitespace().collect();
    words.iter()
        .find(|w| w.starts_with("http://") || w.starts_with("https://") || w.starts_with("git@"))
        .map(|s| s.to_string())
}