# Shell Assistant PowerShell Module

This PowerShell module integrates the Shell Assistant CLI tool directly into your PowerShell terminal, making it easy to translate natural language requests into shell commands without leaving your terminal.

## AI Backend Requirements

**Important**: Shell Assistant supports multiple AI backends - you can choose based on your needs:

### Option 1: Ollama (Recommended)
- **Setup Required**: Install Ollama locally
- **Installation**:
  ```powershell
  # Install Ollama
  winget install Ollama.Ollama
  
  # Pull the AI model
  ollama pull codellama
  ```
- **Usage**: `sa "your request"` (uses Ollama by default)

### Option 2: Local GGUF Models (Fully Offline)
- **Setup Required**: Download a GGUF model file
- **Installation**: 
  ```powershell
  # Create models directory and download a GGUF model
  mkdir models
  # Download tinyllama.gguf or similar to models/ folder
  ```
- **Usage**: `sa -Backend llm-rs -ModelPath "models/tinyllama.gguf" "your request"`

### Option 3: OpenAI API (Cloud-based)
- **Setup Required**: OpenAI API key
- **Installation**:
  ```powershell
  # Set your API key as environment variable
  $env:OPENAI_API_KEY = "your-api-key-here"
  ```
- **Usage**: `sa -Backend openai "your request"`

## Installation

1. Run the installation script:
   ```powershell
   .\Install-ShellAssistant.ps1
   ```

2. Verify the installation:
   ```powershell
   Get-Module -ListAvailable ShellAssistant
   ```

## Usage

### Basic Usage

```powershell
# Process a natural language request
sa "list all markdown files"

# View command history
sa-history

# List available plugins
sa-plugins
```

### Advanced Usage

```powershell
# Use with specific plugin
Invoke-ShellAssistant -Query "commit all changes" -Plugin git

# Dry run mode (don't execute commands)
Invoke-ShellAssistant -Query "delete all temporary files" -DryRun

# Force execution without safety prompts
Invoke-ShellAssistant -Query "show disk space" -Force

# Debug mode to see additional information
Invoke-ShellAssistant -Query "find large files" -DebugMode

# Use different backend with specific model
Invoke-ShellAssistant -Query "count words in text files" -Backend llm-rs -ModelPath "C:\path\to\model.gguf"

# Use online mode
Invoke-ShellAssistant -Query "complex regex search" -Online

# Use offline mode
Invoke-ShellAssistant -Query "simple file operations" -Offline
```

## Customization

You can modify the module by editing:
- `$env:USERPROFILE\Documents\WindowsPowerShell\Modules\ShellAssistant\ShellAssistant.psm1`
- `$env:USERPROFILE\Documents\WindowsPowerShell\Modules\ShellAssistant\ShellAssistant.psd1`

## Getting Help

```powershell
Get-Help Invoke-ShellAssistant -Detailed
```
