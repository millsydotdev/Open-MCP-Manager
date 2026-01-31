# Open MCP Manager

A clean, native desktop application for managing [Model Context Protocol (MCP)](https://modelcontextprotocol.io) servers, built with Rust and Dioxus.

![Open MCP Manager](public/window.svg)

## Features

- 🚀 **Server Management**: Add, edit, and manage local (stdio) and remote (SSE) MCP servers.
- 🛠️ **Interactive Inspector**: Inspect tools, resources, and prompts. Execute tools directly from the UI.
- 📦 **Registry**: Discover and install servers from the MCP registry.
- ⚙️ **Config Export**: Generate configurations for Claude Desktop.
- 🎨 **Modern UI**: Dark mode, animations, and a responsive design.

## Tech Stack

- **Frontend**: Dioxus (Rust)
- **Backend**: Rust (Tokio, Rusqlite)
- **Styling**: Tailwind CSS

## Prerequisites

- **Rust**: [Install Rust](https://www.rust-lang.org/tools/install)
- **Dioxus CLI**: `cargo install dioxus-cli`

## Usage

Run the application:

```bash
cargo run
```

Or with Dioxus CLI (recommended for hot reloading):

```bash
dx serve
```
