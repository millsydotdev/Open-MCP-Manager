# Contributing to Open MCP Manager

Thank you for your interest in contributing to Open MCP Manager! We welcome contributions from the community to help make this the best desktop manager for MCP servers.

## Getting Started

### Prerequisites

- **Rust**: Ensure you have the latest stable version of Rust installed.
- **Dioxus CLI**: Install with `cargo install dioxus-cli`.
- **Tailwind CSS**: Required for styling.

### Building the Project

1. Clone the repository:

   ```bash
   git clone https://github.com/millsydotdev/Open-MCP-Manager.git
   cd Open-MCP-Manager
   ```

2. Run the development server:

   ```bash
   dx serve
   ```

3. Run tests:

   ```bash
   cargo test
   ```

## Contributing Guidelines

### Reporting Bugs

Please use the **Issues** tab to report bugs. Include:

- A clear description of the issue.
- Steps to reproduce.
- Expected vs. actual behavior.
- Screenshots if applicable.
- Using the `bug` label.

### Feature Requests

Have an idea? Open an issue with the `feature-request` label. Describe:

- The problem you are solving.
- Your proposed solution.

### Pull Requests

1. Fork the repository.
2. Create a new branch for your feature (`git checkout -b feature/amazing-feature`).
3. Commit your changes.
4. Push to the branch.
5. Open a Pull Request.

Please ensure your code passes `cargo check` and `cargo fmt` before submitting.

## Style Guide

- We use standard Rust formatting (`cargo fmt`).
- Ensure no clippy warnings remain (`cargo clippy`).

## Community

Join our [Discussions](https://github.com/millsydotdev/Open-MCP-Manager/discussions) to talk about development, ideas, and more.
