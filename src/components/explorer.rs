use crate::models::{
    prepare_install_args, CreateServerArgs, RegistryInstallConfig, RegistryItem, RegistryServer,
    WizardAction, WizardStep,
};
use crate::state::APP_STATE;
use dioxus::prelude::*;

fn get_official_registry() -> Vec<RegistryItem> {
    vec![
        RegistryItem {
            server: RegistryServer {
                name: "Memory".to_string(),
                description: Some(
                    "Ephemeral memory server for storing capabilities and simple state."
                        .to_string(),
                ),
                homepage: None,
                bugs: None,
                version: Some("0.1.0".to_string()),
                category: Some("Featured".to_string()),
            },
            install_config: Some(RegistryInstallConfig {
                command: "npx".to_string(),
                args: vec![
                    "-y".to_string(),
                    "@modelcontextprotocol/server-memory".to_string(),
                ],
                env_template: None,
                wizard: None,
            }),
        },
        RegistryItem {
            server: RegistryServer {
                name: "Brave Search".to_string(),
                description: Some("Web search capabilities using Brave's API.".to_string()),
                homepage: None,
                bugs: None,
                version: Some("0.1.0".to_string()),
                category: Some("Web".to_string()),
            },
            install_config: Some(RegistryInstallConfig {
                command: "npx".to_string(),
                args: vec![
                    "-y".to_string(),
                    "@modelcontextprotocol/server-brave-search".to_string(),
                ],
                env_template: Some(std::collections::HashMap::from([(
                    "BRAVE_API_KEY".to_string(),
                    "YOUR_API_KEY_HERE".to_string(),
                )])),
                wizard: None,
            }),
        },
        RegistryItem {
            server: RegistryServer {
                name: "Google Drive".to_string(),
                description: Some("Access and manage Google Drive files.".to_string()),
                homepage: None,
                bugs: None,
                version: Some("0.1.0".to_string()),
                category: Some("Cloud".to_string()),
            },
            install_config: Some(RegistryInstallConfig {
                command: "npx".to_string(),
                args: vec![
                    "-y".to_string(),
                    "@modelcontextprotocol/server-gdrive".to_string(),
                ],
                env_template: None,
                wizard: Some(vec![
                    WizardStep {
                        title: "Get Credentials".to_string(),
                        description:
                            "You need to create a Google Cloud Project and enable the Drive API."
                                .to_string(),
                        action: WizardAction::Link {
                            url: "https://console.cloud.google.com/".to_string(),
                            label: "Open Google Cloud Console".to_string(),
                        },
                    },
                    WizardStep {
                        title: "Client ID".to_string(),
                        description: "Enter your OAuth Client ID.".to_string(),
                        action: WizardAction::Input {
                            key: "GOOGLE_CLIENT_ID".to_string(),
                            label: "Client ID".to_string(),
                            placeholder: Some("apps.googleusercontent.com".to_string()),
                        },
                    },
                    WizardStep {
                        title: "Client Secret".to_string(),
                        description: "Enter your OAuth Client Secret.".to_string(),
                        action: WizardAction::Input {
                            key: "GOOGLE_CLIENT_SECRET".to_string(),
                            label: "Client Secret".to_string(),
                            placeholder: None,
                        },
                    },
                ]),
            }),
        },
        RegistryItem {
            server: RegistryServer {
                name: "PostgreSQL".to_string(),
                description: Some("Read-only database access for PostgreSQL.".to_string()),
                homepage: None,
                bugs: None,
                version: Some("0.1.0".to_string()),
                category: Some("Database".to_string()),
            },
            install_config: Some(RegistryInstallConfig {
                command: "npx".to_string(),
                args: vec![
                    "-y".to_string(),
                    "@modelcontextprotocol/server-postgres".to_string(),
                    "postgresql://localhost/actions".to_string(),
                ],
                env_template: None,
                wizard: None,
            }),
        },
        RegistryItem {
            server: RegistryServer {
                name: "Git".to_string(),
                description: Some("Git repository interaction tools.".to_string()),
                homepage: None,
                bugs: None,
                version: Some("0.1.0".to_string()),
                category: Some("Developer".to_string()),
            },
            install_config: Some(RegistryInstallConfig {
                command: "python".to_string(),
                args: vec!["-m".to_string(), "mcp_server_git".to_string()],
                env_template: None,
                wizard: None,
            }),
        },

    ]
}

const GITHUB_API_URL: &str = "https://api.github.com/repos/modelcontextprotocol/servers/contents/src";

#[derive(serde::Deserialize, Debug)]
struct GitHubContent {
    name: String,
    #[serde(rename = "type")]
    content_type: String,
}

async fn fetch_dynamic_registry() -> Vec<RegistryItem> {
    let client = reqwest::Client::new();
    let mut items = get_official_registry(); // Start with local items including Wizards

    if let Ok(resp) = client.get(GITHUB_API_URL)
        .header("User-Agent", "Open-MCP-Manager")
        .send()
        .await 
    {
        if let Ok(contents) = resp.json::<Vec<GitHubContent>>().await {

            
            for content in contents {
                if content.content_type == "dir" {
                    // Skip if we already have a local config (e.g., Google Drive with Wizard)
                    
                    let pkg_name = format!("@modelcontextprotocol/server-{}", content.name);
                    
                    // Simple deduplication check: if any local item installs this package
                    let exists = items.iter().any(|i| {
                        if let Some(cfg) = &i.install_config {
                            cfg.args.contains(&pkg_name)
                        } else { false }
                    });

                    if !exists {
                         items.push(RegistryItem {
                            server: RegistryServer {
                                name: capitalize_first(&content.name),
                                description: Some(format!("Official MCP Server for {}", content.name)),
                                homepage: Some(format!("https://github.com/modelcontextprotocol/servers/tree/main/src/{}", content.name)),
                                bugs: None,
                                version: Some("latest".to_string()),
                                category: Some("Official".to_string()),
                            },
                            install_config: Some(RegistryInstallConfig {
                                command: "npx".to_string(),
                                args: vec!["-y".to_string(), pkg_name],
                                env_template: None,
                                wizard: None, 
                            }),
                        });
                    }
                }
            }
        }
    }
    items
}

fn capitalize_first(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

pub fn Explorer(props: ExplorerProps) -> Element {
    let mut query = use_signal(|| String::new());
    let mut all_items = use_signal(get_official_registry); // Start with local
    let mut results = use_signal(get_official_registry);   // Display local initially
    let mut loading = use_signal(|| true); // Start true, fetch will finish
    let mut url_input = use_signal(|| String::new());

    // Fetch Dynamic Registry
    use_future(move || async move {
        loading.set(true);
        let fresh_items = fetch_dynamic_registry().await;
        all_items.set(fresh_items.clone());
        results.set(fresh_items);
        loading.set(false);
    });

    // Wizard State
    let mut active_wizard_item = use_signal(|| None::<RegistryItem>);
    let mut active_wizard_step = use_signal(|| 0);
    // Stores the collected inputs. Key = Env Var Name, Value = User Input
    let mut wizard_env_data = use_signal(|| std::collections::HashMap::<String, String>::new());

    // Heuristic detection logic
    let detect_config_from_url = move |url: String| -> Option<CreateServerArgs> {
        let url_lower = url.to_lowercase();

        // 1. NPM Package: https://www.npmjs.com/package/pkg-name
        if url_lower.contains("npmjs.com/package/") {
            let pkg_name = url.split("package/").nth(1)?.split('/').next()?.to_string();
            return Some(CreateServerArgs {
                name: pkg_name.clone(),
                server_type: "stdio".to_string(),
                command: Some("npx".to_string()),
                args: Some(vec!["-y".to_string(), pkg_name]),
                ..Default::default()
            });
        }

        // 2. Official MCP Monorepo Deep Link: .../tree/main/src/gdrive
        if url_lower.contains("github.com/modelcontextprotocol/servers")
            && url_lower.contains("/src/")
        {
            let component = url.split("/src/").nth(1)?.split('/').next()?.to_string();
            let pkg_name = format!("@modelcontextprotocol/server-{}", component);
            return Some(CreateServerArgs {
                name: component, // e.g., "gdrive"
                server_type: "stdio".to_string(),
                command: Some("npx".to_string()),
                args: Some(vec!["-y".to_string(), pkg_name]),
                ..Default::default()
            });
        }

        // 3. Generic GitHub Repo: https://github.com/user/repo
        if url_lower.contains("github.com/") {
            let parts: Vec<&str> = url.split("github.com/").nth(1)?.split('/').collect();
            if parts.len() >= 2 {
                let repo_name = parts[1].trim_end_matches(".git").to_string();
                return Some(CreateServerArgs {
                    name: repo_name,
                    server_type: "stdio".to_string(),
                    command: Some("python".to_string()), // Guessing python for generic repos
                    args: Some(vec!["main.py".to_string()]),
                    description: Some(format!("Detected from {}", url)),
                    ..Default::default()
                });
            }
        }

        None
    };

    let install_from_url = move |_| {
        let u = url_input.read().clone();
        if let Some(args) = detect_config_from_url(u) {
            (props.on_install)(args);
        } else {
            println!("Could not detect config from URL");
        }
    };

    // Initialize results with official registry
    let mut search = move |_: ()| {
        loading.set(true);
        let q = query.read().to_lowercase();
        let all = all_items.read().clone(); 

        spawn(async move {
            let mut filtered = Vec::new();
            for item in all {
                if item.server.name.to_lowercase().contains(&q)
                    || item.server.description.as_ref().map(|d| d.to_lowercase().contains(&q)).unwrap_or(false)
                {
                    filtered.push(item)
                }
            }
            results.set(filtered);
            loading.set(false);
        });
    };

    // Wizard Overlay Logic
    let wizard_overlay = {
        let active_opt = active_wizard_item.read().clone();
        let step_idx = *active_wizard_step.read();

        if let Some(item) = active_opt {
            if let Some(config) = item.install_config {
                if let Some(steps) = config.wizard {
                    if let Some(step) = steps.get(step_idx) {
                        let item_name = item.server.name.clone();
                        let total_steps = steps.len();

                        rsx! {
                            div {
                                class: "absolute inset-0 z-50 bg-white dark:bg-zinc-900 flex flex-col p-8 animate-fade-in",
                                // Wizard Header
                                div {
                                    class: "flex justify-between items-center mb-8",
                                    div {
                                        h2 { class: "text-2xl font-bold", "{item_name} Setup" }
                                        p { class: "text-zinc-500", "Step {step_idx + 1} of {total_steps}" }
                                    }
                                    button {
                                        class: "p-2 hover:bg-zinc-100 dark:hover:bg-zinc-800 rounded-full",
                                        onclick: move |_| {
                                            active_wizard_item.set(None);
                                            active_wizard_step.set(0);
                                            wizard_env_data.write().clear();
                                        },
                                        "✕"
                                    }
                                }

                                // Wizard Content
                                div {
                                    class: "flex-1 flex flex-col items-center justify-center max-w-2xl mx-auto w-full gap-6 text-center",
                                    h3 { class: "text-xl font-bold", "{step.title}" }
                                    p { class: "text-zinc-600 dark:text-zinc-400 mb-4", "{step.description}" }

                                    {
                                        match &step.action {
                                            WizardAction::Link { url, label } => rsx! {
                                                a {
                                                    class: "px-6 py-3 bg-blue-600 text-white rounded-lg font-bold hover:bg-blue-700 flex items-center gap-2",
                                                    href: "{url}",
                                                    target: "_blank",
                                                    "{label}"
                                                }
                                            },
                                            WizardAction::Input { key, label, placeholder } => {
                                                let key = key.clone();
                                                rsx! {
                                                    div {
                                                        class: "w-full text-left",
                                                        label { class: "block text-sm font-bold mb-2", "{label}" }
                                                        input {
                                                            class: "w-full px-4 py-3 rounded-lg border dark:bg-zinc-950 dark:border-zinc-700",
                                                            placeholder: "{placeholder.clone().unwrap_or_default()}",
                                                            value: "{wizard_env_data.read().get(&key).cloned().unwrap_or_default()}",
                                                            oninput: move |evt| {
                                                                wizard_env_data.write().insert(key.clone(), evt.value());
                                                            }
                                                        }
                                                    }
                                                }
                                            },
                                            WizardAction::Message { text } => rsx! {
                                                div { class: "p-4 bg-zinc-100 dark:bg-zinc-800 rounded-lg", "{text}" }
                                            }
                                        }
                                    }
                                }

                                // Wizard Footer / Navigation
                                div {
                                    class: "mt-8 flex justify-end pt-6 border-t border-zinc-200 dark:border-zinc-800",
                                    {
                                        if step_idx < total_steps - 1 {
                                            rsx! {
                                                button {
                                                    class: "px-6 py-2 bg-indigo-600 text-white rounded-lg font-bold hover:bg-indigo-700",
                                                    onclick: move |_| {
                                                        active_wizard_step.with_mut(|s| *s += 1);
                                                    },
                                                    "Next Step →"
                                                }
                                            }
                                        } else {
                                            rsx! {
                                                button {
                                                    class: "px-6 py-2 bg-emerald-600 text-white rounded-lg font-bold hover:bg-emerald-700",
                                                    onclick: move |_| {
                                                        // Finish Wizard and Install
                                                         let current_item = active_wizard_item.peek().clone(); // Clone to drop borrow
                                                         if let Some(itm) = current_item {
                                                             let args = prepare_install_args(&itm, Some(&*wizard_env_data.read()));
                                                             (props.on_install)(args);
                                                         }

                                                        // Reset state
                                                        active_wizard_item.set(None);
                                                        active_wizard_step.set(0);
                                                        wizard_env_data.write().clear();
                                                    },
                                                    "Complete Setup & Install"
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    } else {
                        rsx! {}
                    }
                } else {
                    rsx! {}
                }
            } else {
                rsx! {}
            }
        } else {
            rsx! {}
        }
    };
    let items = results.read().clone();

    rsx! {
         div {
            class: "fixed inset-0 z-50 bg-black/50 backdrop-blur-sm flex items-center justify-center p-4",
            onclick: move |_| (props.on_close)(()),
            div {
                class: "bg-white dark:bg-zinc-900 w-full max-w-5xl h-[80vh] rounded-2xl shadow-2xl flex flex-col overflow-hidden animate-scale-up",
                onclick: move |evt| evt.stop_propagation(),

                // Header
                div {
                    class: "p-6 border-b border-zinc-200 dark:border-zinc-800 flex justify-between items-center bg-zinc-50 dark:bg-zinc-950",
                    div {
                        h2 { class: "text-2xl font-bold", "Discovery Registry" }
                        p { class: "text-zinc-500", "Find and install MCP servers" }
                    }
                    div {
                         class: "flex gap-2",
                         // URL Install Input
                         div {
                             class: "relative",
                             input {
                                 class: "pl-10 pr-4 py-2 w-64 rounded-lg border border-zinc-200 dark:border-zinc-700 bg-white dark:bg-zinc-900 focus:outline-none focus:ring-2 focus:ring-black dark:focus:ring-white",
                                 placeholder: "Install from URL...",
                                 value: "{url_input}",
                                 oninput: move |evt| url_input.set(evt.value()),
                                 onkeydown: move |evt| {
                                     if evt.key() == Key::Enter && !url_input.read().is_empty() {
                                         install_from_url(());
                                     }
                                 }
                             }
                             div { class: "absolute left-3 top-2.5 text-zinc-400", "🔗" }
                         }

                         // Search Input
                         div {
                             class: "relative",
                             input {
                                 class: "pl-10 pr-4 py-2 w-64 rounded-lg border border-zinc-200 dark:border-zinc-700 bg-white dark:bg-zinc-900 focus:outline-none focus:ring-2 focus:ring-black dark:focus:ring-white",
                                 placeholder: "Search registry...",
                                 value: "{query}",
                                 oninput: move |evt| {
                                     query.set(evt.value());
                                     search(());
                                 }
                             }
                             div { class: "absolute left-3 top-2.5 text-zinc-400", "🔍" }
                         }
                    }
                }

                // Content
                div {
                    class: "flex-1 overflow-y-auto p-6 bg-zinc-50 dark:bg-zinc-950/50",
                    if *loading.read() {
                        div { class: "flex justify-center items-center h-full", "Loading..." }
                    } else {
                        div {
                            class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4",
                            for item in items {
                                div {
                                    class: "bg-white dark:bg-zinc-900 p-4 rounded-xl border border-zinc-200 dark:border-zinc-800 hover:border-zinc-400 dark:hover:border-zinc-600 transition-colors flex flex-col justify-between h-full",
                                    div {
                                        div { class: "flex justify-between items-start mb-2",
                                            h3 { class: "font-bold text-lg", "{item.server.name}" }
                                            if let Some(v) = &item.server.version {
                                                span { class: "text-xs bg-zinc-100 dark:bg-zinc-800 px-2 py-1 rounded", "{v}" }
                                            }
                                        }
                                        p { class: "text-sm text-zinc-500 mb-4 line-clamp-3",
                                            "{item.server.description.clone().unwrap_or_default()}"
                                        }
                                    }
                                    
                                    // Item Actions
                                    div {
                                        class: "mt-4 flex justify-between items-center",
                                        div {
                                            if let Some(cat) = &item.server.category {
                                                span {
                                                    class: "px-2 py-1 bg-zinc-100 dark:bg-zinc-800 rounded text-xs text-zinc-500 font-medium border border-zinc-200 dark:border-zinc-700",
                                                    "{cat}"
                                                }
                                            }
                                        }

                                        {
                                            let installed = APP_STATE.read().servers.read().iter().any(|s| s.name == item.server.name);
                                            if installed {
                                                rsx! {
                                                    button {
                                                        class: "px-4 py-2 bg-zinc-100 dark:bg-zinc-800 text-zinc-400 rounded-lg font-bold cursor-not-allowed border border-zinc-200 dark:border-zinc-700",
                                                        disabled: true,
                                                        "Installed"
                                                    }
                                                }
                                            } else {
                                                rsx! {
                                                    button {
                                                        class: "px-4 py-2 bg-black dark:bg-white text-white dark:text-black rounded-lg font-bold hover:opacity-80",
                                                        onclick: move |_| {
                                                            if let Some(config) = &item.install_config {
                                                                if config.wizard.is_some() {
                                                                    active_wizard_item.set(Some(item.clone()));
                                                                    active_wizard_step.set(0);
                                                                    wizard_env_data.write().clear();
                                                                } else {
                                                                    let args = prepare_install_args(&item, None);
                                                                    (props.on_install)(args);
                                                                }
                                                            }
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

                // Footer (Close)
                div {
                    class: "p-4 border-t border-zinc-200 dark:border-zinc-800 flex justify-end bg-white dark:bg-zinc-900",
                    button {
                        class: "px-6 py-2 bg-zinc-200 dark:bg-zinc-800 rounded-lg font-bold hover:bg-zinc-300 dark:hover:bg-zinc-700",
                        onclick: move |_| (props.on_close)(()),
                        "Close"
                    }
                }

                // Modal Overlay for Wizard
                {wizard_overlay}
            }
        }
    }
}

#[derive(PartialEq, Clone, Props)]
pub struct ExplorerProps {
    on_install: EventHandler<CreateServerArgs>,
    on_close: EventHandler<()>,
}
