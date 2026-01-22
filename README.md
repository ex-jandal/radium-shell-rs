# Radium-Shell-rs

A specialized terminal-based command manager and execution environment built in Rust.

## Description
Radium-Shell-rs provides a robust and efficient way to manage and execute commands directly from your terminal. Designed with performance and extensibility in mind, it leverages Rust's capabilities to offer a responsive and reliable user experience.

## Features
- **Command Management**: Organize and store frequently used commands.
- **Efficient Execution**: Run commands quickly within a custom environment.
- **Terminal Integration**: Seamless interaction within your existing terminal setup.
- **Built in Rust**: High performance, memory safety, and concurrency.

## Getting Started

### Prerequisites
To build and run Radium-Shell-rs, you need:
- Rust (2021 edition or newer)
- Cargo (Rust's package manager, installed with Rust)

You can install Rust by following the instructions on the [official Rust website](https://www.rust-lang.org/tools/install).

### Building
Navigate to the project's root directory and run:

```bash
cargo build --release
```

This will compile the project and create an optimized executable in the `target/release/` directory.

### Running
After building, you can run the application:

```bash
./target/release/radium-shell-rs
```

## Usage
After running the application, you will be prompted to enter a master password to log in.

Once logged in, you can use the following built-in commands:

- `help`: Lists all registered commands.
- `reg <name> <command_string> [mode]`: Registers a new command. 
    - `mode` can be `fg` for interactive (foreground) or `bg` for captured (background). Defaults to `bg`.
    - Example: `reg list ls -la bg`
- `del <name>`: Deletes a registered command.
- `clear`: Clears the command history from the view.

### Executing Commands
- To run a registered command, simply type its name and press Enter.
- To run any system command with root privileges, prefix it with `!` for captured output or `!!` for interactive execution.
    - Example (captured): `!apt update`
    - Example (interactive): `!!vim /etc/hosts`

### Navigation
- Use the **Up** and **Down** arrow keys to navigate your command history.
- Use **Page Up** and **Page Down** to scroll through the output history.
- Use **Tab** for command auto-completion.

## Contributing
Contributions are welcome! Please feel free to open an issue or submit a pull request.

## License
This project is licensed under the GPL-3.0 License - see the [LICENSE](LICENSE) file for details.
