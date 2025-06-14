# Shell Assistant - Test Script

This script shows how to use the shell assistant with natural language commands.

## Examples

Try running these commands:

```bash
# Run with direct input
cargo run --bin cli -- "list files in current directory"

# Run with interactive input
cargo run --bin cli

# Show command history
cargo run --bin cli -- --history

# Use dry-run mode (doesn't execute commands)
cargo run --bin cli -- --dry-run "create a new directory called test_folder"

# Run with explicit Ollama backend
cargo run --bin cli -- --backend ollama "list files"

# Run with llm-rs backend (requires llm-rs feature)
cargo run --bin cli -- --backend llm-rs "list files"

# Run with online mode (wizardcoder model)
cargo run --bin cli -- --online "list files"

# Run with a specific model path (for llm-rs backend)
cargo run --bin cli -- --backend llm-rs --model-path "/path/to/model.gguf" "list files"
```

## Testing Backend Functionality

For testing the LLM backend functionality, use the test scripts in this directory:

- Windows: `./test_llm_backends.ps1`
- Linux/macOS: `./test_llm_backends.sh`

These scripts will test all available backends and fallback logic.
