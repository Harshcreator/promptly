# Terminal Emulator Configurations

This directory contains configuration templates for integrating Shell Assistant with various terminal emulators.

## Windows Terminal

1. Install the Shell Assistant PowerShell module:
   ```powershell
   cd ..\powershell-module
   .\Install-ShellAssistant.ps1
   ```

2. Open Windows Terminal settings (JSON)

3. Add the profile from `windows-terminal.json` to your profiles list

## Visual Studio Code

1. Install the Shell Assistant PowerShell module:
   ```powershell
   cd ..\powershell-module
   .\Install-ShellAssistant.ps1
   ```

2. Open VS Code settings (JSON)

3. Add the configuration from `vscode-settings.json` to your settings

## Other Terminal Emulators

For other terminal emulators like iTerm2, Alacritty, or Konsole, you can:

1. Install the appropriate shell integration (PowerShell module or bash/zsh script)
2. Configure your terminal to load the appropriate shell with the integration

For example, with iTerm2 on macOS:
1. Install the bash/zsh integration
2. Create a profile that runs: `bash --init-file ~/.bashrc`
