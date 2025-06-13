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
   Command: <command>
   Explanation: Lists all files with details in the current directory, including hidden files.
   
   [r] Run [c] Copy [a] Abort
   Choose an action:
   ```
5. Enter your choice:
   - `r` to run the command
   - `c` to copy the command to clipboard
   - `a` to abort

## Testing Different Actions

Try the different actions:

- `r` - The command will be executed and the output displayed
- `c` - The command will be copied to the clipboard
- `a` - The operation will be aborted

This mock flow validates the core functionality of the application before integrating with a real LLM.
