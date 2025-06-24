#!/bin/bash
# install.sh - Installer for Shell Assistant shell integration

# Detect shell type
SHELL_TYPE=$(basename "$SHELL")
SHELL_RC=""
case "$SHELL_TYPE" in
  bash)
    SHELL_RC="$HOME/.bashrc"
    ;;
  zsh)
    SHELL_RC="$HOME/.zshrc"
    ;;
  *)
    echo "Unsupported shell: $SHELL_TYPE"
    echo "Only bash and zsh are supported at this time."
    exit 1
    ;;
esac

# Copy the Shell Assistant binary to a location in PATH
mkdir -p "$HOME/.local/bin"
cp "$(dirname "$0")/../target/release/cli" "$HOME/.local/bin/shell-assistant"
chmod +x "$HOME/.local/bin/shell-assistant"

# Copy the shell integration script
mkdir -p "$HOME/.local/share/shell-assistant"
cp "$(dirname "$0")/shell-assistant.sh" "$HOME/.local/share/shell-assistant/"

# Add source line to shell rc file
if ! grep -q "source.*shell-assistant.sh" "$SHELL_RC"; then
  echo "" >> "$SHELL_RC"
  echo "# Shell Assistant Integration" >> "$SHELL_RC"
  echo "source \"$HOME/.local/share/shell-assistant/shell-assistant.sh\"" >> "$SHELL_RC"
fi

echo "Shell Assistant integration installed successfully!"
echo "Please restart your shell or run: source $SHELL_RC"
echo ""
echo "You can now use the following commands:"
echo "  sa \"your request\"      - Process a natural language request"
echo "  sa-history              - Show command history"
echo "  sa-plugins              - List available plugins"
