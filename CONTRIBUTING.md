# Contributing to Open MCP Manager

👋 **Welcome!** We are thrilled that you're interested in contributing to Open MCP Manager. It's people like you who help keep the open-source ecosystem alive and thriving.

Whether you're fixing a typo, updating docs, or building the next big feature, we appreciate your help!

## 🗺️ Project Structure

To help you find your way around, here's a quick overview of the codebase:

- **`src/`**: The heart of the application (Rust).
  - **`components/`**: Reusable Dioxus UI components (e.g., `NavBar`, `ServerList`).
  - **`app.rs`**: The main entry point and routing logic.
  - **`state.rs`**: Global application state management.
  - **`db.rs`**: Database interactions and persistence.
- **`public/`**: Static assets like images, icons, and global CSS.
- **`scripts/`**: Helper scripts for build and maintenance tasks.
- **`.github/`**: CI/CD workflows and issue templates.

## 🛠️ Development Setup

We strive to make the developer experience as smooth as possible. We use `npm` to standardize commands across platforms.

### Prerequisites

1. **Rust**: Make sure you have the latest stable Rust installed.
2. **Dioxus CLI**: Install it with `cargo install dioxus-cli`.
3. **Node.js**: Required for our build scripts.

### Workflow

1. **Clone & Install**:

    ```bash
    git clone https://github.com/millsydotdev/Open-MCP-Manager.git
    cd Open-MCP-Manager
    npm install
    ```

2. **Run Development Server**:

    ```bash
    npm run dev
    ```

    This will start the app with hot-reloading enabled.

3. **Check Your Work**:
    Before submitting a PR, please run:

    ```bash
    npm run lint   # Checks for code style issues
    npm run test   # Runs unit tests
    npm run check  # Verifies compilation
    ```

## 🤝 How to Contribute

### Reporting Issues

Found a bug or have a feature request? Please use [GitHub Issues](https://github.com/millsydotdev/Open-MCP-Manager/issues). Provide as much detail as possible—screenshots, logs, and reproduction steps are super helpful!

### Submitting Pull Requests

1. **Fork** the repository.
2. **Create a branch** for your feature or fix (`git checkout -b feature/amazing-feature`).
3. **Commit** your changes with clear messages used [Conventional Commits](https://www.conventionalcommits.org/) if possible.
4. **Push** to your fork.
5. **Open a Pull Request** against our `main` branch.

We'll review your PR as soon as possible and work with you to get it merged!

## 📜 Code of Conduct

We are committed to providing a friendly, safe, and welcoming environment for all. Please review our [Code of Conduct](CODE_OF_CONDUCT.md).

---

**Happy Hacking!** 🚀
