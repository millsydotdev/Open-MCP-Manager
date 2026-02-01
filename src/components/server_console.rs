use crate::models::{McpServer, Prompt, Resource, Tool};
use crate::state::AppState;
use crate::state::APP_STATE;
use dioxus::prelude::*;
use serde_json;

#[derive(PartialEq, Clone, Props)]
pub struct ServerConsoleProps {
    server: McpServer,
    on_close: EventHandler<()>,
}

#[derive(Clone, PartialEq)]
enum Tab {
    Logs,
    Tools,
    Resources,
    Prompts,
}

pub fn ServerConsole(props: ServerConsoleProps) -> Element {
    let mut active_tab = use_signal(|| Tab::Logs);
    let mut active_tool = use_signal(|| None::<Tool>);
    let mut tool_args = use_signal(|| "{}".to_string());
    let mut tool_output = use_signal(|| None::<String>);
    let mut tool_error = use_signal(|| false);
    let mut active_resource_content = use_signal(|| None::<(String, String)>); // (uri, content)

    let mut tools_list = use_signal(|| Vec::<Tool>::new());
    let mut resources_list = use_signal(|| Vec::<Resource>::new());
    let mut prompts_list = use_signal(|| Vec::<Prompt>::new());
    let mut error_msg = use_signal(|| None::<String>);
    let mut is_loading = use_signal(|| false);
    let mut ping_result = use_signal(|| None::<Result<u128, String>>);

    // Access the global processes map to find the signal for this server's logs
    let processes = APP_STATE.read().processes.clone();
    let srv_id = props.server.id.clone();
    let log_signal = use_memo(move || {
        let map = processes.read();
        map.get(&srv_id).cloned()
    });

    let log_text = if let Some(sig) = log_signal() {
        sig.read().clone()
    } else {
        "Process not running or no logs yet.".to_string()
    };

    let status_text = if log_signal().is_some() {
        "Connected"
    } else {
        "Disconnected"
    };

    let srv_id_tools = props.server.id.clone();
    let fetch_tools = move |_| {
        let id_val = srv_id_tools.clone();
        spawn(async move {
            match AppState::get_tools(id_val).await {
                Ok(t) => {
                    tools_list.set(t);
                    error_msg.set(None);
                }
                Err(e) => error_msg.set(Some(e)),
            }
        });
    };

    let srv_id_resources = props.server.id.clone();
    let fetch_resources = move |_| {
        let id_val = srv_id_resources.clone();
        spawn(async move {
            match AppState::get_resources(id_val).await {
                Ok(r) => {
                    resources_list.set(r);
                    error_msg.set(None);
                }
                Err(e) => error_msg.set(Some(e)),
            }
        });
    };

    let srv_id_prompts = props.server.id.clone();
    let fetch_prompts = move |_| {
        let id_val = srv_id_prompts.clone();
        spawn(async move {
            match AppState::get_prompts(id_val).await {
                Ok(p) => {
                    prompts_list.set(p);
                    error_msg.set(None);
                }
                Err(e) => error_msg.set(Some(e)),
            }
        });
    };

    let srv_id_exec = props.server.id.clone();
    let execute_tool = move |_| {
        let id_val = srv_id_exec.clone();
        let t_name = active_tool()
            .as_ref()
            .map(|t| t.name.clone())
            .unwrap_or_default();
        let t_args_str = tool_args();

        is_loading.set(true);
        tool_output.set(None);
        tool_error.set(false);

        spawn(async move {
            let args_json: serde_json::Value = match serde_json::from_str(&t_args_str) {
                Ok(v) => v,
                Err(e) => {
                    tool_output.set(Some(format!("Invalid JSON: {}", e)));
                    tool_error.set(true);
                    is_loading.set(false);
                    return;
                }
            };

            match AppState::execute_tool(id_val, t_name, args_json).await {
                Ok(res) => {
                    // Combine all content parts
                    let mut output = String::new();
                    for content in res.content {
                        if let Some(text) = content.text {
                            output.push_str(&text);
                            output.push('\n');
                        } else if let Some(data) = content.data {
                            output.push_str(&format!(
                                "[Base64 Data: {}...]\n",
                                data.chars().take(50).collect::<String>()
                            ));
                        }
                    }
                    tool_output.set(Some(output));
                    if let Some(is_err) = res.isError {
                        tool_error.set(is_err);
                    }
                }
                Err(e) => {
                    tool_output.set(Some(e));
                    tool_error.set(true);
                }
            }
            is_loading.set(false);
        });
    };

    let srv_id_read = props.server.id.clone();
    let srv_id_ping = props.server.id.clone();

    let test_connection = move |_| {
        let id_val = srv_id_ping.clone();
        ping_result.set(None);
        spawn(async move {
            let res = AppState::ping_server(id_val).await;
            ping_result.set(Some(res));
        });
    };

    let current_tab = active_tab.read().clone();
    let current_tool = active_tool.read().clone();
    let current_resource = active_resource_content.read().clone();

    let active_class = "px-4 py-2 text-sm font-medium transition-colors text-white border-b-2 border-indigo-500 bg-zinc-800/50";
    let inactive_class =
        "px-4 py-2 text-sm font-medium transition-colors text-zinc-500 hover:text-zinc-300";

    rsx! {
        div { class: "fixed inset-0 z-50 flex items-center justify-center bg-black/60 p-4 backdrop-blur-md",
            div { class: "w-full max-w-5xl h-[80vh] bg-zinc-950 text-zinc-300 rounded-2xl flex flex-col overflow-hidden border border-zinc-800 shadow-2xl relative animate-scale-in",

                // Header
                div { class: "flex justify-between items-center p-4 bg-zinc-900 border-b border-zinc-800",
                    div { class: "flex items-center gap-3",
                        span { class: "p-2 bg-indigo-500/20 text-indigo-400 rounded-lg", "💻" }
                        div {
                            h2 { class: "font-bold text-white", "{props.server.name}" }
                            span { class: "text-xs font-mono text-zinc-500", "{props.server.id}" }
                        }
                    }
                    div { class: "flex items-center gap-2",
                        if let Some(res) = ping_result() {
                             match res {
                                 Ok(ms) => rsx! { span { class: "text-green-400 text-xs font-bold mr-2 animate-pulse", "🟢 {ms}ms" } },
                                 Err(_) => rsx! { span { class: "text-red-400 text-xs font-bold mr-2", "🔴 Failed" } },
                             }
                         }
                        button {
                            class: "px-3 py-1 bg-zinc-800 hover:bg-zinc-700 text-zinc-300 rounded text-xs font-bold mr-2 border border-zinc-700 transition-colors",
                            onclick: test_connection,
                            if ping_result().is_none() { "Test Connection" } else { "Retest" }
                        }
                        button {
                            class: "p-2 hover:bg-zinc-800 rounded-full text-zinc-400 hover:text-white transition-colors",
                            onclick: move |_| props.on_close.call(()),
                            "✕"
                        }
                    }
                }

                // Tabs
                div { class: "flex border-b border-zinc-800 bg-zinc-900/50",
                    button {
                        class: if current_tab == Tab::Logs { active_class } else { inactive_class },
                        onclick: move |_| active_tab.set(Tab::Logs),
                        "Logs"
                    }
                    button {
                        class: if current_tab == Tab::Tools { active_class } else { inactive_class },
                        onclick: move |_| {
                            active_tab.set(Tab::Tools);
                            fetch_tools(());
                        },
                        "Tools"
                    }
                    button {
                        class: if current_tab == Tab::Resources { active_class } else { inactive_class },
                        onclick: move |_| {
                            active_tab.set(Tab::Resources);
                            fetch_resources(());
                        },
                        "Resources"
                    }
                    button {
                        class: if current_tab == Tab::Prompts { active_class } else { inactive_class },
                        onclick: move |_| {
                            active_tab.set(Tab::Prompts);
                            fetch_prompts(());
                        },
                        "Prompts"
                    }
                }

                // Error Banner
                if let Some(err) = error_msg() {
                    div { class: "bg-red-500/10 text-red-400 px-4 py-2 text-sm border-b border-red-500/20 flex justify-between",
                        "{err}"
                        button { onclick: move |_| error_msg.set(None), "✕" }
                    }
                }

                // Content Area
                div { class: "flex-1 overflow-auto bg-zinc-950",
                    if current_tab == Tab::Logs {
                        div { class: "p-4 font-mono text-xs whitespace-pre-wrap text-zinc-400", "{log_text}" }
                    } else if current_tab == Tab::Tools {
                         div { class: "p-4 grid gap-4",
                            for tool in tools_list() {
                                div { class: "p-4 border border-zinc-800 rounded-xl bg-zinc-900/50",
                                    div { class: "flex justify-between items-start mb-2",
                                        h3 { class: "font-bold text-white", "{tool.name}" }
                                        button {
                                            class: "px-3 py-1 bg-indigo-600 hover:bg-indigo-500 text-white rounded text-xs font-bold",
                                            onclick: move |_| {
                                                tool_error.set(false);
                                                tool_output.set(None);
                                                tool_args.set("{}".to_string());
                                                active_tool.set(Some(tool.clone()));
                                            },
                                            "Call"
                                        }
                                    }
                                    p { class: "text-sm text-zinc-400 mb-3", "{tool.description.clone().unwrap_or_default()}" }
                                    div { class: "bg-black/50 p-2 rounded border border-zinc-800 font-mono text-xs text-zinc-500 overflow-x-auto",
                                        "{serde_json::to_string_pretty(&tool.inputSchema).unwrap_or_default()}"
                                    }
                                }
                            }
                            if tools_list().is_empty() {
                                div { class: "text-center text-zinc-500 py-10", "No tools found or not fetched." }
                            }
                        }
                    } else if current_tab == Tab::Resources {
                        div { class: "p-4 grid gap-4",
                             for res in resources_list() {
                                div { class: "p-4 border border-zinc-800 rounded-xl bg-zinc-900/50",
                                    h3 { class: "font-bold text-white mb-1", "{res.name}" }
                                    div { class: "flex items-center gap-2 text-xs text-zinc-500 mb-2 font-mono",
                                        span { class: "px-1.5 py-0.5 bg-zinc-800 rounded", "{res.mimeType.clone().unwrap_or(\"unknown\".into())}" }
                                        "{res.uri}"
                                    }
                                    p { class: "text-sm text-zinc-400", "{res.description.clone().unwrap_or_default()}" }
                                    button {
                                        class: "mt-3 px-3 py-1 bg-zinc-800 hover:bg-zinc-700 text-zinc-300 rounded text-xs font-bold",
                                        onclick: {
                                            let uri = res.uri.clone();
                                            let id_val = srv_id_read.clone();
                                            move |_| {
                                                let uri_clone = uri.clone();
                                                let id_val_clone = id_val.clone();
                                                is_loading.set(true);
                                                spawn(async move {
                                                    match AppState::read_resource(id_val_clone, uri_clone.clone()).await {
                                                        Ok(res) => {
                                                            if let Some(content) = res.contents.first() {
                                                                if let Some(text) = &content.text {
                                                                    active_resource_content.set(Some((uri_clone, text.clone())));
                                                                } else if let Some(blob) = &content.blob {
                                                                    active_resource_content.set(Some((
                                                                        uri_clone,
                                                                        format!("[Base64 Blob: {}...]", blob.chars().take(50).collect::<String>()),
                                                                    )));
                                                                } else {
                                                                    active_resource_content.set(Some((uri_clone, "Empty content".into())));
                                                                }
                                                            } else {
                                                                active_resource_content.set(Some((uri_clone, "No content returned".into())));
                                                            }
                                                        }
                                                        Err(e) => {
                                                            error_msg.set(Some(format!("Failed to read resource: {}", e)));
                                                        }
                                                    }
                                                    is_loading.set(false);
                                                });
                                            }
                                        },
                                        "Read Resource"
                                    }
                                }
                            }
                            if resources_list().is_empty() {
                                div { class: "text-center text-zinc-500 py-10", "No resources found or not fetched." }
                            }
                        }
                    } else if current_tab == Tab::Prompts {
                        div { class: "p-4 grid gap-4",
                             for prompt in prompts_list() {
                                div { class: "p-4 border border-zinc-800 rounded-xl bg-zinc-900/50",
                                    h3 { class: "font-bold text-white mb-1", "{prompt.name}" }
                                    p { class: "text-sm text-zinc-400", "{prompt.description.clone().unwrap_or_default()}" }
                                    if let Some(args) = &prompt.arguments {
                                        div { class: "mt-2",
                                            span { class: "text-xs font-bold text-zinc-500 uppercase", "Arguments" }
                                            ul { class: "list-disc list-inside text-xs text-zinc-400 font-mono",
                                                for arg in args {
                                                    li {
                                                        "{arg.name} "
                                                        if arg.required.unwrap_or(false) {
                                                            "(required)"
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    button { class: "mt-3 px-3 py-1 bg-zinc-800 hover:bg-zinc-700 text-zinc-300 rounded text-xs font-bold", "Get Prompt" }
                                }
                            }
                            if prompts_list().is_empty() {
                                div { class: "text-center text-zinc-500 py-10", "No prompts found or not fetched." }
                            }
                        }
                    }
                }

                // Footer
                div { class: "p-2 bg-zinc-900 border-t border-zinc-800 text-xs text-zinc-500 flex justify-between",
                    span { "Status: {status_text}" }
                    if current_tab == Tab::Logs {
                        button { class: "hover:text-white", "Clear Logs" }
                    }
                }

                // Tool Execution Modal Overlay
                if let Some(tool) = current_tool {
                    div { class: "absolute inset-0 z-50 bg-black/80 flex items-center justify-center p-8 backdrop-blur-sm",
                        div { class: "w-full max-w-2xl bg-zinc-900 border border-zinc-700 rounded-xl shadow-2xl flex flex-col max-h-full animate-scale-in",
                            div { class: "p-4 border-b border-zinc-800 flex justify-between items-center",
                                h3 { class: "font-bold text-white", "Execute: {tool.name}" }
                                button { class: "text-zinc-500 hover:text-white", onclick: move |_| active_tool.set(None), "✕" }
                            }
                            div { class: "p-4 flex-1 overflow-auto",
                                label { class: "block text-xs font-bold text-zinc-400 mb-2 uppercase", "Arguments (JSON)" }
                                textarea {
                                    class: "w-full h-40 bg-black/50 border border-zinc-700 rounded p-3 font-mono text-sm text-zinc-300 focus:border-indigo-500 focus:outline-none resize-none",
                                    value: "{tool_args}",
                                    oninput: move |evt| tool_args.set(evt.value())
                                }

                                if let Some(res) = tool_output() {
                                    div { class: "mt-4",
                                        label { class: "block text-xs font-bold text-zinc-400 mb-2 uppercase",
                                            if tool_error() { "Error" } else { "Result" }
                                        }
                                        div { class: "p-3 rounded border font-mono text-sm whitespace-pre-wrap overflow-x-auto",
                                            class: if tool_error() { "bg-red-950/30 border-red-900 text-red-300" } else { "bg-green-950/30 border-green-900 text-green-300" },
                                            "{res}"
                                        }
                                    }
                                }
                            }
                            div { class: "p-4 border-t border-zinc-800 bg-zinc-900 flex justify-end gap-2",
                                button {
                                    class: "px-4 py-2 bg-zinc-800 hover:bg-zinc-700 text-white rounded text-sm",
                                    onclick: move |_| active_tool.set(None),
                                    "Close"
                                }
                                button {
                                    class: "px-4 py-2 bg-indigo-600 hover:bg-indigo-500 text-white rounded text-sm font-bold disabled:opacity-50 disabled:cursor-not-allowed",
                                    disabled: is_loading(),
                                    onclick: execute_tool,
                                    if is_loading() { "Running..." } else { "Run Tool" }
                                }
                            }
                        }
                    }
                }

                // Resource Viewer Modal Overlay
                if let Some((uri, content)) = current_resource {
                     div { class: "absolute inset-0 z-50 bg-black/80 flex items-center justify-center p-8 backdrop-blur-sm",
                        div { class: "w-full max-w-3xl bg-zinc-900 border border-zinc-700 rounded-xl shadow-2xl flex flex-col h-[70vh] animate-scale-in",
                            div { class: "p-4 border-b border-zinc-800 flex justify-between items-center",
                                div {
                                    h3 { class: "font-bold text-white", "Resource Content" }
                                    span { class: "text-xs font-mono text-zinc-500", "{uri}" }
                                }
                                button { class: "text-zinc-500 hover:text-white", onclick: move |_| active_resource_content.set(None), "✕" }
                            }
                            div { class: "p-0 flex-1 overflow-auto bg-black/30",
                                pre { class: "p-4 font-mono text-sm text-zinc-300 whitespace-pre-wrap", "{content}" }
                            }
                             div { class: "p-4 border-t border-zinc-800 bg-zinc-900 flex justify-end",
                                button {
                                    class: "px-4 py-2 bg-zinc-800 hover:bg-zinc-700 text-white rounded text-sm",
                                    onclick: move |_| active_resource_content.set(None),
                                    "Close"
                                }
                            }
                        }
                    }
                }

            }
        }
    }
}
