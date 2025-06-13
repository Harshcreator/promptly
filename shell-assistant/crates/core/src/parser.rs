use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct LLMResponse {
    pub command: String,
    pub explanation: String,
}

pub fn parse_response(response: &str) -> Result<(String, String), String> {
    match serde_json::from_str::<LLMResponse>(response) {
        Ok(parsed) => Ok((parsed.command, parsed.explanation)),
        Err(e) => Err(format!("Failed to parse LLM response: {}", e)),
    }
}

// Mock function to simulate LLM response
pub async fn mock_llm_call(_prompt: &str) -> Result<String, String> {
    // For the end-to-end test, always return the same response
    let command = if cfg!(windows) {
        "Get-ChildItem -Force"
    } else {
        "ls -la"
    };
    
    let explanation = "Lists all files with details in the current directory, including hidden files.";
    
    let response = LLMResponse {
        command: command.to_string(),
        explanation: explanation.to_string(),
    };
    
    match serde_json::to_string(&response) {
        Ok(json) => Ok(json),
        Err(e) => Err(format!("Failed to serialize response: {}", e)),
    }
}