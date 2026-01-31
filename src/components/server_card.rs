use crate::models::McpServer;
use crate::state::APP_STATE;
use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct ServerCardProps {
    server: McpServer,
    on_console_click: EventHandler<()>,
    on_edit_click: EventHandler<()>,
}

pub fn ServerCard(props: ServerCardProps) -> Element {
    let server = props.server.clone();
    let processes = APP_STATE.read().processes.clone();

    // Check if running by looking up ID in processes map
    let is_running = use_memo(move || processes.read().contains_key(&server.id));

    let server_for_toggle = props.server.clone();
    let toggle_server = move |_| {
        let srv = server_for_toggle.clone();
        spawn(async move {
            let p = APP_STATE.read().processes.clone();
            let running = p.read().contains_key(&srv.id);

            if running {
                crate::state::AppState::stop_server_process(&srv.id).await;
            } else {
                let _ = crate::state::AppState::start_server_process(srv).await;
            }
        });
    };

    let server_for_restart = props.server.clone();
    let restart_server = move |_| {
        let srv = server_for_restart.clone();
        spawn(async move {
            // Stop then start
            crate::state::AppState::stop_server_process(&srv.id).await;
            let _ = crate::state::AppState::start_server_process(srv).await;
        });
    };

    let running = is_running();
    let desc = props.server.description.clone().unwrap_or_default();

    // Server type icon
    let type_icon = if props.server.server_type == "sse" {
        "🌐"
    } else {
        "⌨️"
    };
    let type_label = if props.server.server_type == "sse" {
        "Remote SSE"
    } else {
        "Local STDIO"
    };

    // Runtime config display
    let runtime_config = if props.server.server_type == "sse" {
        props
            .server
            .url
            .clone()
            .unwrap_or_else(|| "No URL".to_string())
    } else {
        let cmd = props.server.command.clone().unwrap_or_default();
        let args = props.server.args.clone().unwrap_or_default().join(" ");
        if args.is_empty() {
            cmd
        } else {
            format!("{} {}", cmd, args)
        }
    };

    // Env vars preview (first 3)
    let env_map = props.server.env.clone().unwrap_or_default();
    let env_keys: Vec<_> = env_map.keys().cloned().collect();
    let env_count = env_keys.len();
    let env_preview: Vec<_> = env_keys.into_iter().take(3).collect();

    rsx! {
        div {
            class: format!(
                "group relative flex flex-col overflow-hidden rounded-2xl border-2 transition-all hover:shadow-xl {} {}",
                if running { "border-zinc-200 bg-white shadow-lg dark:border-zinc-700 dark:bg-zinc-800" } else { "border-zinc-100 bg-zinc-50/50 opacity-70 dark:border-zinc-800 dark:bg-zinc-900/50" },
                if !running { "grayscale" } else { "" }
            ),

            // Background accent glow
            if running {
                div {
                    class: "absolute -right-12 -top-12 h-40 w-40 rounded-full bg-indigo-500/10 blur-3xl dark:bg-indigo-500/5",
                }
            }

            // Main content
            div {
                class: "relative flex-1 p-6",

                // Header: Icon, name, status
                div {
                    class: "flex items-start justify-between gap-4 mb-4",
                    div {
                        class: "flex items-center gap-4",
                        // Type icon
                        div {
                            class: format!(
                                "flex h-12 w-12 items-center justify-center rounded-xl text-2xl transition-all {}",
                                if running { "bg-gradient-to-br from-indigo-500 to-violet-600 shadow-lg" } else { "bg-zinc-200 dark:bg-zinc-700" }
                            ),
                            "{type_icon}"
                        }
                        div {
                            div {
                                class: "flex items-center gap-2",
                                h3 {
                                    class: "text-lg font-bold text-zinc-900 dark:text-white",
                                    "{props.server.name}"
                                }
                                // Pulsing status dot
                                span {
                                    class: format!(
                                        "h-2 w-2 rounded-full {}",
                                        if running { "bg-green-500 animate-pulse shadow-[0_0_8px_rgba(34,197,94,0.6)]" } else { "bg-zinc-300 dark:bg-zinc-600" }
                                    ),
                                }
                            }
                            p {
                                class: "text-xs text-zinc-500 dark:text-zinc-400 font-medium",
                                "{type_label}"
                            }
                        }
                    }
                    // Power toggle button
                    button {
                        class: format!(
                            "flex h-10 w-10 items-center justify-center rounded-xl transition-all active:scale-90 {}",
                            if running { "bg-green-100 text-green-600 hover:bg-green-200 dark:bg-green-900/30 dark:text-green-400" } else { "bg-zinc-100 text-zinc-400 hover:bg-zinc-200 dark:bg-zinc-700 dark:text-zinc-500" }
                        ),
                        onclick: toggle_server.clone(),
                        title: if running { "Stop Server" } else { "Start Server" },
                        "⏻"
                    }
                }

                // Description
                p {
                    class: "text-sm text-zinc-600 dark:text-zinc-400 line-clamp-2 mb-4 italic min-h-[2.5rem]",
                    "{desc}"
                }

                // Runtime config panel
                div {
                    class: "rounded-xl bg-zinc-50 p-4 dark:bg-zinc-900/50 border border-zinc-100 dark:border-zinc-700/50 mb-4",
                    div {
                        class: "flex items-center gap-2 text-[10px] font-bold uppercase tracking-wider text-zinc-400 dark:text-zinc-500 mb-2",
                        "⚡ Runtime Config"
                    }
                    div {
                        class: "font-mono text-xs text-zinc-700 dark:text-zinc-300 break-all leading-relaxed bg-white/50 dark:bg-black/20 rounded-lg p-2 border border-zinc-200/50 dark:border-zinc-700/50 truncate",
                        "{runtime_config}"
                    }
                }

                // Env vars preview
                if !env_preview.is_empty() {
                    div {
                        class: "flex flex-wrap gap-2",
                        for key in env_preview.iter() {
                            div {
                                key: "{key}",
                                class: "flex items-center gap-1 rounded-lg border border-zinc-100 bg-white px-2 py-1 text-[10px] dark:border-zinc-700 dark:bg-zinc-800",
                                span {
                                    class: "font-bold text-zinc-500 uppercase",
                                    "{key}"
                                }
                            }
                        }
                        if env_count > 3 {
                            div {
                                class: "rounded-lg bg-zinc-100 px-2 py-1 text-[10px] font-bold text-zinc-400 dark:bg-zinc-700",
                                "+{env_count - 3} more"
                            }
                        }
                    }
                }
            }

            // Action bar
            div {
                class: "flex items-center justify-between border-t border-zinc-100 bg-zinc-50/50 px-6 py-4 dark:border-zinc-700 dark:bg-zinc-900/30",
                div {
                    class: "flex items-center gap-3 text-[11px] font-bold text-zinc-400",
                    span { "{props.server.server_type.to_uppercase()}" }
                }
                div {
                    class: "flex items-center gap-2",
                    // Console/Logs
                    if props.server.server_type == "stdio" {
                        button {
                            class: "flex h-8 w-8 items-center justify-center rounded-lg text-zinc-400 transition-all hover:bg-white hover:text-green-600 hover:shadow dark:hover:bg-zinc-700 dark:hover:text-green-400",
                            onclick: move |_| (props.on_console_click)(()),
                            title: "Open Console",
                            "📟"
                        }
                    }
                    // Restart
                    button {
                        class: "flex h-8 w-8 items-center justify-center rounded-lg text-zinc-400 transition-all hover:bg-white hover:text-indigo-600 hover:shadow dark:hover:bg-zinc-700 dark:hover:text-indigo-400 active:rotate-180",
                        onclick: restart_server,
                        title: "Restart Server",
                        "🔄"
                    }
                    // Settings
                    button {
                        class: "flex h-8 w-8 items-center justify-center rounded-lg text-zinc-400 transition-all hover:bg-white hover:text-zinc-900 hover:shadow dark:hover:bg-zinc-700 dark:hover:text-zinc-200",
                        onclick: move |_| (props.on_edit_click)(()),
                        title: "Settings",
                        "⚙️"
                    }
                }
            }
        }
    }
}
