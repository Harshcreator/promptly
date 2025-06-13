# Shell Assistant

## Overview
The Shell Assistant is a Rust-based command-line interface (CLI) application that transforms natural language input into shell commands. It leverages a modular architecture with multiple crates to handle various aspects of the application, including command execution, prompt construction, and plugin management.

## Project Structure
The project is organized as a Cargo workspace with the following crates:

- **core**: Manages prompt construction and parsing of responses from a mock language model (LLM).
- **executor**: Executes shell commands with options for dry-run and user confirmation.
- **plugins**: Defines a plugin system with a sample plugin for Git.
- **storage**: Stores command history, either in memory or in a flat file.
- **cli**: The main binary crate that integrates all components and handles user input.

## Dependencies
The project utilizes several dependencies to facilitate its functionality:
- `reqwest`: For making HTTP requests (if needed for LLM interaction).
- `serde`: For serialization and deserialization of data.
- `tokio`: For asynchronous programming.
- `clap`: For command-line argument parsing.

## Features
- Accepts natural language input and converts it into shell commands.
- Displays explanations for the generated commands.
- Provides options to run, copy, or abort the command execution.
- Modular design allows for easy extension with additional plugins.

## Getting Started
To build and run the project, follow these steps:

1. Clone the repository:
   ```
   git clone <repository-url>
   cd shell-assistant
   ```

2. Build the project:
   ```
   cargo build
   ```

3. Run the CLI application:
   ```
   cargo run --workspace --bin cli
   ```

## Contributing
Contributions are welcome! Please open an issue or submit a pull request for any enhancements or bug fixes.

## License
This project is licensed under the MIT License. See the LICENSE file for more details.