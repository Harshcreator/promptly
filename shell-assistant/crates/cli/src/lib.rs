use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about = "A natural language shell command assistant")]
pub struct CliArgs {
    /// Natural language input for the shell command
    #[clap(value_parser)]
    pub input: Option<String>,
    
    /// Run in dry-run mode (don't execute commands)
    #[clap(short, long, action)]
    pub dry_run: bool,
    
    /// Show command history
    #[clap(short = 'H', long, action)]
    pub history: bool,
    
    /// Path to config file
    #[clap(short, long, value_parser)]
    pub config: Option<String>,
    
    /// LLM backend to use (ollama, llm-rs, openai)
    /// - ollama: Uses the Ollama API (http://localhost:11434) with codellama model
    /// - llm-rs: Uses the llm-rs crate with a local GGUF model
    /// - openai: Uses the OpenAI API with gpt-3.5-turbo model
    #[clap(short, long, value_parser, default_value = "ollama")]
    pub backend: String,
    
    /// Force online mode (use OpenAI if other backends fail)
    /// Also selects wizardcoder model for Ollama
    #[clap(long, action)]
    pub online: bool,
      /// Path to local LLM model for llm-rs backend
    /// Default: "models/tinyllama.gguf"
    #[clap(long, value_parser)]
    pub model_path: Option<String>,
    
    /// Path to history file
    /// Default: ~/.shell-assistant/history.json
    #[clap(long, value_parser)]
    pub history_file: Option<String>,
}

pub fn copy_to_clipboard(text: &str) -> Result<(), String> {
    // This is a placeholder function - in a real application we would use
    // a clipboard library like `clipboard` or `arboard`
    println!("Text copied to clipboard: {}", text);
    Ok(())
}