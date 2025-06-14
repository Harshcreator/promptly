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

// Construct a prompt for the LLM that asks it to generate a shell command
pub fn construct_prompt(user_input: &str) -> String {
    let os_type = if cfg!(windows) { "Windows PowerShell" } else { "Unix/Linux bash" };
    
    format!(
        r#"You are a shell command assistant. Convert the following natural language query into a {os_type} command.
Your response must be in this JSON format:
{{
  "command": "the actual shell command",
  "explanation": "brief explanation of what the command does"
}}

The command should be valid for {os_type}. Do not include any markdown formatting, just return valid JSON.

USER QUERY: {user_input}
"#,
        os_type = os_type,
        user_input = user_input
    )
}