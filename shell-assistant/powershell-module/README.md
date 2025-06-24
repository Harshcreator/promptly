# Shell Assistant PowerShell Module

This PowerShell module integrates the Shell Assistant CLI tool directly into your PowerShell terminal, making it easy to translate natural language requests into shell commands without leaving your terminal.

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
