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

## Installation

Releases are available for Windows, macOS, and Linux. This application is currently **unsigned**, so you may need to bypass security warnings to run it.

### Windows

1. Download `open-mcp-manager-windows-amd64.exe` from the [Releases](https://github.com/millsydotdev/Open-MCP-Manager/releases) page.
2. Run the executable.
3. If you see "Windows protected your PC", click **More info** -> **Run anyway**.

### macOS

1. Download `open-mcp-manager-macos-intel.tar.gz`.
2. Extract the `.app` file.
3. If you see "App cannot be opened because it is from an unidentified developer":
    - Right-click (Control-click) the app in Finder.
    - Select **Open**.
    - Click **Open** in the dialog box.

### Linux

1. Download `open-mcp-manager-linux-amd64.deb`.
2. Install with `sudo dpkg -i open-mcp-manager-linux-amd64.deb`.

## Usage

Run the application:

```bash
cargo run
```

Or with Dioxus CLI (recommended for hot reloading):

```bash
dx serve
```
