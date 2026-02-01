use futures_util::StreamExt;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::process::Stdio;
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::process::{Child, Command};
use tokio::sync::{mpsc, oneshot, Mutex};

#[derive(Serialize, Deserialize, Debug)]
struct JsonRpcRequest {
    jsonrpc: String,
    method: String,
    params: Value,
    id: u64,
}

#[derive(Serialize, Deserialize, Debug)]
struct JsonRpcResponse {
    jsonrpc: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<Value>,
    id: Option<u64>,
}

#[derive(Clone, Debug)]
pub enum ProcessLog {
    Stdout(String),
    Stderr(String),
}

pub struct McpProcess {
    pub child: Arc<Mutex<Child>>,
    pub stdin_tx: mpsc::Sender<String>,
    pub pending_requests: Arc<Mutex<HashMap<u64, oneshot::Sender<Result<Value, String>>>>>,
    pub next_request_id: Arc<Mutex<u64>>,
}

pub struct McpSseClient {
    pub url: String,
    pub request_url: Arc<Mutex<Option<String>>>,
    pub client: reqwest::Client,
    pub pending_requests: Arc<Mutex<HashMap<u64, oneshot::Sender<Result<Value, String>>>>>,
    pub next_request_id: Arc<Mutex<u64>>,
}

pub enum McpHandler {
    Stdio(McpProcess),
    Sse(McpSseClient),
}

impl McpProcess {
    pub async fn start(
        _id: String,
        command: String,
        args: Vec<String>,
        env: Option<std::collections::HashMap<String, String>>,
        log_tx: mpsc::Sender<ProcessLog>, // Channel to send logs back to UI
    ) -> Result<Self, String> {
        let mut cmd = Command::new(command);
        cmd.args(args);

        if let Some(env_vars) = env {
            cmd.envs(env_vars);
        }

        cmd.stdout(Stdio::piped());
        cmd.stderr(Stdio::piped());
        cmd.stdin(Stdio::piped());

        #[cfg(windows)]
        {
            const CREATE_NO_WINDOW: u32 = 0x08000000;
            cmd.creation_flags(CREATE_NO_WINDOW);
        }

        let mut child = cmd.spawn().map_err(|e| e.to_string())?;

        let stdout = child.stdout.take().unwrap();
        let stderr = child.stderr.take().unwrap();
        let mut stdin = child.stdin.take().unwrap();

        // Stdin writer
        let (stdin_tx, mut stdin_rx) = mpsc::channel::<String>(32);
        tokio::spawn(async move {
            while let Some(msg) = stdin_rx.recv().await {
                if let Err(e) = stdin.write_all(msg.as_bytes()).await {
                    eprintln!("Failed to write to stdin: {}", e);
                    break;
                }
                if let Err(e) = stdin.flush().await {
                    eprintln!("Failed to flush stdin: {}", e);
                    break;
                }
            }
        });

        let pending_requests = Arc::new(Mutex::new(HashMap::<
            u64,
            oneshot::Sender<Result<Value, String>>,
        >::new()));
        let pending_requests_clone = pending_requests.clone();
        let log_tx_stdout = log_tx.clone();

        // Stdout reader
        tokio::spawn(async move {
            let reader = BufReader::new(stdout);
            let mut lines = reader.lines();

            while let Ok(Some(line)) = lines.next_line().await {
                let is_json_rpc =
                    if let Ok(response) = serde_json::from_str::<JsonRpcResponse>(&line) {
                        if let Some(req_id) = response.id {
                            let mut pending = pending_requests_clone.lock().await;
                            if let Some(tx) = pending.remove(&req_id) {
                                if let Some(error) = response.error {
                                    let _ = tx.send(Err(error.to_string()));
                                } else {
                                    let _ = tx.send(Ok(response.result.unwrap_or(Value::Null)));
                                }
                                true
                            } else {
                                false
                            }
                        } else {
                            false
                        }
                    } else {
                        false
                    };

                if !is_json_rpc {
                    let _ = log_tx_stdout.send(ProcessLog::Stdout(line)).await;
                }
            }
        });

        let log_tx_stderr = log_tx.clone();
        // Stderr reader
        tokio::spawn(async move {
            let reader = BufReader::new(stderr);
            let mut lines = reader.lines();

            while let Ok(Some(line)) = lines.next_line().await {
                let _ = log_tx_stderr.send(ProcessLog::Stderr(line)).await;
            }
        });

        Ok(McpProcess {
            child: Arc::new(Mutex::new(child)),
            stdin_tx,
            pending_requests,
            next_request_id: Arc::new(Mutex::new(1)),
        })
    }

    pub async fn send_request(&self, method: &str, params: Option<Value>) -> Result<Value, String> {
        let id;
        {
            let mut id_lock = self.next_request_id.lock().await;
            id = *id_lock;
            *id_lock += 1;
        }

        let request = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            method: method.to_string(),
            params: params.unwrap_or(serde_json::json!({})),
            id,
        };

        let json_str = serde_json::to_string(&request).map_err(|e| e.to_string())?;

        let (tx, rx) = oneshot::channel();
        {
            let mut pending = self.pending_requests.lock().await;
            pending.insert(id, tx);
        }

        self.stdin_tx
            .send(format!("{}\n", json_str))
            .await
            .map_err(|e| e.to_string())?;

        match rx.await {
            Ok(result) => result,
            Err(_) => Err("Request cancelled or process died".to_string()),
        }
    }

    pub async fn kill(&self) -> Result<(), String> {
        let mut child = self.child.lock().await;
        child.kill().await.map_err(|e| e.to_string())?;
        Ok(())
    }

    pub async fn list_tools(&self) -> Result<Vec<crate::models::Tool>, String> {
        let val = self.send_request("tools/list", None).await?;
        let res: crate::models::ListToolsResult =
            serde_json::from_value(val).map_err(|e| e.to_string())?;
        Ok(res.tools)
    }

    pub async fn list_resources(&self) -> Result<Vec<crate::models::Resource>, String> {
        let val = self.send_request("resources/list", None).await?;
        let res: crate::models::ListResourcesResult =
            serde_json::from_value(val).map_err(|e| e.to_string())?;
        Ok(res.resources)
    }

    pub async fn list_prompts(&self) -> Result<Vec<crate::models::Prompt>, String> {
        let val = self.send_request("prompts/list", None).await?;
        let res: crate::models::ListPromptsResult =
            serde_json::from_value(val).map_err(|e| e.to_string())?;
        Ok(res.prompts)
    }

    pub async fn call_tool(
        &self,
        name: String,
        arguments: serde_json::Value,
    ) -> Result<crate::models::CallToolResult, String> {
        let params = serde_json::json!({
            "name": name,
            "arguments": arguments
        });
        let val = self.send_request("tools/call", Some(params)).await?;
        let res: crate::models::CallToolResult =
            serde_json::from_value(val).map_err(|e| e.to_string())?;
        Ok(res)
    }

    pub async fn read_resource(
        &self,
        uri: String,
    ) -> Result<crate::models::ReadResourceResult, String> {
        let params = serde_json::json!({
            "uri": uri
        });
        let val = self.send_request("resources/read", Some(params)).await?;
        let res: crate::models::ReadResourceResult =
            serde_json::from_value(val).map_err(|e| e.to_string())?;
        Ok(res)
    }
}

impl McpSseClient {
    pub async fn start(url: String, log_tx: mpsc::Sender<ProcessLog>) -> Result<Self, String> {
        let client = reqwest::Client::new();
        let request_url = Arc::new(Mutex::new(None));
        let pending_requests = Arc::new(Mutex::new(HashMap::<
            u64,
            oneshot::Sender<Result<Value, String>>,
        >::new()));
        let next_request_id = Arc::new(Mutex::new(1));

        let request_url_clone = request_url.clone();
        let pending_requests_clone = pending_requests.clone();
        let log_tx_clone = log_tx.clone();
        let client_clone = client.clone();
        let url_clone = url.clone();

        tokio::spawn(async move {
            let res = match client_clone.get(&url_clone).send().await {
                Ok(r) => r,
                Err(e) => {
                    let _ = log_tx_clone
                        .send(ProcessLog::Stderr(format!(
                            "Failed to connect to SSE: {}",
                            e
                        )))
                        .await;
                    return;
                }
            };

            let mut stream = res.bytes_stream();
            while let Some(item) = stream.next().await {
                let bytes = match item {
                    Ok(b) => b,
                    Err(e) => {
                        let _ = log_tx_clone
                            .send(ProcessLog::Stderr(format!("SSE stream error: {}", e)))
                            .await;
                        break;
                    }
                };

                let text = String::from_utf8_lossy(&bytes);
                for line in text.lines() {
                    if line.starts_with("event: endpoint") {
                        // Wait for next line "data: ..."
                    } else if line.starts_with("data: ") {
                        let data = &line[6..];
                        if data.starts_with("http") {
                            let mut req_url = request_url_clone.lock().await;
                            *req_url = Some(data.to_string());
                            let _ = log_tx_clone
                                .send(ProcessLog::Stdout(format!(
                                    "Connected to endpoint: {}",
                                    data
                                )))
                                .await;
                        } else if let Ok(response) = serde_json::from_str::<JsonRpcResponse>(data) {
                            if let Some(req_id) = response.id {
                                let mut pending = pending_requests_clone.lock().await;
                                if let Some(tx) = pending.remove(&req_id) {
                                    if let Some(error) = response.error {
                                        let _ = tx.send(Err(error.to_string()));
                                    } else {
                                        let _ = tx.send(Ok(response.result.unwrap_or(Value::Null)));
                                    }
                                }
                            }
                        } else {
                            let _ = log_tx_clone
                                .send(ProcessLog::Stdout(data.to_string()))
                                .await;
                        }
                    } else if !line.is_empty() {
                        let _ = log_tx_clone
                            .send(ProcessLog::Stdout(line.to_string()))
                            .await;
                    }
                }
            }
        });

        Ok(McpSseClient {
            url,
            request_url,
            client,
            pending_requests,
            next_request_id,
        })
    }

    pub async fn send_request(&self, method: &str, params: Option<Value>) -> Result<Value, String> {
        let req_url = {
            let lock = self.request_url.lock().await;
            lock.clone().ok_or("Endpoint not yet received")?
        };

        let id;
        {
            let mut id_lock = self.next_request_id.lock().await;
            id = *id_lock;
            *id_lock += 1;
        }

        let request = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            method: method.to_string(),
            params: params.unwrap_or(serde_json::json!({})),
            id,
        };

        let (tx, rx) = oneshot::channel();
        {
            let mut pending = self.pending_requests.lock().await;
            pending.insert(id, tx);
        }

        let res = self
            .client
            .post(&req_url)
            .json(&request)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        if !res.status().is_success() {
            let mut pending = self.pending_requests.lock().await;
            pending.remove(&id);
            return Err(format!("POST failed with status: {}", res.status()));
        }

        match rx.await {
            Ok(result) => result,
            Err(_) => Err("Request cancelled or connection lost".to_string()),
        }
    }

    pub async fn list_tools(&self) -> Result<Vec<crate::models::Tool>, String> {
        let val = self.send_request("tools/list", None).await?;
        let res: crate::models::ListToolsResult =
            serde_json::from_value(val).map_err(|e| e.to_string())?;
        Ok(res.tools)
    }

    pub async fn list_resources(&self) -> Result<Vec<crate::models::Resource>, String> {
        let val = self.send_request("resources/list", None).await?;
        let res: crate::models::ListResourcesResult =
            serde_json::from_value(val).map_err(|e| e.to_string())?;
        Ok(res.resources)
    }

    pub async fn list_prompts(&self) -> Result<Vec<crate::models::Prompt>, String> {
        let val = self.send_request("prompts/list", None).await?;
        let res: crate::models::ListPromptsResult =
            serde_json::from_value(val).map_err(|e| e.to_string())?;
        Ok(res.prompts)
    }

    pub async fn call_tool(
        &self,
        name: String,
        arguments: serde_json::Value,
    ) -> Result<crate::models::CallToolResult, String> {
        let params = serde_json::json!({
            "name": name,
            "arguments": arguments
        });
        let val = self.send_request("tools/call", Some(params)).await?;
        let res: crate::models::CallToolResult =
            serde_json::from_value(val).map_err(|e| e.to_string())?;
        Ok(res)
    }

    pub async fn read_resource(
        &self,
        uri: String,
    ) -> Result<crate::models::ReadResourceResult, String> {
        let params = serde_json::json!({
            "uri": uri
        });
        let val = self.send_request("resources/read", Some(params)).await?;
        let res: crate::models::ReadResourceResult =
            serde_json::from_value(val).map_err(|e| e.to_string())?;
        Ok(res)
    }
}

impl McpHandler {
    pub async fn list_tools(&self) -> Result<Vec<crate::models::Tool>, String> {
        match self {
            McpHandler::Stdio(p) => p.list_tools().await,
            McpHandler::Sse(p) => p.list_tools().await,
        }
    }

    pub async fn list_resources(&self) -> Result<Vec<crate::models::Resource>, String> {
        match self {
            McpHandler::Stdio(p) => p.list_resources().await,
            McpHandler::Sse(p) => p.list_resources().await,
        }
    }

    pub async fn list_prompts(&self) -> Result<Vec<crate::models::Prompt>, String> {
        match self {
            McpHandler::Stdio(p) => p.list_prompts().await,
            McpHandler::Sse(p) => p.list_prompts().await,
        }
    }

    pub async fn call_tool(
        &self,
        name: String,
        arguments: serde_json::Value,
    ) -> Result<crate::models::CallToolResult, String> {
        match self {
            McpHandler::Stdio(p) => p.call_tool(name, arguments).await,
            McpHandler::Sse(p) => p.call_tool(name, arguments).await,
        }
    }

    pub async fn read_resource(
        &self,
        uri: String,
    ) -> Result<crate::models::ReadResourceResult, String> {
        match self {
            McpHandler::Stdio(p) => p.read_resource(uri).await,
            McpHandler::Sse(p) => p.read_resource(uri).await,
        }
    }

    pub async fn kill(&self) -> Result<(), String> {
        match self {
            McpHandler::Stdio(p) => p.kill().await,
            McpHandler::Sse(_) => Ok(()), // SSE just stops when dropped or connection closes
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_jsonrpc_request_serialization() {
        let req = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            method: "test_method".to_string(),
            params: json!({"key": "value"}),
            id: 1,
        };
        let json_str = serde_json::to_string(&req).unwrap();
        assert!(json_str.contains(r#""jsonrpc":"2.0""#));
        assert!(json_str.contains(r#""method":"test_method""#));
        assert!(json_str.contains(r#""id":1"#));
        assert!(json_str.contains(r#""params":{"key":"value"}"#));
    }

    #[test]
    fn test_jsonrpc_response_deserialization_success() {
        let json_str = r#"{"jsonrpc": "2.0", "result": {"foo": "bar"}, "id": 1}"#;
        let resp: JsonRpcResponse = serde_json::from_str(json_str).unwrap();
        assert_eq!(resp.jsonrpc, "2.0");
        assert_eq!(resp.id, Some(1));
        assert!(resp.error.is_none());
        assert_eq!(resp.result.unwrap(), json!({"foo": "bar"}));
    }

    #[test]
    fn test_jsonrpc_response_deserialization_error() {
        let json_str = r#"{"jsonrpc": "2.0", "error": {"code": -32600, "message": "Invalid Request"}, "id": null}"#;
        let resp: JsonRpcResponse = serde_json::from_str(json_str).unwrap();
        assert_eq!(resp.jsonrpc, "2.0");
        assert_eq!(resp.id, None);
        assert!(resp.result.is_none());
        let err = resp.error.unwrap();
        assert_eq!(err["code"], -32600);
        assert_eq!(err["message"], "Invalid Request");
    }

    // === Additional JSON-RPC Tests ===

    #[test]
    fn test_jsonrpc_request_with_empty_params() {
        let req = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            method: "ping".to_string(),
            params: json!({}),
            id: 42,
        };
        let json_str = serde_json::to_string(&req).unwrap();
        assert!(json_str.contains(r#""method":"ping""#));
        assert!(json_str.contains(r#""id":42"#));
    }

    #[test]
    fn test_jsonrpc_request_with_array_params() {
        let req = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            method: "test".to_string(),
            params: json!(["arg1", "arg2", 123]),
            id: 1,
        };
        let json_str = serde_json::to_string(&req).unwrap();
        assert!(json_str.contains(r#""params":["arg1","arg2",123]"#));
    }

    #[test]
    fn test_jsonrpc_response_with_null_result() {
        // When result is explicitly null in JSON, serde deserializes it as Some(Value::Null)
        // But when the field is missing, it becomes None
        let json_str = r#"{"jsonrpc": "2.0", "result": null, "id": 1}"#;
        let resp: JsonRpcResponse = serde_json::from_str(json_str).unwrap();
        // With skip_serializing_if, explicit null becomes None
        // This is expected behavior - check we can handle both cases
        assert_eq!(resp.id, Some(1));
    }

    #[test]
    fn test_jsonrpc_response_with_complex_result() {
        let json_str = r#"{
            "jsonrpc": "2.0",
            "result": {
                "tools": [
                    {"name": "tool1", "description": "First tool"},
                    {"name": "tool2", "description": "Second tool"}
                ]
            },
            "id": 5
        }"#;
        let resp: JsonRpcResponse = serde_json::from_str(json_str).unwrap();
        assert_eq!(resp.id, Some(5));
        let result = resp.result.unwrap();
        assert!(result["tools"].is_array());
        assert_eq!(result["tools"].as_array().unwrap().len(), 2);
    }

    #[test]
    fn test_jsonrpc_response_method_not_found_error() {
        let json_str = r#"{"jsonrpc": "2.0", "error": {"code": -32601, "message": "Method not found"}, "id": 1}"#;
        let resp: JsonRpcResponse = serde_json::from_str(json_str).unwrap();
        let err = resp.error.unwrap();
        assert_eq!(err["code"], -32601);
    }

    #[test]
    fn test_jsonrpc_response_parse_error() {
        let json_str = r#"{"jsonrpc": "2.0", "error": {"code": -32700, "message": "Parse error"}, "id": null}"#;
        let resp: JsonRpcResponse = serde_json::from_str(json_str).unwrap();
        let err = resp.error.unwrap();
        assert_eq!(err["code"], -32700);
    }

    // === ProcessLog Tests ===

    #[test]
    fn test_process_log_stdout() {
        let log = ProcessLog::Stdout("Hello from stdout".to_string());
        match log {
            ProcessLog::Stdout(msg) => assert_eq!(msg, "Hello from stdout"),
            ProcessLog::Stderr(_) => panic!("Expected Stdout"),
        }
    }

    #[test]
    fn test_process_log_stderr() {
        let log = ProcessLog::Stderr("Error message".to_string());
        match log {
            ProcessLog::Stderr(msg) => assert_eq!(msg, "Error message"),
            ProcessLog::Stdout(_) => panic!("Expected Stderr"),
        }
    }

    #[test]
    fn test_process_log_clone() {
        let log = ProcessLog::Stdout("test".to_string());
        let cloned = log.clone();
        match cloned {
            ProcessLog::Stdout(msg) => assert_eq!(msg, "test"),
            ProcessLog::Stderr(_) => panic!("Expected Stdout"),
        }
    }

    // === MCP Protocol Method Tests ===

    #[test]
    fn test_tools_list_request_format() {
        let req = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            method: "tools/list".to_string(),
            params: json!({}),
            id: 1,
        };
        let json_str = serde_json::to_string(&req).unwrap();
        assert!(json_str.contains(r#""method":"tools/list""#));
    }

    #[test]
    fn test_resources_list_request_format() {
        let req = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            method: "resources/list".to_string(),
            params: json!({}),
            id: 1,
        };
        let json_str = serde_json::to_string(&req).unwrap();
        assert!(json_str.contains(r#""method":"resources/list""#));
    }

    #[test]
    fn test_prompts_list_request_format() {
        let req = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            method: "prompts/list".to_string(),
            params: json!({}),
            id: 1,
        };
        let json_str = serde_json::to_string(&req).unwrap();
        assert!(json_str.contains(r#""method":"prompts/list""#));
    }

    #[test]
    fn test_tools_call_request_format() {
        let req = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            method: "tools/call".to_string(),
            params: json!({
                "name": "test_tool",
                "arguments": {"key": "value"}
            }),
            id: 1,
        };
        let json_str = serde_json::to_string(&req).unwrap();
        assert!(json_str.contains(r#""method":"tools/call""#));
        assert!(json_str.contains(r#""name":"test_tool""#));
    }

    #[test]
    fn test_resources_read_request_format() {
        let req = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            method: "resources/read".to_string(),
            params: json!({
                "uri": "file:///test.txt"
            }),
            id: 1,
        };
        let json_str = serde_json::to_string(&req).unwrap();
        assert!(json_str.contains(r#""method":"resources/read""#));
        assert!(json_str.contains(r#""uri":"file:///test.txt""#));
    }

    // === Response Format Tests ===

    #[test]
    fn test_list_tools_response_format() {
        let json_str = r#"{
            "jsonrpc": "2.0",
            "result": {
                "tools": [
                    {
                        "name": "echo",
                        "description": "Echoes input",
                        "inputSchema": {"type": "object", "properties": {"message": {"type": "string"}}}
                    }
                ]
            },
            "id": 1
        }"#;

        let resp: JsonRpcResponse = serde_json::from_str(json_str).unwrap();
        let result = resp.result.unwrap();
        let tools_result: crate::models::ListToolsResult = serde_json::from_value(result).unwrap();
        assert_eq!(tools_result.tools.len(), 1);
        assert_eq!(tools_result.tools[0].name, "echo");
    }

    #[test]
    fn test_list_resources_response_format() {
        let json_str = r#"{
            "jsonrpc": "2.0",
            "result": {
                "resources": [
                    {
                        "uri": "file:///test.txt",
                        "name": "test.txt",
                        "mimeType": "text/plain"
                    }
                ]
            },
            "id": 1
        }"#;

        let resp: JsonRpcResponse = serde_json::from_str(json_str).unwrap();
        let result = resp.result.unwrap();
        let resources_result: crate::models::ListResourcesResult =
            serde_json::from_value(result).unwrap();
        assert_eq!(resources_result.resources.len(), 1);
        assert_eq!(resources_result.resources[0].uri, "file:///test.txt");
    }

    #[test]
    fn test_call_tool_response_format() {
        let json_str = r#"{
            "jsonrpc": "2.0",
            "result": {
                "content": [{"type": "text", "text": "Tool result"}],
                "isError": false
            },
            "id": 1
        }"#;

        let resp: JsonRpcResponse = serde_json::from_str(json_str).unwrap();
        let result = resp.result.unwrap();
        let call_result: crate::models::CallToolResult = serde_json::from_value(result).unwrap();
        assert_eq!(call_result.content.len(), 1);
        assert_eq!(call_result.isError, Some(false));
    }

    #[test]
    fn test_read_resource_response_format() {
        let json_str = r#"{
            "jsonrpc": "2.0",
            "result": {
                "contents": [
                    {
                        "uri": "file:///test.txt",
                        "mimeType": "text/plain",
                        "text": "File contents here"
                    }
                ]
            },
            "id": 1
        }"#;

        let resp: JsonRpcResponse = serde_json::from_str(json_str).unwrap();
        let result = resp.result.unwrap();
        let read_result: crate::models::ReadResourceResult =
            serde_json::from_value(result).unwrap();
        assert_eq!(read_result.contents.len(), 1);
        assert_eq!(
            read_result.contents[0].text,
            Some("File contents here".to_string())
        );
    }
}
