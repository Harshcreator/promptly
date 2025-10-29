use crate::llm::{LLMError, LLMProvider};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct LLMResponse {
    pub command: String,
    pub explanation: String,
}

// Parse the raw LLM response into a structured format
pub fn parse_response(response: &str) -> Result<(String, String), String> {
    // First try to parse as JSON
    match serde_json::from_str::<LLMResponse>(response) {
        Ok(parsed) => return Ok((parsed.command, parsed.explanation)),
        Err(_) => {
            // If not JSON, try to extract command and explanation from text
            // This handles cases where the LLM returns a non-JSON response
            if let Some((command, explanation)) = extract_command_from_text(response) {
                return Ok((command, explanation));
            }
        }
    }

    Err(format!("Failed to parse LLM response. Raw response: {}", response))
}

// Extract command and explanation from text format
fn extract_command_from_text(text: &str) -> Option<(String, String)> {
    // Look for patterns like "Command: xxx" and "Explanation: yyy"
    let mut command = None;
    let mut explanation = None;

    // Try to find JSON-like responses within the text
    if text.contains("\"command\"") && text.contains("\"explanation\"") {
        // Try to extract JSON object from text
        if let Some(json_start) = text.find('{') {
            if let Some(json_end) = text.rfind('}') {
                let json_str = &text[json_start..=json_end];
                if let Ok(parsed) = serde_json::from_str::<LLMResponse>(json_str) {
                    return Some((parsed.command, parsed.explanation));
                }
            }
        }
    }

    for line in text.lines() {
        let line = line.trim();

        // Extract command
        if line.starts_with("Command:") {
            let cmd = line.trim_start_matches("Command:").trim().to_string();
            if !cmd.is_empty() && command.is_none() {
                command = Some(cmd);
            }
        } else if line.starts_with("```") && !line.contains("```json") {
            let cmd = line.trim_start_matches("```").trim().to_string();
            if !cmd.is_empty() && command.is_none() {
                command = Some(cmd);
            }
        }
        // Extract explanation
        else if line.starts_with("Explanation:") {
            let exp = line.trim_start_matches("Explanation:").trim().to_string();
            if !exp.is_empty() && explanation.is_none() {
                explanation = Some(exp);
            }
        }
    }

    // If we found both command and explanation, return them
    if let (Some(cmd), Some(exp)) = (command, explanation) {
        Some((cmd, exp))
    } else {
        // Fallback: treat first line as command, rest as explanation
        let lines: Vec<&str> = text.lines().collect();
        if !lines.is_empty() {
            let cmd = lines[0].trim().to_string();
            let exp = if lines.len() > 1 {
                lines[1..].join("\n").trim().to_string()
            } else {
                "No explanation provided".to_string()
            };
            Some((cmd, exp))
        } else {
            None
        }
    }
}

// Call the LLM to generate a shell command from natural language
pub async fn generate_command(
    provider: &LLMProvider,
    prompt: &str,
) -> Result<(String, String), LLMError> {
    let response = provider.generate_with_fallback(prompt).await?;

    match parse_response(&response) {
        Ok((command, explanation)) => Ok((command, explanation)),
        Err(e) => Err(LLMError::ParsingError(e)),
    }
}

// Mock function to simulate LLM response (for testing)
pub async fn mock_llm_call(_prompt: &str) -> Result<String, String> {
    // For the end-to-end test, always return the same response
    let command = if cfg!(windows) { "Get-ChildItem -Force" } else { "ls -la" };

    let explanation =
        "Lists all files with details in the current directory, including hidden files.";

    let response =
        LLMResponse { command: command.to_string(), explanation: explanation.to_string() };

    match serde_json::to_string(&response) {
        Ok(json) => Ok(json),
        Err(e) => Err(format!("Failed to serialize response: {}", e)),
    }
}
