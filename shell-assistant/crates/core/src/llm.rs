use async_trait::async_trait;
use thiserror::Error;
use std::env;
use serde::{Deserialize, Serialize};
#[cfg(feature = "llm-rs")]
use llama_cpp;
#[cfg(feature = "llm-rs")]
use std::path::Path;

// Define error types for LLM operations
#[derive(Error, Debug)]
pub enum LLMError {
    #[error("Network error: {0}")]
    NetworkError(#[from] reqwest::Error),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("Local model error: {0}")]
    LocalModelError(String),

    #[error("API key not found: {0}")]
    ApiKeyError(String),

    #[error("Parsing error: {0}")]
    ParsingError(String),

    #[error("Rate limit exceeded")]
    RateLimitExceeded,

    #[error("Unknown error: {0}")]
    Unknown(String),
}

// Define a generic trait for LLM engines
#[async_trait]
pub trait LLMEngine: Send + Sync {
    async fn generate(&self, prompt: &str) -> Result<String, LLMError>;
    fn name(&self) -> &str;
    
    /// Returns true if this LLM requires internet access
    fn is_online(&self) -> bool {
        false // Default implementation assumes local model
    }
}

// Ollama LLM implementation
pub struct OllamaProvider {
    api_url: String,
    model: String,
}

impl OllamaProvider {
    pub fn new(model: &str) -> Self {
        Self {
            api_url: "http://localhost:11434/api/generate".to_string(),
            model: model.to_string(),
        }
    }
}

#[derive(Serialize)]
struct OllamaRequest<'a> {
    model: &'a str,
    prompt: &'a str,
    stream: bool,
}

#[derive(Deserialize)]
struct OllamaResponse {
    response: String,
}

#[async_trait]
impl LLMEngine for OllamaProvider {
    async fn generate(&self, prompt: &str) -> Result<String, LLMError> {
        let client = reqwest::Client::new();
        let request = OllamaRequest {
            model: &self.model,
            prompt,
            stream: false,
        };

        let response = client
            .post(&self.api_url)
            .json(&request)
            .send()
            .await?
            .json::<OllamaResponse>()
            .await?;

        Ok(response.response)
    }

    fn name(&self) -> &str {
        "Ollama"
    }

    fn is_online(&self) -> bool {
        // WizardCoder model usually needs to be downloaded
        self.model == "wizardcoder"
    }
}

// OpenAI LLM implementation
pub struct OpenAIProvider {
    api_key: String,
    model: String,
    call_count: std::sync::atomic::AtomicUsize,
    max_calls: usize,
}

impl OpenAIProvider {
    pub fn new() -> Result<Self, LLMError> {
        Self::new_with_model("gpt-3.5-turbo")
    }

    pub fn new_with_model(model: &str) -> Result<Self, LLMError> {
        // Load from .env file if it exists
        let _ = dotenv::dotenv();
        
        // Get API key from environment
        let api_key = env::var("OPENAI_API_KEY")
            .map_err(|_| LLMError::ApiKeyError(
                "OPENAI_API_KEY environment variable not set. Please set your OpenAI API key.".into()
            ))?;

        // Validate API key format (should start with sk-)
        if !api_key.starts_with("sk-") {
            return Err(LLMError::ApiKeyError(
                "Invalid OpenAI API key format. API keys should start with 'sk-'".into()
            ));
        }

        Ok(Self {
            api_key,
            model: model.to_string(),
            call_count: std::sync::atomic::AtomicUsize::new(0),
            max_calls: 50, // Limit to 50 calls per session
        })
    }

    pub fn get_model(&self) -> &str {
        &self.model
    }

    pub fn set_model(&mut self, model: String) {
        self.model = model;
    }
}

#[derive(Serialize)]
struct OpenAIRequest<'a> {
    model: &'a str,
    messages: Vec<OpenAIMessage<'a>>,
}

#[derive(Serialize)]
struct OpenAIMessage<'a> {
    role: &'a str,
    content: &'a str,
}

#[derive(Deserialize)]
struct OpenAIResponse {
    choices: Vec<OpenAIChoice>,
}

#[derive(Deserialize)]
struct OpenAIChoice {
    message: OpenAIResponseMessage,
}

#[derive(Deserialize)]
struct OpenAIResponseMessage {
    content: String,
}

#[async_trait]
impl LLMEngine for OpenAIProvider {
    async fn generate(&self, prompt: &str) -> Result<String, LLMError> {
        // Check if we've exceeded the call limit
        let current_count = self.call_count.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        if current_count >= self.max_calls {
            return Err(LLMError::RateLimitExceeded);
        }

        let client = reqwest::Client::new();
        let request = OpenAIRequest {
            model: &self.model,
            messages: vec![OpenAIMessage {
                role: "user",
                content: prompt,
            }],
        };

        let response = client
            .post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await;

        let response = match response {
            Ok(resp) => resp,
            Err(e) => return Err(LLMError::NetworkError(e)),
        };

        // Check for HTTP errors
        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            
            return match status.as_u16() {
                401 => Err(LLMError::ApiKeyError("Invalid OpenAI API key. Please check your OPENAI_API_KEY environment variable.".into())),
                429 => Err(LLMError::RateLimitExceeded),
                _ => Err(LLMError::ParsingError(format!("OpenAI API error ({}): {}", status, error_text))),
            };
        }

        let openai_response: OpenAIResponse = response.json().await
            .map_err(|e| LLMError::ParsingError(format!("Failed to parse OpenAI response: {}", e)))?;

        if openai_response.choices.is_empty() {
            return Err(LLMError::ParsingError("No choices in OpenAI response".into()));
        }

        Ok(openai_response.choices[0].message.content.clone())
    }

    fn name(&self) -> &str {
        "OpenAI"
    }

    fn is_online(&self) -> bool {
        // OpenAI is always online
        true
    }
}

// LLM-rs (local) implementation
#[cfg(feature = "llm-rs")]
pub struct LlmRsProvider {
    model_path: String,
    model: OnceCell<llama_cpp::LlamaModel>,
}

#[cfg(feature = "llm-rs")]
impl LlmRsProvider {
    pub fn new(model_path: &str) -> Self {
        Self {
            model_path: model_path.to_string(),
            model: OnceCell::new(),
        }
    }

    fn get_model(&self) -> Result<&llama_cpp::LlamaModel, LLMError> {
        self.model.get_or_try_init(|| {
            let model_path = Path::new(&self.model_path);
            if !model_path.exists() {
                return Err(LLMError::LocalModelError(format!(
                    "Model file not found: {}",
                    self.model_path
                )));
            }
            
            // Create a new model with default parameters
            let model_params = llama_cpp::ModelParameters::default();
            let model = llama_cpp::LlamaModel::load_from_file(&self.model_path, model_params)
                .map_err(|e| LLMError::LocalModelError(format!("Failed to load model: {}", e)))?;
            
            Ok(model)
        })
    }
}

#[cfg(feature = "llm-rs")]
#[async_trait]
impl LLMEngine for LlmRsProvider {
    async fn generate(&self, prompt: &str) -> Result<String, LLMError> {
        let model = self.get_model()?;
        
        // Create a new session with default parameters
        let session_params = llama_cpp::SessionParameters::default();
        let mut session = model.create_session(session_params)
            .map_err(|e| LLMError::LocalModelError(format!("Failed to create session: {}", e)))?;
            
        // Set inference parameters
        let inference_params = llama_cpp::InferenceParameters::default()
            .max_tokens(256);
        
        // Generate text
        let result = session.infer(
            prompt,
            inference_params,
            |_token_id, token| {
                print!("{}", token);
                true // continue inference
            },
        )
        .map_err(|e| LLMError::LocalModelError(format!("Inference error: {}", e)))?;
        
        Ok(result.text)
    }

    fn name(&self) -> &str {
        "LLM-rs"
    }
}

#[cfg(not(feature = "llm-rs"))]
pub struct LlmRsProvider;

#[cfg(not(feature = "llm-rs"))]
impl LlmRsProvider {
    pub fn new(_model_path: &str) -> Self {
        Self
    }
}

#[cfg(not(feature = "llm-rs"))]
#[async_trait]
impl LLMEngine for LlmRsProvider {
    async fn generate(&self, _prompt: &str) -> Result<String, LLMError> {
        Err(LLMError::LocalModelError(
            "LLM-rs feature is not enabled. To enable it, build with --features \"core/llm-rs\" and ensure you have libclang installed (for Windows, install LLVM from https://github.com/llvm/llvm-project/releases/)".into()
        ))
    }

    fn name(&self) -> &str {
        "LLM-rs (disabled)"
    }
}

// LLM Provider enum
pub enum LLMProvider {
    Ollama(OllamaProvider),
    LlmRs(LlmRsProvider),
    OpenAI(OpenAIProvider),
}

impl LLMProvider {
    pub fn default() -> Self {
        Self::Ollama(OllamaProvider::new("codellama"))
    }

    pub fn is_online(&self) -> bool {
        match self {
            Self::Ollama(provider) => provider.model == "wizardcoder", // Wizardcoder requires download
            Self::OpenAI(_) => true,
            Self::LlmRs(_) => false,
        }
    }

    pub async fn generate_with_fallback(&self, prompt: &str) -> Result<String, LLMError> {
        match self {
            LLMProvider::Ollama(provider) => {
                match provider.generate(prompt).await {
                    Ok(response) => Ok(response),
                    Err(e) => {                        
                        println!("Ollama failed: {}. Falling back to LLM-rs...", e);
                        match &LLMProvider::LlmRs(LlmRsProvider::new("models/tinyllama.gguf")) {
                            LLMProvider::LlmRs(provider) => {
                                match provider.generate(prompt).await {
                                    Ok(response) => Ok(response),
                                    Err(e) => {
                                        println!("LLM-rs failed: {}. OpenAI fallback disabled.", e);
                                        // Return the error instead of falling back to OpenAI
                                        Err(e)
                                        // Commented out OpenAI fallback for now
                                        /*
                                        match OpenAIProvider::new() {
                                            Ok(openai_provider) => {
                                                let openai = LLMProvider::OpenAI(openai_provider);
                                                match &openai {
                                                    LLMProvider::OpenAI(provider) => provider.generate(prompt).await,
                                                    _ => unreachable!(),
                                                }
                                            },
                                            Err(e) => Err(e),
                                        }
                                        */
                                    }
                                }
                            },
                            _ => unreachable!(),
                        }
                    }
                }
            },            LLMProvider::LlmRs(provider) => {
                match provider.generate(prompt).await {
                    Ok(response) => Ok(response),
                    Err(e) => {
                        println!("LLM-rs failed: {}. OpenAI fallback disabled.", e);
                        // Return the error instead of falling back to OpenAI
                        Err(e)
                        // Commented out OpenAI fallback for now
                        /*
                        match OpenAIProvider::new() {
                            Ok(openai_provider) => {
                                let openai = LLMProvider::OpenAI(openai_provider);
                                match &openai {
                                    LLMProvider::OpenAI(provider) => provider.generate(prompt).await,
                                    _ => unreachable!(),
                                }
                            },
                            Err(e) => Err(e),
                        }
                        */
                    }
                }
            },
            LLMProvider::OpenAI(provider) => provider.generate(prompt).await,
        }
    }
}

#[async_trait]
impl LLMEngine for LLMProvider {
    async fn generate(&self, prompt: &str) -> Result<String, LLMError> {
        match self {
            LLMProvider::Ollama(provider) => provider.generate(prompt).await,
            LLMProvider::LlmRs(provider) => provider.generate(prompt).await,
            LLMProvider::OpenAI(provider) => provider.generate(prompt).await,
        }
    }

    fn name(&self) -> &str {
        match self {
            LLMProvider::Ollama(provider) => provider.name(),
            LLMProvider::LlmRs(provider) => provider.name(),
            LLMProvider::OpenAI(provider) => provider.name(),
        }
    }
}
