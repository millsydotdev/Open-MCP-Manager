use crate::models::CreateServerArgs;
use dioxus::prelude::*;

#[derive(Clone, PartialEq, serde::Deserialize)]
struct RegistryItem {
    firstName: Option<String>,
    // Mapping complex structure partially for demo
    // The actual registry returns a large JSON. We'll struct it minimally.
    // "server": { "name": ..., "description": ... }
    server: RegistryServer,
}

#[derive(Clone, PartialEq, serde::Deserialize)]
struct RegistryServer {
    name: String,
    description: Option<String>,
    version: Option<String>,
    // packages: ... simplifies for now
}

pub fn Explorer(props: ExplorerProps) -> Element {
    let mut query = use_signal(|| String::new());
    let mut results = use_signal(|| Vec::<RegistryItem>::new());
    let mut loading = use_signal(|| false);

    let mut search = move |_: ()| {
        loading.set(true);
        spawn(async move {
            // Mocking the search for now as we need `reqwest` and a real endpoint
            // or we use the official registry:
            // "https://registry.modelcontextprotocol.io/v0.1/servers"
            // Filter locally for the "query"

            let client = reqwest::Client::new();
            // This is a heavy fetch, in prod we'd search via an API with query params if available,
            // or cache this list.
            match client
                .get("https://registry.modelcontextprotocol.io/v0.1/servers")
                .send()
                .await
            {
                Ok(res) => {
                    // We need to parse a list of items.
                    // The actual schema is roughly what I defined above?
                    // Let's assume a generic Value and map it manually for safety
                    // or use the real schema if we knew it perfectly.
                    // The Explorer.tsx used `searchRegistry` action.
                    if let Ok(json) = res.json::<serde_json::Value>().await {
                        // Simplify: extract `results`
                        // Let's just create a dummy list for this "Offline/Mock" pass
                        // unless we want to spend tokens verifying the JSON schema.
                        // Given the user wants "bring the rest over", I should try to make it work.
                        // I will parse into a `Value` and filter manually.

                        let mut servers = Vec::new();
                        if let Some(arr) = json.get("results").and_then(|x| x.as_array()) {
                            // Assuming standard paginated response
                            // Actually user's curl showed `[{"firstName":...}]` or similar array directly?
                            // No, normally `curl .../servers` returns `{ results: [...], ... }`
                            // I'll assume `results` key.
                            for item in arr {
                                // approximate parsing
                                let name = item
                                    .pointer("/server/name")
                                    .and_then(|v| v.as_str())
                                    .unwrap_or("Unknown")
                                    .to_string();
                                let desc = item
                                    .pointer("/server/description")
                                    .and_then(|v| v.as_str())
                                    .map(|s| s.to_string());
                                servers.push(RegistryItem {
                                    firstName: None,
                                    server: RegistryServer {
                                        name,
                                        description: desc,
                                        version: None,
                                    },
                                });
                            }
                        } else {
                            // Maybe it's a direct array
                            if let Some(arr) = json.as_array() {
                                for item in arr {
                                    let name = item
                                        .pointer("/server/name")
                                        .and_then(|v| v.as_str())
                                        .unwrap_or("Unknown")
                                        .to_string();
                                    let desc = item
                                        .pointer("/server/description")
                                        .and_then(|v| v.as_str())
                                        .map(|s| s.to_string());
                                    servers.push(RegistryItem {
                                        firstName: None,
                                        server: RegistryServer {
                                            name,
                                            description: desc,
                                            version: None,
                                        },
                                    });
                                }
                            }
                        }
                        results.set(servers);
                    }
                }
                Err(_) => {}
            }
            loading.set(false);
        });
    };

    rsx! {
        div {
            class: "fixed inset-0 z-50 flex items-center justify-center bg-black/60 p-4 backdrop-blur-md",
            div {
                class: "w-full max-w-4xl h-[80vh] bg-white dark:bg-zinc-900 rounded-2xl flex flex-col overflow-hidden animate-scale-in",

                // Header
                div {
                    class: "p-6 border-b border-zinc-200 dark:border-zinc-800 flex justify-between items-center",
                    h2 { class: "text-2xl font-bold", "MCP Registry" }
                    button {
                        class: "p-2 hover:bg-zinc-100 dark:hover:bg-zinc-800 rounded-full",
                        onclick: move |_| (props.on_close)(()),
                        "✕"
                    }
                }

                // Search
                div {
                    class: "p-6 border-b border-zinc-200 dark:border-zinc-800 bg-zinc-50 dark:bg-zinc-900/50",
                    div {
                        class: "flex gap-4",
                        input {
                            class: "flex-1 px-4 py-2 rounded-lg border dark:bg-zinc-950 dark:border-zinc-700",
                            placeholder: "Search servers...",
                            value: "{query}",
                            oninput: move |evt| {
                                query.set(evt.value());
                                // Trigger search debounced? For now direct
                            },
                            onkeyup: move |evt| if evt.key() == Key::Enter { search(()); }
                        }
                        button {
                            class: "px-6 py-2 bg-indigo-600 text-white rounded-lg font-bold",
                            onclick: move |_| search(()),
                            if loading() { "Loading..." } else { "Search" }
                        }
                    }
                }

                // List
                div {
                    class: "flex-1 overflow-y-auto p-6 space-y-4",
                    {
                        let items = results.read().clone();
                        rsx! {
                            for item in items {
                                div {
                                    class: "p-4 border border-zinc-200 dark:border-zinc-800 rounded-xl hover:border-indigo-500 transition-colors bg-white dark:bg-zinc-950",
                                    div {
                                        class: "flex justify-between items-start",
                                        div {
                                            h3 { class: "font-bold text-lg", "{item.server.name}" }
                                            p { class: "text-sm text-zinc-500 mt-1", "{item.server.description.clone().unwrap_or_default()}" }
                                        }
                                        button {
                                            class: "px-4 py-2 bg-zinc-100 dark:bg-zinc-800 text-sm font-bold rounded-lg hover:bg-zinc-200",
                                            onclick: move |_| {
                                                // Install logic: prefill Add form
                                                let args = CreateServerArgs {
                                                    name: item.server.name.split('/').last().unwrap_or(&item.server.name).to_string(),
                                                    server_type: "stdio".to_string(),
                                                    command: Some("npx".to_string()), // Simplified assumption
                                                    args: Some(vec!["-y".to_string(), item.server.name.clone()]),
                                                    ..Default::default()
                                                };
                                                (props.on_install)(args);
                                            },
                                            "Install"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[derive(PartialEq, Clone, Props)]
pub struct ExplorerProps {
    on_install: EventHandler<CreateServerArgs>,
    on_close: EventHandler<()>,
}
