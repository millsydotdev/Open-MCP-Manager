# <img src="public/window.svg" width="32" height="32" valign="middle" /> Open MCP Manager

[![Release](https://img.shields.io/github/v/release/millsydotdev/Open-MCP-Manager?include_prereleases&label=release)](https://github.com/millsydotdev/Open-MCP-Manager/releases)
[![License](https://img.shields.io/github/license/millsydotdev/Open-MCP-Manager)](LICENSE)
[![Build](https://img.shields.io/github/actions/workflow/status/millsydotdev/Open-MCP-Manager/ci.yml?branch=main)](https://github.com/millsydotdev/Open-MCP-Manager/actions)
[![Rust](https://img.shields.io/badge/rust-1.75+-orange?logo=rust)](https://www.rust-lang.org/)

A **native, open-source desktop application** for managing [Model Context Protocol (MCP)](https://modelcontextprotocol.io) servers. Built with **Rust** and **Dioxus** for maximum performance, security, and a seamless developer experience.

---

## 📖 Table of Contents

- [Key Features](#-key-features)
- [Unified Developer Workflow](#-unified-developer-workflow)
- [Installation & Setup](#-installation--setup)
- [Troubleshooting](#-troubleshooting)
- [Contributing](#-contributing)
- [License](#-license)

---

## 🚀 Key Features

- **🔍 Deep Inspector**
  - Real-time inspection of tools, resources, and prompts.
  - Test server capabilities directly within the UI without needing a separate client.
  - View raw JSON payloads for debugging.

- **📦 Smart Registry**
  - Integrated MCP registry browser.
  - One-click installation of community-verified servers.
  - Automatic dependency resolution for stdio servers.

- **⚙️ Intuitive Management**
  - Edit configuration for local (stdio) and remote (SSE) servers.
  - Manage environment variables and command arguments via a clean UI.
  - Toggle servers on/off instantly.

- **🔌 Client Integration**
  - Generate configurations for Claude Desktop, Windsurf, and other MCP clients.
  - Seamlessly sync your managed servers with your favorite AI coding assistants.

- **✨ Polished UX**
  - Modern, responsive interface with a sleek dark mode.
  - Built with accessibility and performance in mind.

---

## 🛠️ Unified Developer Workflow

We have streamlined the development process using standard `npm` commands, backed by Rust's powerful toolchain.

| Command | Description |
| :--- | :--- |
| `npm run dev` | **Start here!** Launches the app in development mode with hot-reloading. |
| `npm run build` | Compiles an optimized release binary for your platform. |
| `npm run check` | Runs `cargo check` to catch type errors quickly. |
| `npm run test` | Executes the full unit test suite. |
| `npm run lint` | Runs `cargo clippy` to ensure code quality and idiomatic Rust. |

---

## 📦 Installation & Setup

### Prerequisites

- **Rust**: [Install via rustup](https://www.rust-lang.org/tools/install) (ensure you have stable toolchain).
- **Node.js**: Required for the unified build scripts and TailwindCSS processing.
- **Dioxus CLI**: *Optional*. The scripts will attempt to install this automatically if missing. You can also install it manually via `cargo install dioxus-cli`.

### Quick Start (Source)

1. **Clone the Repository**

    ```bash
    git clone https://github.com/millsydotdev/Open-MCP-Manager.git
    cd Open-MCP-Manager
    ```

2. **Install Dependencies**

    ```bash
    npm install
    ```

3. **Run Development Mode**

    ```bash
    npm run dev
    ```

### 📥 Binary Releases

Prefer not to build from source? Download the latest signed binaries for Windows and macOS from the [Releases](https://github.com/millsydotdev/Open-MCP-Manager/releases) page.

> [!NOTE]
> On **Windows**, you may need to click "More Info" -> "Run Anyway" if SmartScreen alerts appear.
> On **macOS**, you may need to right-click the app and select "Open" on the first run.

---

## 🔧 Troubleshooting

### Common Issues

<details>
<summary><strong>Command 'dx' not found</strong></summary>

The development server uses the Dioxus CLI. Ensure you have installed it and it is in your PATH:

```bash
cargo install dioxus-cli
```

On Windows, you might need to restart your terminal after installation.
</details>

<details>
<summary><strong>Build fails with "linker not found"</strong></summary>

Ensure you have the build tools for your platform installed (Visual Studio Build Tools on Windows, Xcode Command Line Tools on macOS).
</details>

<details>
<summary><strong>UI styles look broken or missing</strong></summary>

Ensure you ran `npm install` so that TailwindCSS can generate the styles. `npm run dev` handles the CSS generation automatically.
</details>

---

## 🤝 Contributing

We welcome contributions of all kinds! Whether it's fixing a bug, improving the docs, or adding a new feature.

See [CONTRIBUTING.md](CONTRIBUTING.md) for details on our code of conduct and how to get started.

## 📄 License

Distributed under the **MIT License**. See [LICENSE](LICENSE) for more information.

---

<p align="center">
  Built with ❤️ by <a href="https://github.com/millsydotdev">Millsy</a>
</p>
