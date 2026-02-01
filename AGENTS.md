## Project Summary

A native desktop Model Context Protocol (MCP) manager. It allows users to manage, configure, and monitor MCP servers via a centralized Rust-powered dashboard. Features include a dynamic server registry, real-time inspector, and configuration export for Claude Desktop.

## Tech Stack

- **Framework**: Dioxus 0.6+ (Rust Desktop)
- **Language**: Rust 2021 Edition
- **Database**: SQLite (via `rusqlite` and `r2d2`)
- **Async Runtime**: Tokio
- **Styling**: Vanilla CSS (Premium Dark Theme)
- **Serialization**: Serde (JSON/TOML)

## Architecture

- `src/main.rs`: Entry point and desktop window configuration.
- `src/app.rs`: Main application logic and routing.
- `src/state.rs`: Global application state and async task management.
- `src/process.rs`: Logic for spawning and managing MCP server processes (stdio/sse).
- `src/db.rs`: Database schema and persistence layer for servers and registry cache.
- `src/models.rs`: Shared data structures for MCP protocol and internal state.
- `src/components/`: Reusable UI components (Explorer, ConfigViewer, ServerConsole, etc.).

## Project Patterns

- **Process Management**: Servers are spawned as child processes. Stdio is handled via async pipes.
- **State Synchronization**: Uses Dioxus signals and global state for reactive UI updates.
- **Registry**: Fetches and caches community/official MCP servers from GitHub, NPM, and PyPI.
- **Unified Build**: Use `npm run dev|build|check|test` to manage the lifecycle.

## Development Guidelines

- Always ensure `npm run check` passes before committing.
- Version synchronization between `package.json` and `Cargo.toml` is handled by `npm version`.
- Keep the UI premium with consistent HSL color tokens in `public/style.css`.
