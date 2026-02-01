# <img src="public/window.svg" width="32" height="32" valign="middle" /> Open MCP Manager

<p align="left">
  <a href="https://github.com/millsydotdev/Open-MCP-Manager/releases"><img src="https://img.shields.io/github/v/release/millsydotdev/Open-MCP-Manager?style=flat-square&color=BC1823" alt="Release"></a>
  <a href="https://github.com/millsydotdev/Open-MCP-Manager/blob/main/LICENSE"><img src="https://img.shields.io/github/license/millsydotdev/Open-MCP-Manager?style=flat-square&color=BC1823" alt="License"></a>
  <img src="https://img.shields.io/badge/rust-1.75+-blue?style=flat-square&logo=rust&color=BC1823" alt="Rust Version">
</p>

A premium, native desktop application for managing [Model Context Protocol (MCP)](https://modelcontextprotocol.io) servers. Built with **Rust** and **Dioxus** for maximum performance and a smooth developer experience.

---

## 🚀 Key Features

- **Intuitive Management** — Add, edit, and orchestrate local (stdio) and remote (SSE) MCP servers with ease.
- **Deep Inspector** — Real-time inspection of tools, resources, and prompts. Test commands directly within the UI.
- **Smart Registry** — Instantly discover and install community-verified servers from the integrated MCP registry.
- **Claude Integration** — One-click configuration generation for Claude Desktop and other compatible clients.

* **Modern Aesthetic** — Sleek red/black dark mode, powered by custom CSS and smooth animations.

---

## 🛠️ Unified Developer Workflow

We have unified the build process. You can now use standard `npm` commands to manage your development environment.

| Command | Description |
| :--- | :--- |
| `npm run dev` | Launch the desktop app in development mode with hot-reload. |
| `npm run build` | Compile the optimized production bundle. |
| `npm run check` | Run Rust compiler checks. |
| `npm run test` | Execute the unit test suite. |
| `npm run lint` | Run Clippy for code quality. |

---

## 📦 Installation & Setup

### Prerequisites

- **Rust**: [Install via rustup](https://www.rust-lang.org/tools/install)
- **Dioxus CLI**: `cargo install dioxus-cli`
- **Node.js**: Required for the unified build scripts.

### Quick Start

1. **Clone & Install Dependencies**

   ```bash
   git clone https://github.com/millsydotdev/Open-MCP-Manager.git
   cd Open-MCP-Manager
   npm install
   ```

2. **Run Development Mode**

   ```bash
   npm run dev
   ```

### Releases

Download signed (optional) binaries for your platform from the [Releases](https://github.com/millsydotdev/Open-MCP-Manager/releases) page.

> [!NOTE]
> On **Windows**, you may need to click "More Info" -> "Run Anyway" as the binary is currently unsigned.
> On **macOS**, right-click the `.app` and select "Open" to bypass security gatekeeper for the first run.

---

## 🤝 Contributing

Contributions are what make the open source community such an amazing place to learn, inspire, and create. Any contributions you make are **greatly appreciated**.

See [CONTRIBUTING.md](CONTRIBUTING.md) for details on our code of conduct and the process for submitting pull requests.

## 📄 License

Distributed under the **ISC License**. See `LICENSE` for more information.

---

<p align="center">
  Built with ❤️ by <a href="https://github.com/millsydotdev">Millsy</a>
</p>
