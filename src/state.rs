use crate::db::Database;
use crate::models::{CreateServerArgs, McpServer, UpdateServerArgs};
use crate::process::{McpProcess, ProcessLog};
use dioxus::prelude::*;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::mpsc;

#[derive(Clone, Copy)]
pub struct AppState {
    pub servers: Signal<Vec<McpServer>>,
    pub processes: Signal<HashMap<String, Signal<String>>>, // Log signals
    pub running_handlers: Signal<HashMap<String, Arc<McpProcess>>>, // Process handles
    pub db: Signal<Option<Database>>,
}

// Global signal
pub static APP_STATE: GlobalSignal<AppState> = Signal::global(|| AppState {
    servers: Signal::new(Vec::new()),
    processes: Signal::new(HashMap::new()),
    running_handlers: Signal::new(HashMap::new()),
    db: Signal::new(None),
});

pub fn use_app_state() {
    use_hook(|| {
        spawn(async move {
            let db_res = Database::new();
            match db_res {
                Ok(db) => {
                    APP_STATE.write().db.set(Some(db.clone()));
                    if let Ok(servers) = db.get_servers() {
                        APP_STATE.write().servers.set(servers);
                    }
                }
                Err(e) => {
                    tracing::error!("Failed to init DB: {}", e);
                }
            }
        });
    });
}

impl AppState {
    pub async fn refresh_servers() {
        let db_opt = APP_STATE.read().db.cloned();
        if let Some(db) = db_opt {
            if let Ok(servers) = db.get_servers() {
                APP_STATE.write().servers.set(servers);
            }
        }
    }

    pub async fn add_server(args: CreateServerArgs) -> Result<(), String> {
        let db_opt = APP_STATE.read().db.cloned();
        if let Some(db) = db_opt {
            db.create_server(args).map_err(|e| e.to_string())?;
            Self::refresh_servers().await;
            Ok(())
        } else {
            Err("DB not initialized".into())
        }
    }

    pub async fn update_server(id: String, args: UpdateServerArgs) -> Result<(), String> {
        let db_opt = APP_STATE.read().db.cloned();
        if let Some(db) = db_opt {
            db.update_server(id, args).map_err(|e| e.to_string())?;
            Self::refresh_servers().await;
            Ok(())
        } else {
            Err("DB not initialized".into())
        }
    }

    pub async fn delete_server(id: String) -> Result<(), String> {
        let db_opt = APP_STATE.read().db.cloned();
        if let Some(db) = db_opt {
            db.delete_server(id).map_err(|e| e.to_string())?;
            Self::refresh_servers().await;
            Ok(())
        } else {
            Err("DB not initialized".into())
        }
    }

    pub async fn start_server_process(server: McpServer) -> Result<(), String> {
        if server.command.is_none() {
            return Err("No command specified".into());
        }

        // Don't start if already running
        if APP_STATE
            .read()
            .running_handlers
            .read()
            .contains_key(&server.id)
        {
            return Ok(());
        }

        let (log_tx, mut log_rx) = mpsc::channel(100);
        let log_signal = Signal::new(String::new());

        // Spawn listener for logs
        let s_id = server.id.clone();
        let mut s_log_sig = log_signal; // copy signal
        spawn(async move {
            while let Some(log) = log_rx.recv().await {
                let line = match log {
                    ProcessLog::Stdout(s) => format!("[stdout] {}\n", s),
                    ProcessLog::Stderr(s) => format!("[stderr] {}\n", s),
                };
                // Update the global signal for this process
                s_log_sig.with_mut(|s| s.push_str(&line));
                // Also log to tracing
                tracing::debug!("[{}] {}", s_id, line.trim());
            }
        });

        // Store log signal in map
        APP_STATE
            .write()
            .processes
            .write()
            .insert(server.id.clone(), log_signal);

        let env_map = server.env.unwrap_or_default();
        let cmd = server.command.unwrap();
        let args = server.args.unwrap_or_default();

        let process_res =
            McpProcess::start(server.id.clone(), cmd, args, Some(env_map), log_tx).await;

        match process_res {
            Ok(proc) => {
                let arc_proc = Arc::new(proc);
                let state = APP_STATE.read();
                let mut handlers = state.running_handlers;
                handlers.write().insert(server.id.clone(), arc_proc);
                tracing::info!("Started server {}", server.name);
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    pub async fn stop_server_process(id: &str) {
        // Retrieve process handle
        let proc_opt = {
            let state = APP_STATE.read();
            let handlers = state.running_handlers.read();
            handlers.get(id).cloned()
        };

        if let Some(proc) = proc_opt {
            if let Err(e) = proc.kill().await {
                tracing::error!("Failed to kill process {}: {}", id, e);
            } else {
                tracing::info!("Process {} killed", id);
            }
        }

        // Cleanup maps
        APP_STATE.write().running_handlers.write().remove(id);
        APP_STATE.write().processes.write().remove(id);
    }

    pub async fn get_tools(id: String) -> Result<Vec<crate::models::Tool>, String> {
        let proc_opt = {
            let state = APP_STATE.read();
            let handlers = state.running_handlers.read();
            handlers.get(&id).cloned()
        };

        if let Some(proc) = proc_opt {
            let tools = proc.list_tools().await?;
            Ok(tools)
        } else {
            Err("Process not running".into())
        }
    }

    pub async fn get_resources(id: String) -> Result<Vec<crate::models::Resource>, String> {
        let proc_opt = {
            let state = APP_STATE.read();
            let handlers = state.running_handlers.read();
            handlers.get(&id).cloned()
        };

        if let Some(proc) = proc_opt {
            let resources = proc.list_resources().await?;
            Ok(resources)
        } else {
            Err("Process not running".into())
        }
    }

    pub async fn get_prompts(id: String) -> Result<Vec<crate::models::Prompt>, String> {
        let proc_opt = {
            let state = APP_STATE.read();
            let handlers = state.running_handlers.read();
            handlers.get(&id).cloned()
        };

        if let Some(proc) = proc_opt {
            let prompts = proc.list_prompts().await?;
            Ok(prompts)
        } else {
            Err("Process not running".into())
        }
    }

    pub async fn execute_tool(
        id: String,
        name: String,
        args: serde_json::Value,
    ) -> Result<crate::models::CallToolResult, String> {
        let proc_opt = {
            let state = APP_STATE.read();
            let handlers = state.running_handlers.read();
            handlers.get(&id).cloned()
        };

        if let Some(proc) = proc_opt {
            proc.call_tool(name, args).await
        } else {
            Err("Process not running".into())
        }
    }

    pub async fn read_resource(
        id: String,
        uri: String,
    ) -> Result<crate::models::ReadResourceResult, String> {
        let proc_opt = {
            let state = APP_STATE.read();
            let handlers = state.running_handlers.read();
            handlers.get(&id).cloned()
        };

        if let Some(proc) = proc_opt {
            proc.read_resource(uri).await
        } else {
            Err("Process not running".into())
        }
    }
}
