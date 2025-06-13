use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Prompt {
    pub user_input: String,
    pub command: String,
    pub explanation: String,
}

impl Prompt {
    pub fn new(user_input: String, command: String, explanation: String) -> Self {
        Self {
            user_input,
            command,
            explanation,
        }
    }
}

pub fn construct_prompt(user_input: &str) -> String {
    format!(
        "You are a shell command assistant. Convert the following natural language query into a shell command:\n\n{}",
        user_input
    )
}