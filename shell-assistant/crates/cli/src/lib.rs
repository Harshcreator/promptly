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
}

pub fn copy_to_clipboard(text: &str) -> Result<(), String> {
    // This is a placeholder function - in a real application we would use
    // a clipboard library like `clipboard` or `arboard`
    println!("Text copied to clipboard: {}", text);
    Ok(())
}