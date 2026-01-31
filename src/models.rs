use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug, Serialize)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(String),
    #[error("IO error: {0}")]
    Io(String),
    #[error("Serialization error: {0}")]
    Serialization(String),
    // Validation(String),
    // Process(String),
}

impl From<rusqlite::Error> for AppError {
    fn from(err: rusqlite::Error) -> Self {
        AppError::Database(err.to_string())
    }
}

impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        AppError::Serialization(err.to_string())
    }
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::Io(err.to_string())
    }
}

pub type AppResult<T> = Result<T, AppError>;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)] // Added PartialEq for Dioxus props
pub struct McpServer {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub server_type: String, // 'stdio' or 'sse'
    pub command: Option<String>,
    pub args: Option<Vec<String>>,
    pub url: Option<String>,
    pub env: Option<std::collections::HashMap<String, String>>,
    pub description: Option<String>,
    pub is_active: bool,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct CreateServerArgs {
    pub name: String,
    #[serde(rename = "type")]
    pub server_type: String,
    pub command: Option<String>,
    pub args: Option<Vec<String>>,
    pub url: Option<String>,
    pub env: Option<std::collections::HashMap<String, String>>,
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UpdateServerArgs {
    pub name: Option<String>,
    #[serde(rename = "type")]
    pub server_type: Option<String>,
    pub command: Option<String>,
    pub args: Option<Vec<String>>,
    pub url: Option<String>,
    pub env: Option<std::collections::HashMap<String, String>>,
    pub description: Option<String>,
    pub is_active: Option<bool>,
}

// MCP Protocol Structs

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Tool {
    pub name: String,
    pub description: Option<String>,
    pub inputSchema: serde_json::Value,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Resource {
    pub uri: String,
    pub name: String,
    pub description: Option<String>,
    pub mimeType: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Prompt {
    pub name: String,
    pub description: Option<String>,
    pub arguments: Option<Vec<PromptArgument>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PromptArgument {
    pub name: String,
    pub description: Option<String>,
    pub required: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ListToolsResult {
    pub tools: Vec<Tool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ListResourcesResult {
    pub resources: Vec<Resource>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ListPromptsResult {
    pub prompts: Vec<Prompt>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Content {
    #[serde(rename = "type")]
    pub content_type: String,
    pub text: Option<String>,
    pub mimeType: Option<String>,
    pub data: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CallToolResult {
    pub content: Vec<Content>,
    pub isError: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResourceContent {
    pub uri: String,
    pub mimeType: Option<String>,
    pub text: Option<String>,
    pub blob: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReadResourceResult {
    pub contents: Vec<ResourceContent>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct RegistryItem {
    pub server: RegistryServer,
    pub install_config: Option<RegistryInstallConfig>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct RegistryServer {
    pub name: String,
    pub description: Option<String>,
    pub homepage: Option<String>,
    pub bugs: Option<String>,
    pub version: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct RegistryInstallConfig {
    pub command: String,   // e.g. "npx" or "uvx"
    pub args: Vec<String>, // e.g. ["-y", "@modelcontextprotocol/server-gdrive"]
    pub env_template: Option<std::collections::HashMap<String, String>>, // Keys to prompt for
}
