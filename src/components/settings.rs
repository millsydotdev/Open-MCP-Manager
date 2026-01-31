use crate::models::{CreateServerArgs, McpServer};
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct SettingsProps {
    pub server: Option<McpServer>,
    pub on_close: EventHandler<()>,
    pub on_save: EventHandler<CreateServerArgs>,
    pub on_delete: EventHandler<String>,
}

#[derive(Clone, Copy, PartialEq)]
enum ServerType {
    Stdio,
    Sse,
}

pub fn Settings(props: SettingsProps) -> Element {
    let is_edit = props.server.is_some();

    // Form state
    let mut server_type = use_signal(|| {
        props
            .server
            .as_ref()
            .map(|s| {
                if s.server_type == "sse" {
                    ServerType::Sse
                } else {
                    ServerType::Stdio
                }
            })
            .unwrap_or(ServerType::Stdio)
    });

    let mut name = use_signal(|| {
        props
            .server
            .as_ref()
            .map(|s| s.name.clone())
            .unwrap_or_default()
    });

    let mut description = use_signal(|| {
        props
            .server
            .as_ref()
            .and_then(|s| s.description.clone())
            .unwrap_or_default()
    });

    let mut command = use_signal(|| {
        props
            .server
            .as_ref()
            .and_then(|s| s.command.clone())
            .unwrap_or_default()
    });

    let mut url = use_signal(|| {
        props
            .server
            .as_ref()
            .and_then(|s| s.url.clone())
            .unwrap_or_default()
    });

    // Arguments as Vec<String>
    let mut args_list = use_signal(|| {
        props
            .server
            .as_ref()
            .and_then(|s| s.args.clone())
            .unwrap_or_default()
    });
    let mut arg_input = use_signal(String::new);

    // Env as HashMap<String, String>
    let mut env_map = use_signal(|| {
        props
            .server
            .as_ref()
            .and_then(|s| s.env.clone())
            .unwrap_or_default()
    });
    let mut env_key_input = use_signal(String::new);
    let mut env_value_input = use_signal(String::new);

    // Add argument
    let add_arg = move |_| {
        let val = arg_input().trim().to_string();
        if !val.is_empty() {
            args_list.write().push(val);
            arg_input.set(String::new());
        }
    };

    // Add env var
    let add_env = move |_| {
        let key = env_key_input().trim().to_string();
        let value = env_value_input().trim().to_string();
        if !key.is_empty() {
            env_map.write().insert(key, value);
            env_key_input.set(String::new());
            env_value_input.set(String::new());
        }
    };

    let onsubmit = move |_| {
        let st = server_type();
        let type_str = match st {
            ServerType::Stdio => "stdio".to_string(),
            ServerType::Sse => "sse".to_string(),
        };

        let final_args = {
            let a = args_list();
            if a.is_empty() {
                None
            } else {
                Some(a)
            }
        };

        let final_env = {
            let e = env_map();
            if e.is_empty() {
                None
            } else {
                Some(e)
            }
        };

        let cmd_val = command();
        let final_command = if cmd_val.trim().is_empty() {
            None
        } else {
            Some(cmd_val)
        };

        let url_val = url();
        let final_url = if url_val.trim().is_empty() {
            None
        } else {
            Some(url_val)
        };

        let desc_val = description();
        let final_desc = if desc_val.trim().is_empty() {
            None
        } else {
            Some(desc_val)
        };

        (props.on_save)(CreateServerArgs {
            name: name(),
            server_type: type_str,
            command: final_command,
            args: final_args,
            env: final_env,
            url: final_url,
            description: final_desc,
        });
    };

    let title = if is_edit {
        "Edit Server"
    } else {
        "Add New Server"
    };

    let current_type = server_type();
    let current_args = args_list();
    let current_env: Vec<(String, String)> = env_map().into_iter().collect();

    rsx! {
        div {
            class: "fixed inset-0 z-50 flex items-center justify-center bg-black/60 p-4 backdrop-blur-md",
            div {
                class: "w-full max-w-2xl bg-zinc-950 text-zinc-300 rounded-2xl flex flex-col overflow-hidden border border-zinc-800 shadow-2xl animate-scale-in",

                // Header
                div {
                    class: "flex justify-between items-center p-5 bg-zinc-900 border-b border-zinc-800",
                    div {
                        h2 { class: "font-bold text-xl text-white", "{title}" }
                        p { class: "text-xs text-zinc-500 mt-1", "Configure your MCP server instance" }
                    }
                    button {
                        class: "p-2 hover:bg-zinc-800 rounded-full transition-colors",
                        onclick: move |_| (props.on_close)(()),
                        "✕"
                    }
                }

                // Form
                div {
                    class: "p-6 space-y-5 overflow-y-auto max-h-[65vh]",

                    // Server Type Toggle
                    div {
                        class: "flex gap-2 p-1 bg-zinc-900 rounded-xl",
                        button {
                            class: if current_type == ServerType::Stdio { "flex-1 flex items-center justify-center gap-2 py-2.5 text-sm font-bold rounded-lg bg-zinc-800 text-indigo-400 shadow-lg transition-all" } else { "flex-1 flex items-center justify-center gap-2 py-2.5 text-sm font-bold rounded-lg text-zinc-500 hover:text-zinc-300 transition-all" },
                            onclick: move |_| server_type.set(ServerType::Stdio),
                            "⌨ stdio (Local)"
                        }
                        button {
                            class: if current_type == ServerType::Sse { "flex-1 flex items-center justify-center gap-2 py-2.5 text-sm font-bold rounded-lg bg-zinc-800 text-indigo-400 shadow-lg transition-all" } else { "flex-1 flex items-center justify-center gap-2 py-2.5 text-sm font-bold rounded-lg text-zinc-500 hover:text-zinc-300 transition-all" },
                            onclick: move |_| server_type.set(ServerType::Sse),
                            "🌐 sse (Remote)"
                        }
                    }

                    // Name
                    div {
                        label { class: "block text-sm font-bold mb-2 text-zinc-400", "Name" }
                        input {
                            class: "w-full px-4 py-2.5 bg-zinc-900 border border-zinc-700 rounded-xl focus:outline-none focus:border-indigo-500 transition-colors",
                            placeholder: "e.g. github-mcp",
                            value: "{name}",
                            oninput: move |evt| name.set(evt.value())
                        }
                    }

                    // Description
                    div {
                        label { class: "block text-sm font-bold mb-2 text-zinc-400", "Description" }
                        textarea {
                            class: "w-full px-4 py-2.5 bg-zinc-900 border border-zinc-700 rounded-xl focus:outline-none focus:border-indigo-500 transition-colors resize-none h-20",
                            placeholder: "What does this server do?",
                            value: "{description}",
                            oninput: move |evt| description.set(evt.value())
                        }
                    }

                    // Conditional: Stdio or SSE fields
                    if current_type == ServerType::Stdio {
                        // Command
                        div {
                            label { class: "block text-sm font-bold mb-2 text-zinc-400", "Command" }
                            input {
                                class: "w-full px-4 py-2.5 bg-zinc-900 border border-zinc-700 rounded-xl focus:outline-none focus:border-indigo-500 transition-colors font-mono",
                                placeholder: "e.g. npx, node, python, uvx",
                                value: "{command}",
                                oninput: move |evt| command.set(evt.value())
                            }
                        }

                        // Arguments
                        div {
                            label { class: "block text-sm font-bold mb-2 text-zinc-400", "Arguments" }
                            div { class: "flex gap-2",
                                input {
                                    class: "flex-1 px-4 py-2.5 bg-zinc-900 border border-zinc-700 rounded-xl focus:outline-none focus:border-indigo-500 transition-colors",
                                    placeholder: "Add argument...",
                                    value: "{arg_input}",
                                    oninput: move |evt| arg_input.set(evt.value()),
                                    onkeypress: move |evt| {
                                        if evt.key() == Key::Enter {
                                            let val = arg_input().trim().to_string();
                                            if !val.is_empty() {
                                                args_list.write().push(val);
                                                arg_input.set(String::new());
                                            }
                                        }
                                    }
                                }
                                button {
                                    class: "px-4 py-2.5 bg-zinc-800 hover:bg-zinc-700 text-zinc-400 rounded-xl transition-colors",
                                    onclick: add_arg,
                                    "+"
                                }
                            }
                            div { class: "flex flex-wrap gap-2 mt-3",
                                for (i, arg) in current_args.iter().enumerate() {
                                    span {
                                        key: "{i}",
                                        class: "inline-flex items-center gap-2 px-3 py-1.5 bg-indigo-500/10 text-indigo-400 rounded-lg text-xs font-semibold",
                                        "{arg}"
                                        button {
                                            class: "hover:text-white transition-colors",
                                            onclick: {
                                                let idx = i;
                                                move |_| {
                                                    args_list.write().remove(idx);
                                                }
                                            },
                                            "×"
                                        }
                                    }
                                }
                            }
                        }
                    } else {
                        // URL for SSE
                        div {
                            label { class: "block text-sm font-bold mb-2 text-zinc-400", "SSE Endpoint URL" }
                            input {
                                class: "w-full px-4 py-2.5 bg-zinc-900 border border-zinc-700 rounded-xl focus:outline-none focus:border-indigo-500 transition-colors font-mono",
                                placeholder: "https://example.com/mcp",
                                value: "{url}",
                                oninput: move |evt| url.set(evt.value())
                            }
                            p { class: "mt-2 text-xs text-zinc-500", "The server must support SSE transport." }
                        }
                    }

                    // Environment Variables
                    div {
                        label { class: "block text-sm font-bold mb-2 text-zinc-400", "Environment Variables" }
                        div { class: "flex gap-2",
                            input {
                                class: "w-1/3 px-4 py-2.5 bg-zinc-900 border border-zinc-700 rounded-xl focus:outline-none focus:border-indigo-500 transition-colors font-mono text-xs",
                                placeholder: "KEY",
                                value: "{env_key_input}",
                                oninput: move |evt| env_key_input.set(evt.value())
                            }
                            input {
                                class: "flex-1 px-4 py-2.5 bg-zinc-900 border border-zinc-700 rounded-xl focus:outline-none focus:border-indigo-500 transition-colors font-mono text-xs",
                                placeholder: "VALUE",
                                value: "{env_value_input}",
                                oninput: move |evt| env_value_input.set(evt.value())
                            }
                            button {
                                class: "px-4 py-2.5 bg-zinc-800 hover:bg-zinc-700 text-zinc-400 rounded-xl transition-colors",
                                onclick: add_env,
                                "+"
                            }
                        }
                        div { class: "grid gap-2 mt-3",
                            for (key, value) in current_env.iter() {
                                div {
                                    key: "{key}",
                                    class: "flex items-center justify-between p-3 bg-zinc-900 rounded-xl border border-zinc-800",
                                    div { class: "flex gap-4",
                                        div {
                                            span { class: "text-[10px] font-bold uppercase text-zinc-500 block", "KEY" }
                                            span { class: "font-mono text-sm font-bold text-indigo-400", "{key}" }
                                        }
                                        div {
                                            span { class: "text-[10px] font-bold uppercase text-zinc-500 block", "VALUE" }
                                            span { class: "font-mono text-sm text-zinc-300 truncate max-w-[200px]", "{value}" }
                                        }
                                    }
                                    button {
                                        class: "p-2 text-zinc-500 hover:text-red-400 hover:bg-red-500/10 rounded-lg transition-colors",
                                        onclick: {
                                            let k = key.clone();
                                            move |_| {
                                                env_map.write().remove(&k);
                                            }
                                        },
                                        "🗑"
                                    }
                                }
                            }
                        }
                    }
                }

                // Footer
                div {
                    class: "p-5 bg-zinc-900 border-t border-zinc-800 flex justify-end gap-3",
                    if is_edit {
                        button {
                            class: "px-4 py-2.5 bg-red-500/10 text-red-500 hover:bg-red-500/20 rounded-xl text-sm font-bold transition-colors mr-auto",
                            onclick: move |_| {
                                if let Some(s) = &props.server {
                                    (props.on_delete)(s.id.clone());
                                }
                            },
                            "Delete"
                        }
                    }
                    button {
                        class: "px-5 py-2.5 text-zinc-400 hover:text-white transition-colors",
                        onclick: move |_| (props.on_close)(()),
                        "Cancel"
                    }
                    button {
                        class: "px-6 py-2.5 bg-indigo-600 hover:bg-indigo-500 text-white rounded-xl text-sm font-bold transition-colors shadow-lg shadow-indigo-500/20",
                        onclick: onsubmit,
                        if is_edit { "Save Changes" } else { "Create Server" }
                    }
                }
            }
        }
    }
}
