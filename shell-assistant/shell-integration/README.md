# Shell Assistant Terminal Integration

This directory contains scripts to integrate the Shell Assistant CLI with various terminal environments.

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
