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
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum NotificationLevel {
    Info,
    Success,
    Warning,
    Error,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Notification {
    pub id: u32,
    pub message: String,
    pub level: NotificationLevel,
    pub duration: u32, // in seconds
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
    pub category: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum WizardAction {
    Link {
        url: String,
        label: String,
    },
    Input {
        key: String,
        label: String,
        placeholder: Option<String>,
    },
    Message {
        text: String,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct WizardStep {
    pub title: String,
    pub description: String,
    pub action: WizardAction,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct RegistryInstallConfig {
    pub command: String,   // e.g. "npx" or "uvx"
    pub args: Vec<String>, // e.g. ["-y", "@modelcontextprotocol/server-gdrive"]
    pub env_template: Option<std::collections::HashMap<String, String>>, // Keys to prompt for
    pub wizard: Option<Vec<WizardStep>>,
}

pub fn prepare_install_args(
    item: &RegistryItem,
    wizard_env_data: Option<&std::collections::HashMap<String, String>>,
) -> CreateServerArgs {
    if let Some(config) = &item.install_config {
        let mut final_env = config.env_template.clone().unwrap_or_default();
        if let Some(w_data) = wizard_env_data {
            for (k, v) in w_data {
                final_env.insert(k.clone(), v.clone());
            }
        }

        CreateServerArgs {
            name: item.server.name.clone(),
            server_type: "stdio".to_string(), // Default to stdio for registry items
            command: Some(config.command.clone()),
            args: Some(config.args.clone()),
            env: Some(final_env),
            description: item.server.description.clone(),
            ..Default::default()
        }
    } else {
        // Default heuristic: npx -y <name>
        CreateServerArgs {
            name: item.server.name.clone(),
            server_type: "stdio".to_string(),
            command: Some("npx".to_string()),
            args: Some(vec!["-y".to_string(), item.server.name.clone()]),
            description: item.server.description.clone(),
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_prepare_install_args_simple() {
        let item = RegistryItem {
            server: RegistryServer {
                name: "simple-server".to_string(),
                description: None,
                homepage: None,
                bugs: None,
                version: None,
                category: None,
            },
            install_config: None,
        };

        let args = prepare_install_args(&item, None);
        assert_eq!(args.name, "simple-server");
        assert_eq!(args.command, Some("npx".to_string()));
        assert_eq!(
            args.args,
            Some(vec!["-y".to_string(), "simple-server".to_string()])
        );
    }

    #[test]
    fn test_prepare_install_args_with_config_and_wizard() {
        let mut env_template = HashMap::new();
        env_template.insert("API_KEY".to_string(), "".to_string());

        let item = RegistryItem {
            server: RegistryServer {
                name: "complex-server".to_string(),
                description: None,
                homepage: None,
                bugs: None,
                version: None,
                category: None,
            },
            install_config: Some(RegistryInstallConfig {
                command: "uvx".to_string(),
                args: vec!["complex-pkg".to_string()],
                env_template: Some(env_template),
                wizard: None, // Wizard steps don't matter for this logic, only the result map
            }),
        };

        let mut wizard_data = HashMap::new();
        wizard_data.insert("API_KEY".to_string(), "secret_123".to_string());

        let args = prepare_install_args(&item, Some(&wizard_data));

        assert_eq!(args.name, "complex-server");
        assert_eq!(args.command, Some("uvx".to_string()));
        assert_eq!(
            args.env.as_ref().unwrap().get("API_KEY"),
            Some(&"secret_123".to_string())
        );
    }
}
