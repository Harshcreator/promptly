# Shell Assistant Terminal Integration

This document outlines how to integrate Shell Assistant into your existing terminal, rather than running it as a separate application.

## Integration Options

We've provided several ways to integrate Shell Assistant into your existing terminal environment:

1. **PowerShell Module**: For Windows PowerShell and PowerShell Core users
2. **Bash/Zsh Integration**: For Linux and macOS users
3. **Terminal Emulator Configs**: For VS Code, Windows Terminal, and other terminal emulators

## Quick Installation Guide

### Windows (PowerShell)

```powershell
# Navigate to the powershell-module directory
cd powershell-module

# Install the PowerShell module
.\Install-ShellAssistant.ps1

# Test it out
sa "list all markdown files"
```

### Linux/macOS (Bash/Zsh)

```bash
# Build the CLI in release mode
cargo build --release

# Navigate to the shell-integration directory
cd shell-integration

# Install the shell integration
./install.sh

# Restart your shell or source your rc file
source ~/.bashrc  # or ~/.zshrc

# Test it out
sa "list all markdown files"
```

## Terminal-Specific Integration

### VS Code

See `terminal-configs/vscode-settings.json` for VS Code settings.

### Windows Terminal

See `terminal-configs/windows-terminal.json` for Windows Terminal profile settings.

## Usage After Integration

Once integrated, you can use Shell Assistant from your terminal with:

```
sa "your natural language request"
```

For example:

```
sa "find all text files larger than 1MB"
```

Other commands:

```
sa-history     # Show command history
sa-plugins     # List available plugins
```

For advanced options, use the full command syntax:

```
sa --plugin git "commit changes with message 'update docs'"
sa --dry-run "delete temporary files"
sa --force "show disk usage"
```

## Detailed Documentation

For more detailed documentation on each integration method:

- PowerShell Module: See `powershell-module/README.md`
- Bash/Zsh Integration: See `shell-integration/README.md`
- Terminal Configs: See `terminal-configs/README.md`
