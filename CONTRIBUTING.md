# Contributing to Open MCP Manager

First off, thank you for considering contributing to Open MCP Manager! It's people like you who make this tool better for everyone.

## 🛠️ Development Setup

### Prerequisites

- **Rust**: Latest stable version.
- **Dioxus CLI**: `cargo install dioxus-cli`.
- **Node.js & npm**: Required for our unified build scripts.

### Local Development Workflow

We use `npm` to orchestrate our Rust development tasks.

1. **Clone the Repository**

   ```bash
   git clone https://github.com/millsydotdev/Open-MCP-Manager.git
   cd Open-MCP-Manager
   ```

2. **Install Dependencies**

   ```bash
   npm install
   ```

3. **Spin up the Dev Server**

   ```bash
   npm run dev
   ```

4. **Run Quality Checks**
   Before submitting a PR, please ensure all checks pass:

   ```bash
   npm run lint
   npm run test
   npm run check
   ```

## 🤝 How to Contribute

### Reporting Issues

Use the [GitHub Issues](https://github.com/millsydotdev/Open-MCP-Manager/issues) to report bugs or request features. Please provide as much context as possible.

### Pull Requests

1. Fork the repo and create your branch from `main`.
2. If you've added code that should be tested, add tests.
3. Ensure the test suite passes.
4. Issue that pull request!

## 📜 Code of Conduct

We expect all contributors to adhere to our [Code of Conduct](CODE_OF_CONDUCT.md).

## 💬 Community

Questions? Feature ideas? Join the conversation in [GitHub Discussions](https://github.com/millsydotdev/Open-MCP-Manager/discussions).

---

*Happy Coding!*
