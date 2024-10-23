# Kaisen_Yao_IDS706_Week7 - Rust Command-Line Tool

[![CI](https://github.com/nogibjj/Kaisen_Yao_IDS706_Week8/actions/workflows/CI.yml/badge.svg)](https://github.com/nogibjj/Kaisen_Yao_IDS706_Week8/actions/workflows/CI.yml)

## Overview

This project is a Rust-based command-line tool. The tool reads user input and provides feedback via the command line. It includes a functional CI/CD pipeline using GitHub Actions and an optional Docker container setup for development and deployment.

### Key Features
- **Rust-based CLI**: Written in Rust for high performance and safety.
- **CI/CD Pipeline**: Automated builds, testing, formatting, and releases with GitHub Actions.
- **Devcontainer**: Containerized development environment for consistent workflows in VSCode.
- **Packaged Binary**: Built and distributed using a release build in CI/CD.

### Project Requirements Met:
1. **Packaged Tool**: The command-line tool is built using Rust and packaged as a binary.
2. **User Guide**: Detailed instructions on installation and usage are provided below.
3. **Database Communication**: [Optional for Rust, as per project requirements].
4. **CI/CD Pipeline**: A complete CI/CD setup using GitHub Actions is included to automate builds, testing, and releases.
5. **README.md**: Complete documentation is included with detailed instructions.

## Tool Functionality

This Rust project provides a simple command-line tool that prints back the input argument passed by the user. It uses the `clap` crate for parsing arguments and follows the typical Rust project structure.

### Example Usage

To use the tool, you can run the following command (after building the binary):

```bash
comment_tool "This is a test comment"
```

The output will be:

```bash
This is a test comment
```

### Build and Run Instructions

#### Prerequisites

- **Rust toolchain**: Install Rust using [rustup](https://rustup.rs/).
- **Docker**: Optional for building and running in a containerized environment.
- **VSCode with Remote Containers**: Optional for using the devcontainer setup.

#### Building the Project Locally

To build the project, navigate to the project directory and run the following commands:

1. Clone the repository:

    ```bash
    git clone https://github.com/nogibjj/Kaisen_Yao_IDS706_Week8.git
    cd Kaisen_Yao_IDS706_Week8
    ```

2. Build the project in release mode:

    ```bash
    cargo build --release
    ```

3. Run the tool:

    ```bash
    ./target/release/add "Hello, world!"
    ```

    Expected output:

    ```bash
    Hello, world!
    ```

#### Running in a Docker Container

To run the project inside Docker:

1. Build the Docker image:

    ```bash
    docker build -t rust_project .
    ```

2. Run the container:

    ```bash
    docker run -it rust_project
    ```

This will build and run the project inside a container.

## Devcontainer Setup

If you're using Visual Studio Code with the Remote Containers extension:

1. Open the project in VSCode.
2. Press `F1` and select **Remote-Containers: Reopen in Container**.
3. The development environment will automatically be set up inside the container.

## CI/CD Pipeline

This project uses GitHub Actions for CI/CD, which automates the following tasks:

1. **Build**: Compiles the project in release mode.
2. **Test**: Runs all the unit tests in the project.
3. **Format**: Checks that all Rust code is formatted according to `cargo fmt`.
4. **Check**: Runs `cargo check` to verify that the code compiles without errors.

The CI pipeline is triggered on every push and pull request to the `main` branch. You can view the status of the CI pipeline using the badge at the top of this `README.md`.

### Binary Artifact

The binary built during the `release` step is automatically uploaded as an artifact by the CI pipeline. The binary can be downloaded from the GitHub Actions page under the respective workflow run.

## Code Formatting and Testing

- **Format**: You can format the code locally using:

    ```bash
    cargo fmt --all
    ```

- **Test**: Run tests using:

    ```bash
    cargo test
    ```

## Contribution

Feel free to fork this repository, make improvements, and open a pull request. Contributions are always welcome!

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for more details.

## Reference
https://github.com/johncoogan53/Rust_SQLite/tree/main
