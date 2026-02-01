//! Open MCP Manager - A unified desktop manager for Model Context Protocol servers
//!
//! This library provides the core functionality for managing MCP servers,
//! including process management, database operations, and data models.

#![allow(non_snake_case)]

// Core modules
pub mod db;
pub mod models;
pub mod process;
pub mod state;

// UI components (keep private to the crate)
pub mod app;
pub(crate) mod components;

// Re-exports for convenience
pub use db::Database;
pub use models::{AppError, AppResult, CreateServerArgs, McpServer, UpdateServerArgs};
pub use process::{McpProcess, ProcessLog};
