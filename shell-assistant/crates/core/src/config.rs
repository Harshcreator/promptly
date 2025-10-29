use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Failed to read config file: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("Failed to parse config file: {0}")]
    ParseError(#[from] serde_yaml::Error),
    
    #[error("Config file not found")]
    NotFound,
}

/// Enterprise configuration for Shell Assistant
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnterpriseConfig {
    #[serde(default = "default_version")]
    pub version: String,
    
    #[serde(default)]
    pub llm: LLMConfig,
    
    #[serde(default)]
    pub security: SecurityConfig,
    
    #[serde(default)]
    pub privacy: PrivacyConfig,
    
    #[serde(default)]
    pub enterprise: EnterpriseSettings,
}

/// LLM configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMConfig {
    /// Backend to use: "ollama", "openai", or "llm-rs"
    #[serde(default = "default_backend")]
    pub backend: String,
    
    /// Path to local GGUF model file (for llm-rs backend)
    pub model_path: Option<String>,
    
    /// Model name (for ollama/openai)
    #[serde(default = "default_model")]
    pub model: String,
    
    /// Maximum tokens to generate
    #[serde(default = "default_max_tokens")]
    pub max_tokens: u32,
    
    /// Temperature for generation (0.0 - 2.0)
    #[serde(default = "default_temperature")]
    pub temperature: f32,
    
    /// Top-p sampling
    #[serde(default = "default_top_p")]
    pub top_p: f32,
    
    /// Number of CPU threads to use
    #[serde(default = "default_threads")]
    pub threads: u32,
    
    /// Use memory mapping for model loading
    #[serde(default = "default_use_mmap")]
    pub use_mmap: bool,
    
    /// Lock model in memory
    #[serde(default)]
    pub use_mlock: bool,
    
    /// Number of GPU layers to offload (0 = CPU only)
    #[serde(default)]
    pub gpu_layers: u32,
}

/// Security configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    /// Enable safety checker for dangerous commands
    #[serde(default = "default_true")]
    pub safety_check: bool,
    
    /// Always require confirmation before executing commands
    #[serde(default = "default_true")]
    pub always_confirm: bool,
    
    /// Disable automatic execution
    #[serde(default)]
    pub auto_execute: bool,
    
    /// Enable audit logging
    #[serde(default = "default_true")]
    pub audit_log: bool,
    
    /// Path to audit log file
    pub audit_log_path: Option<String>,
}

/// Privacy configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivacyConfig {
    /// Disable telemetry (always disabled in enterprise builds)
    #[serde(default)]
    pub telemetry: bool,
    
    /// Enable offline-only mode (block external network calls)
    #[serde(default = "default_true")]
    pub offline_only: bool,
    
    /// Save command history
    #[serde(default = "default_true")]
    pub save_history: bool,
    
    /// Path to history file
    pub history_path: Option<String>,
}

/// Enterprise-specific settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnterpriseSettings {
    /// Organization name
    pub organization: Option<String>,
    
    /// Department/Team name
    pub department: Option<String>,
    
    /// Enable compliance mode
    #[serde(default = "default_true")]
    pub compliance_mode: bool,
    
    /// Allowed command patterns (whitelist)
    #[serde(default)]
    pub allowed_commands: Vec<String>,
    
    /// Blocked command patterns (blacklist)
    #[serde(default)]
    pub blocked_commands: Vec<String>,
}

// Default value functions
fn default_version() -> String {
    "1.0".to_string()
}

fn default_backend() -> String {
    "ollama".to_string()
}

fn default_model() -> String {
    "codellama".to_string()
}

fn default_max_tokens() -> u32 {
    256
}

fn default_temperature() -> f32 {
    0.7
}

fn default_top_p() -> f32 {
    0.9
}

fn default_threads() -> u32 {
    num_cpus::get() as u32
}

fn default_use_mmap() -> bool {
    true
}

fn default_true() -> bool {
    true
}

// Default implementations
impl Default for EnterpriseConfig {
    fn default() -> Self {
        Self {
            version: default_version(),
            llm: LLMConfig::default(),
            security: SecurityConfig::default(),
            privacy: PrivacyConfig::default(),
            enterprise: EnterpriseSettings::default(),
        }
    }
}

impl Default for LLMConfig {
    fn default() -> Self {
        Self {
            backend: default_backend(),
            model_path: None,
            model: default_model(),
            max_tokens: default_max_tokens(),
            temperature: default_temperature(),
            top_p: default_top_p(),
            threads: default_threads(),
            use_mmap: default_use_mmap(),
            use_mlock: false,
            gpu_layers: 0,
        }
    }
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            safety_check: true,
            always_confirm: true,
            auto_execute: false,
            audit_log: true,
            audit_log_path: None,
        }
    }
}

impl Default for PrivacyConfig {
    fn default() -> Self {
        Self {
            telemetry: false,
            offline_only: true,
            save_history: true,
            history_path: None,
        }
    }
}

impl Default for EnterpriseSettings {
    fn default() -> Self {
        Self {
            organization: None,
            department: None,
            compliance_mode: true,
            allowed_commands: vec![],
            blocked_commands: vec![
                "rm -rf /".to_string(),
                "format".to_string(),
                "del /s /q C:\\".to_string(),
            ],
        }
    }
}

impl EnterpriseConfig {
    /// Get the default config path (~/.shell-assistant/config.yaml)
    pub fn default_path() -> PathBuf {
        let home = dirs::home_dir().expect("Unable to find home directory");
        home.join(".shell-assistant").join("config.yaml")
    }
    
    /// Load configuration from default path or create default if not found
    pub fn load() -> Result<Self, ConfigError> {
        Self::load_from(&Self::default_path())
    }
    
    /// Load configuration from a specific path
    pub fn load_from(path: &PathBuf) -> Result<Self, ConfigError> {
        if !path.exists() {
            tracing::info!("Config file not found at {:?}, using defaults", path);
            return Ok(Self::default());
        }
        
        let contents = fs::read_to_string(path)?;
        let config: EnterpriseConfig = serde_yaml::from_str(&contents)?;
        
        tracing::info!("Loaded config from {:?}", path);
        Ok(config)
    }
    
    /// Save configuration to default path
    pub fn save(&self) -> Result<(), ConfigError> {
        self.save_to(&Self::default_path())
    }
    
    /// Save configuration to a specific path
    pub fn save_to(&self, path: &PathBuf) -> Result<(), ConfigError> {
        // Create parent directory if it doesn't exist
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        
        let yaml = serde_yaml::to_string(self)?;
        fs::write(path, yaml)?;
        
        tracing::info!("Saved config to {:?}", path);
        Ok(())
    }
    
    /// Get the model path, resolving relative paths and expanding ~ 
    pub fn get_model_path(&self) -> Option<PathBuf> {
        self.llm.model_path.as_ref().map(|path| {
            let path_str = if path.starts_with('~') {
                let home = dirs::home_dir().expect("Unable to find home directory");
                path.replacen('~', home.to_str().unwrap(), 1)
            } else {
                path.clone()
            };
            PathBuf::from(path_str)
        })
    }
    
    /// Get the audit log path, using default if not specified
    pub fn get_audit_log_path(&self) -> PathBuf {
        self.security.audit_log_path.as_ref()
            .map(|p| PathBuf::from(p))
            .unwrap_or_else(|| {
                let home = dirs::home_dir().expect("Unable to find home directory");
                home.join(".shell-assistant").join("audit.log")
            })
    }
    
    /// Get the history path, using default if not specified
    pub fn get_history_path(&self) -> PathBuf {
        self.privacy.history_path.as_ref()
            .map(|p| PathBuf::from(p))
            .unwrap_or_else(|| {
                let home = dirs::home_dir().expect("Unable to find home directory");
                home.join(".shell-assistant").join("history.json")
            })
    }
    
    /// Check if a command is allowed based on whitelist/blacklist
    pub fn is_command_allowed(&self, command: &str) -> bool {
        // First check blacklist
        for pattern in &self.enterprise.blocked_commands {
            if command.contains(pattern) {
                return false;
            }
        }
        
        // If whitelist is empty, allow all (except blacklisted)
        if self.enterprise.allowed_commands.is_empty() {
            return true;
        }
        
        // Check whitelist
        for pattern in &self.enterprise.allowed_commands {
            if command.starts_with(pattern) {
                return true;
            }
        }
        
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_default_config() {
        let config = EnterpriseConfig::default();
        assert_eq!(config.version, "1.0");
        assert_eq!(config.llm.backend, "ollama");
        assert!(config.security.safety_check);
        assert!(config.privacy.offline_only);
    }
    
    #[test]
    fn test_command_blacklist() {
        let config = EnterpriseConfig::default();
        assert!(!config.is_command_allowed("rm -rf /"));
        assert!(config.is_command_allowed("ls -la"));
    }
    
    #[test]
    fn test_command_whitelist() {
        let mut config = EnterpriseConfig::default();
        config.enterprise.allowed_commands = vec!["git".to_string(), "ls".to_string()];
        
        assert!(config.is_command_allowed("git status"));
        assert!(config.is_command_allowed("ls -la"));
        assert!(!config.is_command_allowed("rm file.txt"));
    }
}
