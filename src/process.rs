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
