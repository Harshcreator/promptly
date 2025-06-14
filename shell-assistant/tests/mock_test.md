# Shell Assistant - Mock Flow Test

This script demonstrates the end-to-end mock flow of the shell assistant.

## Test the Application

Run the following command to test the application with the mock flow:

```bash
cargo run --bin cli -- "list files"
```

## Expected Flow

1. You enter a natural language request (e.g., "list files")
2. The system processes your request 
3. The mock LLM returns a fixed response:
   - On Windows: `Get-ChildItem -Force`
   - On Unix: `ls -la`
4. The system displays:
   ```
   Shell Command Assistant
   ----------------------
   Your request: list files
   
   Translated command: ls -la (or Get-ChildItem -Force on Windows)
   
   Command output:
   [Output of the command]
   ```

## Testing with Different Backends

The mock flow can be tested with different backends:

```bash
# Test with default (Ollama) backend
cargo run --bin cli -- "list files"

# Test with llm-rs backend
cargo run --bin cli -- --backend llm-rs "list files"

# Test with online mode
cargo run --bin cli -- --online "list files"
```

For more comprehensive backend testing, use the test scripts:
- `tests/test_llm_backends.ps1` (PowerShell)
- `tests/test_llm_backends.sh` (Bash)
