use crate::db::Database;
use crate::models::{
    CreateServerArgs, McpServer, Notification, NotificationLevel, UpdateServerArgs,
};
use crate::process::{McpProcess, ProcessLog};
use dioxus::prelude::*;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::process::Command;
use tokio::sync::mpsc; // Added for running updates

#[derive(Clone, Copy)]
pub struct AppState {
    pub servers: Signal<Vec<McpServer>>,
    pub processes: Signal<HashMap<String, Signal<String>>>,
    pub running_handlers: Signal<HashMap<String, Arc<McpProcess>>>,
    pub db: Signal<Option<Database>>,
    pub notifications: Signal<Vec<Notification>>, // New signal
}

// Global signal
pub static APP_STATE: GlobalSignal<AppState> = Signal::global(|| AppState {
    servers: Signal::new(Vec::new()),
    processes: Signal::new(HashMap::new()),
    running_handlers: Signal::new(HashMap::new()),
    db: Signal::new(None),
    notifications: Signal::new(Vec::new()),
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

    pub async fn ping_server(id: String) -> Result<u128, String> {
        let proc_opt = {
            let state = APP_STATE.read();
            let handlers = state.running_handlers.read();
            handlers.get(&id).cloned()
        };

        if let Some(proc) = proc_opt {
            let start = std::time::Instant::now();
            // We use list_tools as a ping mechanism. It's a standard MCP method.
            let _ = proc.list_tools().await.map_err(|e| e.to_string())?;
            let duration = start.elapsed().as_millis();
            Ok(duration)
        } else {
            Err("Process not running".into())
        }
    }

    pub fn push_notification(message: String, level: NotificationLevel) {
        let mut notifications = APP_STATE.write().notifications;
        // Simple ID generation using time
        let id = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .subsec_nanos();

        notifications.push(Notification {
            id,
            message,
            level,
            duration: 5,
        });
    }

    pub fn remove_notification(id: u32) {
        let mut notifications = APP_STATE.write().notifications;
        notifications.retain(|n| n.id != id);
    }

    pub async fn update_server_package(id: String) {
        let server_opt: Option<McpServer> = {
            let state = APP_STATE.read();
            let db_lock = state.db.read();
            if let Some(db) = db_lock.as_ref() {
                db.get_server(id).ok()
            } else {
                None
            }
        };

        if let Some(server) = server_opt {
            if let Some(cmd) = server.command {
                let cmd_str = cmd.as_str();

                // Heuristic for NPM
                if cmd_str == "npx" || cmd_str.ends_with("npx") || cmd_str.ends_with("npx.cmd") {
                    if let Some(args) = &server.args {
                        // Borrow args
                        let pkg_opt = args.iter().find(|a: &&String| !a.starts_with("-"));
                        if let Some(pkg) = pkg_opt {
                            Self::push_notification(
                                format!("Updating {}...", pkg),
                                NotificationLevel::Info,
                            );

                            let output = Command::new("npm")
                                .args(["install", "-g", &format!("{}@latest", pkg)])
                                .output()
                                .await;

                            match output {
                                Ok(o) => {
                                    if o.status.success() {
                                        Self::push_notification(
                                            format!("Updated {} successfully", pkg),
                                            NotificationLevel::Success,
                                        );
                                    } else {
                                        let err = String::from_utf8_lossy(&o.stderr);
                                        Self::push_notification(
                                            format!("Update failed: {}", err),
                                            NotificationLevel::Error,
                                        );
                                    }
                                }
                                Err(e) => {
                                    Self::push_notification(
                                        format!("Failed to run update: {}", e),
                                        NotificationLevel::Error,
                                    );
                                }
                            }
                            return;
                        }
                    }
                }

                // Heuristic for Python (uvx/uv)
                if cmd_str == "uvx" || cmd_str == "uv" {
                    if let Some(args) = &server.args {
                        // Borrow args
                        let pkg_opt = args.iter().find(|a: &&String| {
                            !a.starts_with("-") && a.as_str() != "tool" && a.as_str() != "run"
                        });
                        if let Some(pkg) = pkg_opt {
                            Self::push_notification(
                                format!("Updating {}...", pkg),
                                NotificationLevel::Info,
                            );
                            let output = Command::new("uv")
                                .args(["tool", "upgrade", pkg])
                                .output()
                                .await;
                            match output {
                                Ok(o) => {
                                    if o.status.success() {
                                        Self::push_notification(
                                            format!("Updated {} successfully", pkg),
                                            NotificationLevel::Success,
                                        );
                                    } else {
                                        let err = String::from_utf8_lossy(&o.stderr);
                                        Self::push_notification(
                                            format!("Update info: {}", err),
                                            NotificationLevel::Info,
                                        );
                                    }
                                }
                                Err(e) => Self::push_notification(
                                    format!("Update error: {}", e),
                                    NotificationLevel::Error,
                                ),
                            }
                            return;
                        }
                    }
                }

                Self::push_notification(
                    "Automatic update not supported for this configuration.".to_string(),
                    NotificationLevel::Warning,
                );
            }
        } else {
            Self::push_notification("Server not found".to_string(), NotificationLevel::Error);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_app_state_crud_headless() {
        // Create a dummy app to get a VirtualDom which provides the runtime for signals
        fn mock_app() -> Element {
            rsx! { div {} }
        }
        let mut dom = VirtualDom::new(mock_app);

        // Push the runtime to the current thread via rebuild
        dom.rebuild_in_place();

        dom.in_runtime(|| {
            // 1. Manually inject In-Memory DB
            let db = Database::new_in_memory().expect("failed create memory db");

            // This set() works because we are inside `in_runtime`
            APP_STATE.write().db.set(Some(db.clone()));

            // 2. Perform CRUD operations by manipulating signals manually to simulate the async actions
            // (Since actual AppState methods are async and returning futures outside runtime context is tricky)

            // Create
            let args = CreateServerArgs {
                name: "headless-test".to_string(),
                server_type: "stdio".to_string(),
                command: Some("echo".to_string()),
                args: None,
                url: None,
                env: None,
                description: None,
            };
            db.create_server(args).unwrap();

            // Refresh (simulate what AppState::refresh_servers does)
            let servers = db.get_servers().unwrap();
            APP_STATE.write().servers.set(servers);

            // Verify
            let s_list = APP_STATE.read().servers.cloned();
            assert_eq!(s_list.len(), 1);
            assert_eq!(s_list[0].name, "headless-test");

            // Delete
            let id = s_list[0].id.clone();
            db.delete_server(id).unwrap();
            let servers_after = db.get_servers().unwrap();
            APP_STATE.write().servers.set(servers_after);

            let s_list_after = APP_STATE.read().servers.cloned();
            assert_eq!(s_list_after.len(), 0);
        });
    }
}
