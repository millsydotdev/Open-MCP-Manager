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

    // Icons
    let type_icon = if props.server.server_type == "sse" {
        // Globe icon
        rsx! {
            svg { class: "w-6 h-6", fill: "none", view_box: "0 0 24 24", stroke: "currentColor", stroke_width: "1.5",
                path { stroke_linecap: "round", stroke_linejoin: "round", d: "M21 12a9 9 0 01-9 9m9-9a9 9 0 00-9-9m9 9H3m9 9a9 9 0 01-9-9m9 9c1.657 0 3-4.03 3-9s-1.343-9-3-9m0 18c-1.657 0-3-4.03-3-9s1.343-9 3-9m-9 9a9 9 0 019-9" }
            }
        }
    } else {
        // Terminal/Command icon
        rsx! {
            svg { class: "w-6 h-6", fill: "none", view_box: "0 0 24 24", stroke: "currentColor", stroke_width: "1.5",
                path { stroke_linecap: "round", stroke_linejoin: "round", d: "M8 9l3 3-3 3m5 0h3M5 20h14a2 2 0 002-2V6a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z" }
            }
        }
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

    let bg_class = if running {
        "bg-zinc-900/90 border-red-500/30 shadow-[0_0_30px_rgba(220,38,38,0.15)]"
    } else {
        "glass-panel hover:bg-zinc-900/80 hover:border-white/10"
    };

    rsx! {
        div {
            class: "group relative flex flex-col overflow-hidden rounded-2xl border transition-all duration-300 {bg_class}",

            // Content Container
            div {
                class: "p-6 flex flex-col h-full relative z-10",

                // Header
                div {
                    class: "flex items-start justify-between gap-4 mb-5",
                    div {
                        class: "flex items-center gap-4",
                        // Icon Box
                        div {
                            class: format!(
                                "flex h-14 w-14 items-center justify-center rounded-2xl transition-all duration-300 {}",
                                if running { "bg-gradient-to-br from-red-600 to-red-500 text-white shadow-lg shadow-red-500/30 ring-2 ring-red-500/20" }
                                else { "bg-zinc-800 text-zinc-400 group-hover:text-zinc-200" }
                            ),
                            {type_icon}
                        }

                        // Title & Status
                        div {
                            class: "flex flex-col gap-1",
                            h3 {
                                class: "text-xl font-bold text-white tracking-tight group-hover:text-red-400 transition-colors",
                                "{props.server.name}"
                            }
                            div {
                                class: "flex items-center gap-2",
                                span {
                                    class: format!(
                                        "h-2 w-2 rounded-full {}",
                                        if running { "bg-green-400 shadow-[0_0_8px_rgba(74,222,128,0.6)] animate-pulse" } else { "bg-zinc-600" }
                                    ),
                                }
                                span {
                                    class: "text-xs font-medium text-zinc-400 uppercase tracking-wider",
                                    "{type_label}"
                                }
                            }
                        }
                    }

                    // Power Button
                    button {
                        class: format!(
                            "flex h-10 w-10 items-center justify-center rounded-xl transition-all active:scale-95 duration-200 {}",
                            if running { "bg-red-500/10 text-red-400 hover:bg-red-500/20 ring-1 ring-red-500/20" }
                            else { "bg-green-500/10 text-green-400 hover:bg-green-500/20 ring-1 ring-green-500/20" }
                        ),
                        onclick: toggle_server.clone(),
                        title: if running { "Stop Server" } else { "Start Server" },
                        svg { class: "w-5 h-5", fill: "none", view_box: "0 0 24 24", stroke: "currentColor", stroke_width: "2",
                            path { stroke_linecap: "round", stroke_linejoin: "round", d: "M5.636 5.636a9 9 0 1012.728 0M12 3v9" }
                        }
                    }
                }

                // Description
                div {
                    class: "mb-6 min-h-[40px]",
                    if desc.is_empty() {
                         p { class: "text-sm text-zinc-600 italic", "No description provided." }
                    } else {
                         p { class: "text-sm text-zinc-400 leading-relaxed line-clamp-2", "{desc}" }
                    }
                }

                // Details Area
                div {
                    class: "mt-auto space-y-3",

                    // Config Box
                    div {
                        class: "rounded-xl bg-black-30 border border-white-5 p-3",
                        div {
                            class: "flex items-center gap-2 text-[10px] font-bold uppercase tracking-wider text-zinc-500 mb-1.5",
                            svg { class: "w-3 h-3 text-red-500", fill: "none", view_box: "0 0 24 24", stroke: "currentColor", stroke_width: "2", path { stroke_linecap: "round", stroke_linejoin: "round", d: "M13 10V3L4 14h7v7l9-11h-7z" }},
                            "Config Source"
                        }
                        div {
                            class: "font-mono text-xs text-zinc-300 truncate opacity-80",
                            "{runtime_config}"
                        }
                    }

                    // Env Vars
                    if !env_preview.is_empty() {
                        div {
                            class: "flex flex-wrap gap-2 pt-1",
                            for key in env_preview.iter() {
                                span {
                                    class: "px-2 py-1 rounded bg-white-8 border border-white-5 text-[10px] font-mono text-zinc-400",
                                    "{key}"
                                }
                            }
                            if env_count > 3 {
                                span {
                                    class: "px-2 py-1 rounded bg-white-5 text-[10px] font-bold text-zinc-500",
                                    "+{env_count - 3}"
                                }
                            }
                        }
                    }
                }
            }

            // Footer Actions
            div {
                class: "relative z-10 border-t border-white-5 bg-black-20 px-6 py-3 flex items-center justify-between",

                // Status Text
                div {
                    class: "text-[10px] font-bold uppercase tracking-wider text-zinc-600",
                     if running { span { class: "text-green-500/80", "• Active" } } else { span { "• Idle" } }
                }

                div {
                    class: "flex items-center gap-2",

                    if props.server.server_type == "stdio" {
                        button {
                            class: "p-2 rounded-lg text-zinc-400 hover:text-white hover:bg-white-8 transition-colors",
                            onclick: move |_| (props.on_console_click)(()),
                            title: "Open Console",
                            svg { class: "w-4 h-4", fill: "none", view_box: "0 0 24 24", stroke: "currentColor", stroke_width: "2",
                                path { stroke_linecap: "round", stroke_linejoin: "round", d: "M4 6h16M4 12h16m-7 6h7" }
                            }
                        }
                    }

                    button {
                        class: "p-2 rounded-lg text-zinc-400 hover:text-red-400 hover:bg-white-8 transition-colors",
                        onclick: restart_server,
                        title: "Restart",
                        svg { class: "w-4 h-4", fill: "none", view_box: "0 0 24 24", stroke: "currentColor", stroke_width: "2",
                            path { stroke_linecap: "round", stroke_linejoin: "round", d: "M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" }
                        }
                    }

                    button {
                        class: "p-2 rounded-lg text-zinc-400 hover:text-white hover:bg-white-8 transition-colors",
                        onclick: move |_| (props.on_edit_click)(()),
                        title: "Settings",
                        svg { class: "w-4 h-4", fill: "none", view_box: "0 0 24 24", stroke: "currentColor", stroke_width: "2",
                            path { stroke_linecap: "round", stroke_linejoin: "round", d: "M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z" }
                            path { stroke_linecap: "round", stroke_linejoin: "round", d: "M15 12a3 3 0 11-6 0 3 3 0 016 0z" }
                        }
                    }
                }
            }
        }
    }
}
