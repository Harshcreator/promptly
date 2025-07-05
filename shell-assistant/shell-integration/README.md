# Shell Assistant Terminal Integration

This directory contains scripts to integrate the Shell Assistant CLI with various terminal environments.

## AI Backend Requirements

**Choose Your AI Backend**: Shell Assistant works with multiple AI backends - no single requirement:

### Option 1: Ollama (Recommended)
- **Setup**: Install Ollama locally and pull a model
  ```bash
  # Install Ollama (method varies by OS)
  # Linux: curl -fsSL https://ollama.com/install.sh | sh
  # macOS: brew install ollama
  
  # Pull the AI model
  ollama pull codellama
  ```

### Option 2: Local GGUF Models (Fully Offline)
- **Setup**: Download a GGUF model file
  ```bash
  # Create models directory and download a model
  mkdir models
  # Download tinyllama.gguf or similar to models/ folder
  ```
- **Usage**: `sa --backend llm-rs --model-path "models/tinyllama.gguf" "your request"`

### Option 3: OpenAI API (Cloud-based)
- **Setup**: Get an OpenAI API key
  ```bash
  # Set your API key
  export OPENAI_API_KEY="your-api-key-here"
  ```
- **Usage**: `sa --backend openai "your request"`

## PowerShell Integration

For Windows PowerShell and PowerShell Core users, see the `../powershell-module` directory.

## Bash/Zsh Integration

For Linux and macOS users, we provide a shell script that can be sourced in your `.bashrc` or `.zshrc`.

### Installation

1. Build the Shell Assistant CLI:
   ```bash
   cd ../
   cargo build --release
   ```

2. Run the installation script:
   ```bash
   ./install.sh
   ```

3. Restart your shell or source your rc file:
   ```bash
   source ~/.bashrc  # or ~/.zshrc
   ```

### Usage

Once installed, you can use the following commands:

```bash
# Process a natural language request
sa "list all markdown files"

# View command history
sa-history

# List available plugins
sa-plugins

# Use with specific plugin
sa --plugin git "commit all changes"

# Dry run mode (don't execute commands)
sa --dry-run "delete all temporary files"

# Force execution without safety prompts
sa --force "show disk space"

# Debug mode to see additional information
sa --debug "find large files"
```

## VS Code Terminal Integration

To use Shell Assistant within VS Code's integrated terminal:

1. Install the shell integration as described above
2. Open VS Code settings (JSON)
3. Add the following to your settings:

```json
"terminal.integrated.profiles.linux": {
  "bash-with-shell-assistant": {
    "path": "bash",
    "args": ["--init-file", "~/.bashrc"],
    "icon": "terminal-bash"
  }
},
"terminal.integrated.defaultProfile.linux": "bash-with-shell-assistant"
```

Replace `linux` with `osx` for macOS or `windows` for Windows (using PowerShell).

## Windows Terminal Integration

For Windows Terminal users:

1. Install the PowerShell module as described in `../powershell-module`
2. Open Windows Terminal settings
3. Add the following to your PowerShell profile:

```json
"commandline": "pwsh.exe -NoExit -Command \"Import-Module ShellAssistant\""
```
