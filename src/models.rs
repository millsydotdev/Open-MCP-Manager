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
pub struct ResearchNote {
    pub id: String,
    pub title: String,
    pub content: Option<String>,
    pub tags: Vec<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct RegistryItem {
    pub server: RegistryServer,
    pub install_config: Option<RegistryInstallConfig>,
    #[serde(default = "default_source")]
    pub source: String, // "official" or "community"
    #[serde(default)]
    pub stars: u32,
    #[serde(default)]
    pub topics: Vec<String>,
}

fn default_source() -> String {
    "official".to_string()
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

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct GitHubSearchResponse {
    pub total_count: u32,
    pub items: Vec<GitHubRepo>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct GitHubRepo {
    pub name: String,
    pub full_name: String,
    pub description: Option<String>,
    pub html_url: String,
    pub stargazers_count: u32,
    pub topics: Vec<String>,
    pub language: Option<String>,
    pub updated_at: String,
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
            source: "official".to_string(),
            stars: 0,
            topics: vec![],
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
            source: "official".to_string(),
            stars: 0,
            topics: vec![],
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

    // === McpServer Tests ===

    #[test]
    fn test_mcp_server_serialization() {
        let server = McpServer {
            id: "test-id".to_string(),
            name: "test-server".to_string(),
            server_type: "stdio".to_string(),
            command: Some("npx".to_string()),
            args: Some(vec!["-y".to_string(), "test".to_string()]),
            url: None,
            env: Some(HashMap::from([("KEY".to_string(), "VALUE".to_string())])),
            description: Some("Test server".to_string()),
            is_active: true,
            created_at: "2024-01-01".to_string(),
            updated_at: "2024-01-01".to_string(),
        };

        let json = serde_json::to_string(&server).unwrap();
        assert!(json.contains("\"name\":\"test-server\""));
        assert!(json.contains("\"type\":\"stdio\"")); // Uses serde rename
    }

    #[test]
    fn test_mcp_server_deserialization() {
        let json = r#"{
            "id": "test-id",
            "name": "test-server",
            "type": "sse",
            "url": "https://example.com/sse",
            "is_active": true,
            "created_at": "2024-01-01",
            "updated_at": "2024-01-01"
        }"#;

        let server: McpServer = serde_json::from_str(json).unwrap();
        assert_eq!(server.name, "test-server");
        assert_eq!(server.server_type, "sse");
        assert_eq!(server.url, Some("https://example.com/sse".to_string()));
    }

    // === CreateServerArgs Tests ===

    #[test]
    fn test_create_server_args_default() {
        let args = CreateServerArgs::default();
        assert_eq!(args.name, "");
        assert_eq!(args.server_type, "");
        assert!(args.command.is_none());
        assert!(args.args.is_none());
        assert!(args.env.is_none());
    }

    #[test]
    fn test_create_server_args_serialization() {
        let args = CreateServerArgs {
            name: "test".to_string(),
            server_type: "stdio".to_string(),
            command: Some("npx".to_string()),
            args: Some(vec!["-y".to_string()]),
            url: None,
            env: None,
            description: None,
        };

        let json = serde_json::to_string(&args).unwrap();
        assert!(json.contains("\"type\":\"stdio\""));
    }

    // === AppError Tests ===

    #[test]
    fn test_app_error_display() {
        let db_error = AppError::Database("connection failed".to_string());
        assert_eq!(format!("{}", db_error), "Database error: connection failed");

        let io_error = AppError::Io("file not found".to_string());
        assert_eq!(format!("{}", io_error), "IO error: file not found");

        let ser_error = AppError::Serialization("invalid json".to_string());
        assert_eq!(
            format!("{}", ser_error),
            "Serialization error: invalid json"
        );
    }

    // === Notification Tests ===

    #[test]
    fn test_notification_level_equality() {
        assert_eq!(NotificationLevel::Info, NotificationLevel::Info);
        assert_eq!(NotificationLevel::Success, NotificationLevel::Success);
        assert_eq!(NotificationLevel::Warning, NotificationLevel::Warning);
        assert_eq!(NotificationLevel::Error, NotificationLevel::Error);
        assert_ne!(NotificationLevel::Info, NotificationLevel::Error);
    }

    #[test]
    fn test_notification_serialization() {
        let notification = Notification {
            id: 1,
            message: "Test message".to_string(),
            level: NotificationLevel::Success,
            duration: 5,
        };

        let json = serde_json::to_string(&notification).unwrap();
        assert!(json.contains("\"message\":\"Test message\""));
        assert!(json.contains("\"level\":\"Success\""));
    }

    // === Tool Tests ===

    #[test]
    fn test_tool_deserialization() {
        let json = r#"{
            "name": "test_tool",
            "description": "A test tool",
            "inputSchema": {"type": "object", "properties": {}}
        }"#;

        let tool: Tool = serde_json::from_str(json).unwrap();
        assert_eq!(tool.name, "test_tool");
        assert_eq!(tool.description, Some("A test tool".to_string()));
    }

    // === Resource Tests ===

    #[test]
    fn test_resource_deserialization() {
        let json = r#"{
            "uri": "file:///test.txt",
            "name": "test.txt",
            "mimeType": "text/plain"
        }"#;

        let resource: Resource = serde_json::from_str(json).unwrap();
        assert_eq!(resource.uri, "file:///test.txt");
        assert_eq!(resource.name, "test.txt");
        assert_eq!(resource.mimeType, Some("text/plain".to_string()));
    }

    // === Prompt Tests ===

    #[test]
    fn test_prompt_with_arguments() {
        let json = r#"{
            "name": "test_prompt",
            "description": "A test prompt",
            "arguments": [
                {"name": "arg1", "required": true},
                {"name": "arg2", "required": false}
            ]
        }"#;

        let prompt: Prompt = serde_json::from_str(json).unwrap();
        assert_eq!(prompt.name, "test_prompt");
        assert!(prompt.arguments.is_some());
        let args = prompt.arguments.unwrap();
        assert_eq!(args.len(), 2);
        assert_eq!(args[0].required, Some(true));
    }

    // === WizardAction Tests ===

    #[test]
    fn test_wizard_action_link_serialization() {
        let action = WizardAction::Link {
            url: "https://example.com".to_string(),
            label: "Click here".to_string(),
        };

        let json = serde_json::to_string(&action).unwrap();
        assert!(json.contains("\"type\":\"link\""));
        assert!(json.contains("\"url\":\"https://example.com\""));
    }

    #[test]
    fn test_wizard_action_input_serialization() {
        let action = WizardAction::Input {
            key: "API_KEY".to_string(),
            label: "API Key".to_string(),
            placeholder: Some("Enter your key".to_string()),
        };

        let json = serde_json::to_string(&action).unwrap();
        assert!(json.contains("\"type\":\"input\""));
        assert!(json.contains("\"key\":\"API_KEY\""));
    }

    #[test]
    fn test_wizard_action_message_serialization() {
        let action = WizardAction::Message {
            text: "Hello world".to_string(),
        };

        let json = serde_json::to_string(&action).unwrap();
        assert!(json.contains("\"type\":\"message\""));
        assert!(json.contains("\"text\":\"Hello world\""));
    }

    // === Content Tests ===

    #[test]
    fn test_content_text_deserialization() {
        let json = r#"{
            "type": "text",
            "text": "Hello world"
        }"#;

        let content: Content = serde_json::from_str(json).unwrap();
        assert_eq!(content.content_type, "text");
        assert_eq!(content.text, Some("Hello world".to_string()));
    }

    #[test]
    fn test_content_blob_deserialization() {
        let json = r#"{
            "type": "image",
            "mimeType": "image/png",
            "data": "base64data"
        }"#;

        let content: Content = serde_json::from_str(json).unwrap();
        assert_eq!(content.content_type, "image");
        assert_eq!(content.mimeType, Some("image/png".to_string()));
        assert_eq!(content.data, Some("base64data".to_string()));
    }

    // === CallToolResult Tests ===

    #[test]
    fn test_call_tool_result_success() {
        let json = r#"{
            "content": [{"type": "text", "text": "Result"}],
            "isError": false
        }"#;

        let result: CallToolResult = serde_json::from_str(json).unwrap();
        assert_eq!(result.content.len(), 1);
        assert_eq!(result.isError, Some(false));
    }

    #[test]
    fn test_call_tool_result_error() {
        let json = r#"{
            "content": [{"type": "text", "text": "Error occurred"}],
            "isError": true
        }"#;

        let result: CallToolResult = serde_json::from_str(json).unwrap();
        assert_eq!(result.isError, Some(true));
    }

    // === prepare_install_args edge cases ===

    #[test]
    fn test_prepare_install_args_preserves_description() {
        let item = RegistryItem {
            server: RegistryServer {
                name: "test".to_string(),
                description: Some("Test description".to_string()),
                homepage: None,
                bugs: None,
                version: None,
                category: None,
            },
            install_config: None,
            source: "official".to_string(),
            stars: 0,
            topics: vec![],
        };

        let args = prepare_install_args(&item, None);
        assert_eq!(args.description, Some("Test description".to_string()));
    }

    #[test]
    fn test_prepare_install_args_wizard_overrides_template() {
        let mut env_template = HashMap::new();
        env_template.insert("KEY1".to_string(), "default1".to_string());
        env_template.insert("KEY2".to_string(), "default2".to_string());

        let item = RegistryItem {
            server: RegistryServer {
                name: "test".to_string(),
                description: None,
                homepage: None,
                bugs: None,
                version: None,
                category: None,
            },
            install_config: Some(RegistryInstallConfig {
                command: "npx".to_string(),
                args: vec!["test".to_string()],
                env_template: Some(env_template),
                wizard: None,
            }),
            source: "official".to_string(),
            stars: 0,
            topics: vec![],
        };

        let mut wizard_data = HashMap::new();
        wizard_data.insert("KEY1".to_string(), "wizard_value".to_string());

        let args = prepare_install_args(&item, Some(&wizard_data));

        // Wizard value should override template
        assert_eq!(
            args.env.as_ref().unwrap().get("KEY1"),
            Some(&"wizard_value".to_string())
        );
        // Template value should remain if not in wizard data
        assert_eq!(
            args.env.as_ref().unwrap().get("KEY2"),
            Some(&"default2".to_string())
        );
    }
}
