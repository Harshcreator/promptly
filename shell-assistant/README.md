# Shell Assistant

## Overview
The Shell Assistant is a Rust-based command-line interface (CLI) application that transforms natural language input into shell commands. It leverages a modular architecture with multiple crates to handle various aspects of the application, including command execution, prompt construction, and plugin management.

This smart assistant helps users by translating natural language requests into appropriate shell commands with explanations, reducing the need to remember complex command syntax.

## Project Structure
The project is organized as a Cargo workspace with the following crates:

- **core**: Manages prompt construction, LLM integration, command safety checking, and parsing of responses.
- **executor**: Executes shell commands with options for dry-run and user confirmation, featuring a safety checker.
- **plugins**: Defines a plugin system with specialized handlers for Git and Docker operations.
- **storage**: Stores command history with feedback, either in memory or in a persistent JSON file.
- **cli**: The main binary crate that integrates all components and handles user input/output.

## Features

### Core Features
- **Natural Language Processing**: Accepts natural language input and converts it into shell commands.
- **Multiple LLM Backends**:
  - **Ollama**: Local or online models via the Ollama API.
  - **LLM-rs**: Direct integration with local GGUF models.
  - **OpenAI**: (Experimental) Integration with OpenAI's API.
- **Command Safety**: Built-in safety checks to warn about potentially destructive commands.
- **History Management**: Records commands with timestamps and user feedback.
- **Plugin System**: Extensible plugin architecture for specialized command generation.
- **Terminal Integration**: Can be used as a plugin in existing terminals (PowerShell, Bash, Zsh).

### User Interface
- **Colorized Output**: Uses colors to distinguish between different types of information.
- **Emoji Indicators**: Visual indicators for status (✅), warnings (⚠️), errors (❌), etc.
- **Interactive Selection**: User-friendly menus for actions and feedback.
- **Progress Display**: Clear indication of command execution status.

### Command Execution Options
- **Run Mode**: Execute the command directly.
- **Copy Mode**: Copy the command to clipboard for manual execution.
- **Abort Option**: Cancel execution if the command is not what you want.
- **Force Mode**: Skip confirmation prompts for safe commands with the `--force` flag.
- **Dry Run**: See what commands would be executed without actually running them.

### Plugin System
- **Git Plugin**: Specialized handling for common Git operations.
- **Docker Plugin**: Support for Docker commands and container management.
- **Extensible**: Easy to add new plugins for specialized domains.

### Feedback and Learning
- **Command Feedback**: Mark commands as helpful, not helpful, or provide edited versions.
- **Persistent History**: Commands and feedback are stored for future reference.
- **Auto-save**: History is automatically saved to a JSON file for persistence across sessions.

## Command-Line Options
```
A natural language shell command assistant
Usage: cli.exe [OPTIONS] [INPUT]

Arguments:
  [INPUT]  Natural language input for the shell command

Options:
  -d, --dry-run                      Run in dry-run mode (don't execute commands)
  -H, --history                      Show command history
  -L, --list-plugins                 List available plugins
  -c, --config <CONFIG>              Path to config file
  -b, --backend <BACKEND>            LLM backend to use (ollama, llm-rs, openai)
                                     [default: ollama]
      --online                       Force online mode (use online models)
      --offline                      Force offline mode (never use online APIs)
      --debug                        Enable debug output
      --force                        Force execution without safety prompts
      --plugin <PLUGIN>              Specify plugin to use for command generation
      --model-path <MODEL_PATH>      Path to local LLM model for llm-rs backend
      --history-file <HISTORY_FILE>  Path to history file
      --no-feedback                  Disable feedback prompts
  -h, --help                         Print help
  -V, --version                      Print version
```

## Usage Examples

### Basic Usage
```powershell
# Process natural language input
cargo run -- "show me all files in the current directory"

# Use a specific plugin
cargo run -- --plugin git "commit all changes with message 'update documentation'"

# View command history
cargo run -- --history
```

### Advanced Usage
```powershell
# Use offline mode with a local model
cargo run -- --offline --backend llm-rs --model-path "models/tinyllama.gguf" "list running processes"

# Debug mode with forced execution
cargo run -- --debug --force "show disk space usage"

# List available plugins
cargo run -- --list-plugins

# Dry run mode to see what would be executed
cargo run -- --dry-run "find all log files larger than 10MB"
```

## Detailed Usage Guide

### Interactive Mode

When run without input arguments, the CLI will enter interactive mode, prompting you for input:

```powershell
cargo run
# Output: Enter your request: 
```

You can then type your natural language request and press Enter to process it.

### Plugin Selection

The Shell Assistant has specialized plugins for different domains:

#### Git Plugin
The Git plugin handles various Git-related operations:

```powershell
# View Git status
cargo run -- "show git status"

# Add files
cargo run -- "stage all changes"
cargo run -- "add the README.md file to git"

# Commit changes
cargo run -- "commit with message 'update documentation'"

# Branch operations
cargo run -- "create a new branch called feature-x"
cargo run -- "switch to main branch"

# Remote operations
cargo run -- "push changes to remote"
cargo run -- "pull latest changes"
```

#### Docker Plugin
The Docker plugin handles Docker container and image management:

```powershell
# List containers
cargo run -- "show all docker containers"
cargo run -- "list running containers"

# Image management
cargo run -- "list all docker images"
cargo run -- "pull nginx image"

# Container operations
cargo run -- "run redis container"
cargo run -- "stop container abcd1234"
```

### LLM Backend Selection

Shell Assistant supports multiple LLM backends:

#### Ollama (Default)
```powershell
# Use default Ollama backend with codellama model
cargo run -- "your request"

# Use Ollama with online mode (wizardcoder model)
cargo run -- --online "your request"
```

#### Local LLM via llm-rs
```powershell
# Use a local GGUF model
cargo run -- --backend llm-rs --model-path "path/to/model.gguf" "your request"
```

#### OpenAI (Experimental)
```powershell
# Note: This backend is currently disabled by default
cargo run -- --backend openai "your request"
```

### Command Safety

The Shell Assistant has built-in safety mechanisms to prevent accidental execution of dangerous commands:

```powershell
# This will prompt with additional safety warnings
cargo run -- "delete all files in this directory"
```

When a potentially dangerous command is detected, you'll see:
- A warning message explaining the risk
- A requirement for double confirmation
- An explanation of what makes the command risky

Use the `--force` flag to bypass safety prompts for trusted operations:

```powershell
cargo run -- --force "your request"
```

The safety checker monitors:
- Destructive commands like `rm`, `del`, `rmdir`, `format`, etc.
- PowerShell-specific dangerous cmdlets like `Remove-Item`, `Set-ExecutionPolicy`
- High-risk patterns like `-rf`, `-force`, `/s /q`, etc.
- File redirections that might overwrite files

### Debug Information

For troubleshooting or understanding how commands are generated, use the `--debug` flag:

```powershell
cargo run -- --debug "your request"
```

This will show:
- The CLI arguments being used
- The path to the history file
- The full prompt sent to the LLM
- Plugin selection information
- Other behind-the-scenes details

### Offline Use

For environments without internet access or for privacy:

```powershell
cargo run -- --offline "your request"
```

This ensures that:
- No online APIs are used
- Only local models are employed
- All processing stays on your machine

If you attempt to use an online-only backend with `--offline`, the system will automatically switch to a local alternative.

### History Management

View your command history with timestamps and feedback:

```powershell
cargo run -- --history
```

Each history entry includes:
- The original natural language input
- The generated command
- Timestamp
- Your feedback (helpful, not helpful, edited)
- Any explanations provided

The history is automatically saved to a JSON file in `~/.shell-assistant/history.json` (Linux/macOS) or `%USERPROFILE%\.shell-assistant\history.json` (Windows).

### Custom Configuration

Use your own configuration file or history location:

```powershell
# Custom configuration
cargo run -- --config "path/to/config.yaml" "your request"

# Custom history file
cargo run -- --history-file "path/to/history.json" "your request"
```

### Feedback System

After command execution, you'll be prompted for feedback unless disabled:

```powershell
# Disable feedback prompts
cargo run -- --no-feedback "your request"
```

Feedback options:
1. **Helpful** (👍): Marks the command as helpful in the history
2. **Not Helpful** (👎): Marks the command as not helpful
3. **Edit** (✏️): Allows you to provide a corrected version of the command
4. **Skip** (⏭️): Skip providing feedback

## Keyboard Shortcuts

When using the interactive selection menus:
- `↑`/`↓` arrows: Navigate options
- `Enter`: Select the highlighted option
- `q` or `Ctrl+C`: Quit/cancel operation

## Getting Started
To build and run the project, follow these steps:

1. Clone the repository:
   ```powershell
   git clone <repository-url>
   cd shell-assistant
   ```

2. Build the project:
   ```powershell
   cargo build
   ```

3. Run the CLI application:
   ```powershell
   cargo run -- "your natural language request here"
   ```

### Prerequisites
- Rust toolchain (1.65+)
- For local LLM support:
  - Ollama installed and running (for Ollama backend), or
  - GGUF model files for the llm-rs backend
- For Windows using llm-rs: Install LLVM from https://github.com/llvm/llvm-project/releases/

## Testing
The project includes several test scripts to verify functionality:

1. Basic usage examples are in `tests/test_script.md`.

2. To test the LLM backend functionality:
   - For Windows: Run `./tests/test_llm_backends.ps1`
   - For Linux/macOS: Run `./tests/test_llm_backends.sh`

3. A mock flow test example is available in `tests/mock_test.md`.

## Customization

### Adding New Plugins
1. Create a new implementation of the `Plugin` trait
2. Register your plugin in the `PluginManager` initialization
3. Handle specific domain commands in your plugin's `handle` method

Example plugin structure:
```rust
pub struct MyPlugin;

impl Plugin for MyPlugin {
    fn name(&self) -> &str {
        "my-plugin"
    }
    
    fn description(&self) -> &str {
        "A specialized plugin for handling specific operations"
    }
    
    fn can_handle(&self, input: &str) -> bool {
        // Logic to determine if this plugin can handle the input
        input.contains("keyword")
    }
    
    fn handle(&self, input: &str) -> Option<CommandResult> {
        // Logic to convert input to a shell command
        Some(CommandResult {
            command: "my-command".to_string(),
            explanation: "This command does...".to_string(),
            executed: false,
            output: None,
        })
    }
}
```

### Using Different LLM Models
- For Ollama:
  - Default: `codellama` (local)
  - Online mode: `wizardcoder` (requires download)
  - Custom: Change the model name in the codebase
- For llm-rs:
  - Provide different GGUF model files with the `--model-path` flag
  - Default: `models/tinyllama.gguf`

## Terminal Integration

Shell Assistant can be integrated into your existing terminal environment instead of running as a separate application. This allows you to use its functionality directly within your preferred terminal.

### Integration Methods

1. **PowerShell Module** (Windows):
   ```powershell
   cd powershell-module
   .\Install-ShellAssistant.ps1
   ```
   After installation, you can use:
   ```powershell
   sa "your request"  # Process a natural language request
   sa-history         # Show command history
   sa-plugins         # List available plugins
   ```

2. **Bash/Zsh Integration** (Linux/macOS):
   ```bash
   cd shell-integration
   ./install.sh
   source ~/.bashrc  # or ~/.zshrc
   ```
   After installation, you can use the same commands as in PowerShell.

3. **Terminal Emulator Configs**:
   - VS Code: See `terminal-configs/vscode-settings.json`
   - Windows Terminal: See `terminal-configs/windows-terminal.json`

See `TERMINAL_INTEGRATION.md` for detailed instructions on integrating Shell Assistant with your terminal environment.

## Contributing
Contributions are welcome! Please open an issue or submit a pull request for any enhancements or bug fixes.

## License
This project is licensed under the MIT License. See the LICENSE file for more details.