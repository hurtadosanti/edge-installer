# Edge Installer

Edge Installer is a terminal-based installation wizard built in Rust using asynchronous functionality. It allows users
to select disks and images for installation, and performs an installation process based on the user's choices. The
application also includes a timeout mechanism for prompts to ensure smooth execution even when no input is provided.

## Features

- Asynchronous terminal user interface (TUI) for user interaction.
- Disk and image selection via prompts.
- Timeout for prompts with default responses to avoid blocking.
- Installation setup logic and graceful shutdown.

## Requirements

- Rust (latest stable version)
- [Tokio](https://tokio.rs) for asynchronous runtime
- [Dialoguer](https://docs.rs/dialoguer/latest/dialoguer/) for interactive prompts

## Installation

1. Clone the repository:

    ```bash
    git clone https://github.com/hurtadosanti/edge-installer.git
    ```

2. Navigate to the project directory:

    ```bash
    cd edge-installer
    ```

3. Build the project:

    ```bash
    cargo build --release
    ```

4. Run the application:

    ```bash
    cargo run --release
    ```

## Usage

When you run Edge Installer, it will guide you through a simple installation process:

1. **Disk Selection**: Choose the disk where you want to install.
2. **Image Selection**: Select an OS image to install from the given options.
3. **Shutdown**: At the end of the installation, you'll be prompted to shut down the system.

If no input is provided within 2 seconds, default choices will be made automatically to keep the process running
smoothly.

## Example Output

```bash
Welcome to the installation wizard.
Do you want to start the installation with default values? (y/n)
> [y] 

Using default installation configuration.
Selected Disk: Disk 1: /dev/sda
Selected Image: Image 1: Ubuntu 20.04

Installation started...
Installation completed.
Do you want to shutdown the system? (y/n)
> [n] 

Shutdown aborted.
Installation was finished successfully.
```

## Contributing

Contributions are welcome! Please fork the repository and submit a pull request with your changes.

## License

This project is licensed under the MIT License. See the LICENSE file for more details.

### Notes:

- Replace `https://github.com/yourusername/edge-installer.git` with the actual repository URL.
- You can customize the **Usage** and **Example Output** sections based on how your application evolves.
