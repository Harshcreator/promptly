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
```

## Expected Output

When you run the assistant, it will:
1. Process your natural language request
2. Generate a shell command
3. Provide an explanation
4. Ask if you want to run, copy, or abort the command
5. Execute the command if you choose to run it

## Features Used

This demonstrates:
- Natural language processing
- Command generation
- User confirmation
- Command execution
- History tracking
