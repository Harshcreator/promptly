#!/bin/bash
# shell-assistant.sh - Shell integration for Shell Assistant

# Path to the Shell Assistant executable
# The installation script will replace this with the correct path
SHELL_ASSISTANT_PATH="$HOME/.local/bin/shell-assistant"

# Main function to invoke Shell Assistant
function sa() {
  local args=()
  local query=""
  local in_query=false
  
  # Process arguments
  for arg in "$@"; do
    if [[ "$arg" == "--"* ]]; then
      # It's an option
      args+=("$arg")
      in_query=false
    elif [[ "$in_query" == true ]]; then
      # Append to existing query with space
      query="$query $arg"
    else
      # Start a new query
      query="$arg"
      in_query=true
    fi
  done
  
  # If query is not empty, add it as the last argument
  if [[ -n "$query" ]]; then
    args+=("$query")
  fi
  
  # Execute Shell Assistant with all arguments
  "$SHELL_ASSISTANT_PATH" "${args[@]}"
}

# Show command history
function sa-history() {
  "$SHELL_ASSISTANT_PATH" --history
}

# List plugins
function sa-plugins() {
  "$SHELL_ASSISTANT_PATH" --list-plugins
}

# Export the functions
export -f sa
export -f sa-history
export -f sa-plugins
