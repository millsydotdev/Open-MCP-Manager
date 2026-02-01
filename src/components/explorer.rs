use crate::db::Database;
use crate::models::{
    prepare_install_args, CreateServerArgs, GitHubSearchResponse, RegistryInstallConfig,
    RegistryItem, RegistryServer, WizardAction,
};
use crate::state::APP_STATE;
use dioxus::prelude::*;

const GITHUB_SEARCH_API: &str = "https://api.github.com/search/repositories?q=topic:mcp-server&sort=stars&order=desc&per_page=100";
#[cfg(test)]
const GITHUB_API_URL: &str =
    "https://api.github.com/repos/modelcontextprotocol/servers/contents/src";
const NPM_SEARCH_URL: &str = "https://registry.npmjs.org/-/v1/search";
const PYPI_SEARCH_URL: &str = "https://pypi.org/pypi";

#[cfg(test)]
#[derive(serde::Deserialize, Debug)]
struct GitHubContent {
    name: String,
    #[serde(rename = "type")]
    content_type: String, // "file" or "dir"
}

// NPM API response structures
#[derive(serde::Deserialize, Debug)]
struct NpmSearchResponse {
    objects: Vec<NpmSearchObject>,
}

#[derive(serde::Deserialize, Debug)]
struct NpmSearchObject {
    package: NpmPackage,
}

#[derive(serde::Deserialize, Debug)]
struct NpmPackage {
    name: String,
    version: String,
    description: Option<String>,
    keywords: Option<Vec<String>>,
    links: Option<NpmLinks>,
}

#[derive(serde::Deserialize, Debug)]
struct NpmLinks {
    npm: Option<String>,
    homepage: Option<String>,
    bugs: Option<String>,
}

// PyPI API response structures
#[derive(serde::Deserialize, Debug)]
struct PypiSearchResponse {
    info: PypiInfo,
}

#[derive(serde::Deserialize, Debug)]
struct PypiInfo {
    name: String,
    version: String,
    summary: Option<String>,
    home_page: Option<String>,
    #[allow(dead_code)]
    keywords: Option<String>,
    project_urls: Option<std::collections::HashMap<String, String>>,
}

/// Search NPM for MCP server packages
async fn search_npm_registry(query: &str) -> Vec<RegistryItem> {
    let client = reqwest::Client::new();
    let mut items = Vec::new();

    // Search for MCP-related packages
    let search_terms = [
        format!("{} mcp", query),
        "mcp-server".to_string(),
        "model-context-protocol".to_string(),
    ];

    for term in search_terms {
        let url = format!(
            "{}?text={}&size=20",
            NPM_SEARCH_URL,
            urlencoding::encode(&term)
        );

        if let Ok(resp) = client
            .get(&url)
            .header("User-Agent", "Open-MCP-Manager")
            .send()
            .await
        {
            if let Ok(search_result) = resp.json::<NpmSearchResponse>().await {
                for obj in search_result.objects {
                    let pkg = obj.package;

                    // Filter for MCP-related packages
                    let is_mcp = pkg.name.contains("mcp")
                        || pkg
                            .description
                            .as_ref()
                            .map(|d| {
                                d.to_lowercase().contains("mcp")
                                    || d.to_lowercase().contains("model context protocol")
                            })
                            .unwrap_or(false)
                        || pkg
                            .keywords
                            .as_ref()
                            .map(|k| k.iter().any(|kw| kw.to_lowercase().contains("mcp")))
                            .unwrap_or(false);

                    if is_mcp {
                        // Avoid duplicates
                        if !items
                            .iter()
                            .any(|i: &RegistryItem| i.server.name == pkg.name)
                        {
                            items.push(RegistryItem {
                                server: RegistryServer {
                                    name: pkg.name.clone(),
                                    description: pkg.description.clone(),
                                    homepage: pkg
                                        .links
                                        .as_ref()
                                        .and_then(|l| l.homepage.clone().or(l.npm.clone())),
                                    bugs: pkg.links.as_ref().and_then(|l| l.bugs.clone()),
                                    version: Some(pkg.version),
                                    category: Some("NPM".to_string()),
                                },
                                install_config: Some(RegistryInstallConfig {
                                    command: "npx".to_string(),
                                    args: vec!["-y".to_string(), pkg.name],
                                    env_template: None,
                                    wizard: None,
                                }),
                                source: "npm".to_string(),
                                stars: 0,
                                topics: pkg.keywords.unwrap_or_default(),
                            });
                        }
                    }
                }
            }
        }
    }

    items
}

/// Search PyPI for MCP server packages (by specific known package names)
async fn search_pypi_registry(query: &str) -> Vec<RegistryItem> {
    let client = reqwest::Client::new();
    let mut items = Vec::new();

    // PyPI doesn't have a search API, so we check known MCP package patterns
    let known_patterns = [
        format!("mcp-server-{}", query),
        format!("mcp-{}", query),
        "mcp-server-git".to_string(),
        "mcp-server-fetch".to_string(),
        "mcp-server-filesystem".to_string(),
        "mcp-server-sqlite".to_string(),
        "mcp-server-time".to_string(),
    ];

    for pkg_name in known_patterns {
        let url = format!("{}/{}/json", PYPI_SEARCH_URL, pkg_name);

        if let Ok(resp) = client
            .get(&url)
            .header("User-Agent", "Open-MCP-Manager")
            .send()
            .await
        {
            if resp.status().is_success() {
                if let Ok(pkg_info) = resp.json::<PypiSearchResponse>().await {
                    // Avoid duplicates
                    if !items
                        .iter()
                        .any(|i: &RegistryItem| i.server.name == pkg_info.info.name)
                    {
                        let homepage = pkg_info.info.home_page.clone().or_else(|| {
                            pkg_info
                                .info
                                .project_urls
                                .as_ref()
                                .and_then(|u| u.get("Homepage").cloned())
                        });

                        items.push(RegistryItem {
                            server: RegistryServer {
                                name: pkg_info.info.name.clone(),
                                description: pkg_info.info.summary.clone(),
                                homepage,
                                bugs: pkg_info
                                    .info
                                    .project_urls
                                    .as_ref()
                                    .and_then(|u| u.get("Bug Tracker").cloned()),
                                version: Some(pkg_info.info.version),
                                category: Some("PyPI".to_string()),
                            },
                            install_config: Some(RegistryInstallConfig {
                                command: "uvx".to_string(),
                                args: vec![pkg_info.info.name],
                                env_template: None,
                                wizard: None,
                            }),
                            source: "pypi".to_string(),
                            stars: 0,
                            topics: vec![],
                        });
                    }
                }
            }
        }
    }

    items
}

/// Fetch from all registries (GitHub, NPM, PyPI)
#[allow(dead_code)]
pub async fn fetch_all_registries(query: &str) -> Vec<RegistryItem> {
    let mut all_items = fetch_dynamic_registry().await;

    // Add NPM results
    let npm_items = search_npm_registry(query).await;
    for item in npm_items {
        if !all_items.iter().any(|i| i.server.name == item.server.name) {
            all_items.push(item);
        }
    }

    // Add PyPI results
    let pypi_items = search_pypi_registry(query).await;
    for item in pypi_items {
        if !all_items.iter().any(|i| i.server.name == item.server.name) {
            all_items.push(item);
        }
    }

    // Cache all results
    if let Ok(db) = Database::new() {
        let _ = db.cache_registry(&all_items, "all");
    }

    all_items
}

/// Fetch from GitHub Search API (Community Registry)
async fn fetch_community_registry() -> Vec<RegistryItem> {
    let client = reqwest::Client::new();
    let mut items = Vec::new();

    if let Ok(resp) = client
        .get(GITHUB_SEARCH_API)
        .header("User-Agent", "Open-MCP-Manager")
        .send()
        .await
    {
        if let Ok(search_res) = resp.json::<GitHubSearchResponse>().await {
            for repo in search_res.items {
                // Heuristic for installation command
                let install_config = if let Some(lang) = &repo.language {
                    match lang.as_ref() {
                        "Python" => Some(RegistryInstallConfig {
                            command: "uvx".to_string(),
                            args: vec![repo.name.clone()], // Best guess for PyPI package name
                            env_template: None,
                            wizard: None,
                        }),
                        "TypeScript" | "JavaScript" => Some(RegistryInstallConfig {
                            command: "npx".to_string(),
                            args: vec!["-y".to_string(), repo.name.clone()], // Best guess for NPM package
                            env_template: None,
                            wizard: None,
                        }),
                        _ => None, // Manual install
                    }
                } else {
                    None
                };

                items.push(RegistryItem {
                    server: RegistryServer {
                        name: repo.name.clone(),
                        description: repo.description.clone(),
                        homepage: Some(repo.html_url),
                        bugs: None,
                        version: Some(repo.updated_at.split('T').next().unwrap_or("").to_string()),
                        category: repo.topics.first().cloned(), // Use first topic as category
                    },
                    install_config,
                    source: "community".to_string(),
                    stars: repo.stargazers_count,
                    topics: repo.topics,
                });
            }

            // Cache community results
            if let Ok(db) = Database::new() {
                let _ = db.cache_registry(&items, "community");
            }
        }
    }
    items
}

/// Consolidated fetch function
async fn fetch_dynamic_registry() -> Vec<RegistryItem> {
    let mut items = get_official_registry();

    // 1. Fetch Official (existing logic kept but minimized if needed, for now we rely on community search mostly)
    // Actually, let's just use community search as the primary "Massive" list.
    // But we might want to keep the specific parsing of the official repo if it provides better data.
    // For now, let's append the community search results.

    let community_items = fetch_community_registry().await;

    // Merge logic: prefer official items if names collide?
    for item in community_items {
        if !items
            .iter()
            .any(|existing| existing.server.name == item.server.name)
        {
            items.push(item);
        }
    }

    items
}

/// Fetch registry with explicit cache check (useful for forcing refresh)
#[allow(dead_code)]
pub async fn fetch_registry_with_cache(force_refresh: bool) -> Vec<RegistryItem> {
    let db = Database::new().ok();

    // Check if we should use cache
    if !force_refresh {
        if let Some(ref db) = db {
            // Use cache if less than 24 hours old
            if let Ok(false) = db.is_cache_stale("github", 24) {
                if let Ok(cached) = db.get_cached_registry(None) {
                    if !cached.is_empty() {
                        return cached;
                    }
                }
            }
        }
    }

    // Fetch fresh data
    fetch_dynamic_registry().await
}

pub fn detect_config_from_url(url: &str) -> Option<CreateServerArgs> {
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
    if url_lower.contains("github.com/modelcontextprotocol/servers") && url_lower.contains("/src/")
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
}

pub fn Explorer(props: ExplorerProps) -> Element {
    let mut query = use_signal(|| String::new());
    let mut all_items = use_signal(get_official_registry); // Start with local
    let mut results = use_signal(get_official_registry); // Display local initially
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
    let install_from_url = move |_| {
        let u = url_input.read().clone();
        if let Some(args) = detect_config_from_url(&u) {
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
                    || item
                        .server
                        .description
                        .as_ref()
                        .map(|d: &String| d.to_lowercase().contains(&q))
                        .unwrap_or(false)
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
            class: "fixed inset-0 z-50 bg-black/60 backdrop-blur-sm flex items-center justify-center p-4 animate-fade-in",
            onclick: move |_| (props.on_close)(()),
            div {
                class: "glass-panel w-full max-w-5xl h-[80vh] rounded-2xl shadow-2xl flex flex-col overflow-hidden animate-scale-in border border-zinc-800",
                onclick: move |evt| evt.stop_propagation(),

                // Header
                div {
                    class: "p-6 border-b border-white-5 flex justify-between items-center bg-zinc-900/50",
                    div {
                        h2 { class: "text-2xl font-bold text-white", "Discovery Registry" }
                        p { class: "text-zinc-400", "Find and install MCP servers" }
                    }
                    div {
                         class: "flex gap-2",
                         // URL Install Input
                         div {
                             class: "relative",
                             input {
                                 class: "pl-10 pr-4 py-2 w-64 rounded-xl border border-white-10 bg-black-20 text-white focus:outline-none focus:ring-2 focus:ring-red-500/50 placeholder-zinc-600 transition-all",
                                 placeholder: "Install from URL...",
                                 value: "{url_input}",
                                 oninput: move |evt| url_input.set(evt.value()),
                                 onkeydown: move |evt| {
                                     if evt.key() == Key::Enter && !url_input.read().is_empty() {
                                         install_from_url(());
                                     }
                                 }
                             }
                             div { class: "absolute left-3 top-2.5 text-zinc-500", "🔗" }
                         }

                         // Search Input
                         div {
                             class: "relative",
                             input {
                                 class: "pl-10 pr-4 py-2 w-64 rounded-xl border border-white-10 bg-black-20 text-white focus:outline-none focus:ring-2 focus:ring-red-500/50 placeholder-zinc-600 transition-all",
                                 placeholder: "Search registry...",
                                 value: "{query}",
                                 oninput: move |evt| {
                                     query.set(evt.value());
                                     search(());
                                 }
                             }
                             div { class: "absolute left-3 top-2.5 text-zinc-500", "🔍" }
                         }
                    }
                }

                // Content
                div {
                    class: "flex-1 overflow-y-auto p-6 bg-transparent custom-scrollbar",
                    if *loading.read() {
                        div { class: "flex justify-center items-center h-full text-zinc-400", "Loading..." }
                    } else {
                        div {
                            class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4",
                            for item in items {
                                div {
                                    class: "group relative flex flex-col justify-between h-full bg-zinc-900/50 p-5 rounded-2xl border border-white-5 hover:border-red-500/30 hover:bg-zinc-900 transition-all duration-300",
                                    div {
                                        div { class: "flex justify-between items-start mb-3",
                                            h3 { class: "font-bold text-lg text-white group-hover:text-red-400 transition-colors", "{item.server.name}" }
                                            if let Some(v) = &item.server.version {
                                                span { class: "text-[10px] font-mono bg-white-5 text-zinc-400 px-2 py-1 rounded", "{v}" }
                                            }
                                        }
                                        // Stars badge
                                        div { class: "flex items-center gap-1 mb-2",
                                            span { class: "text-amber-400 text-xs", "★" }
                                            span { class: "text-zinc-400 text-xs", "{item.stars}" }
                                            if !item.topics.is_empty() {
                                                span { class: "mx-1 text-zinc-600 text-xs", "•" }
                                                span { class: "text-zinc-500 text-xs truncate max-w-[150px]", "{item.topics.join(\", \")}" }
                                            }
                                        }
                                        p { class: "text-sm text-zinc-400 mb-4 line-clamp-3 leading-relaxed",
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
                                                        class: "relative z-10 px-4 py-2 bg-black dark:bg-white text-white dark:text-black rounded-lg font-bold hover:opacity-80",
                                                        onclick: move |evt| {
                                                            evt.stop_propagation();
                                                            println!("Install clicked for {}", item.server.name);
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

pub fn get_official_registry() -> Vec<RegistryItem> {
    let registry_json = include_str!("../../registry.json");
    serde_json::from_str(registry_json).unwrap_or_default()
}

#[cfg(test)]
fn capitalize_first(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

#[derive(PartialEq, Clone, Props)]
pub struct ExplorerProps {
    on_install: EventHandler<CreateServerArgs>,
    on_close: EventHandler<()>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_capitalize_first_normal() {
        assert_eq!(capitalize_first("hello"), "Hello");
        assert_eq!(capitalize_first("world"), "World");
        assert_eq!(capitalize_first("gdrive"), "Gdrive");
    }

    #[test]
    fn test_capitalize_first_already_capitalized() {
        assert_eq!(capitalize_first("Hello"), "Hello");
        assert_eq!(capitalize_first("HELLO"), "HELLO");
    }

    #[test]
    fn test_capitalize_first_empty() {
        assert_eq!(capitalize_first(""), "");
    }

    #[test]
    fn test_capitalize_first_single_char() {
        assert_eq!(capitalize_first("a"), "A");
        assert_eq!(capitalize_first("Z"), "Z");
    }

    #[test]
    fn test_capitalize_first_with_numbers() {
        assert_eq!(capitalize_first("123abc"), "123abc");
    }

    #[test]
    fn test_capitalize_first_unicode() {
        assert_eq!(capitalize_first("über"), "Über");
    }

    #[test]
    fn test_official_registry_not_empty() {
        let registry = get_official_registry();
        assert!(!registry.is_empty());
    }

    #[test]
    fn test_official_registry_has_memory() {
        let registry = get_official_registry();
        let memory = registry.iter().find(|r| r.server.name == "Memory");
        assert!(memory.is_some());
    }

    #[test]
    fn test_official_registry_has_brave_search() {
        let registry = get_official_registry();
        let brave = registry.iter().find(|r| r.server.name == "Brave Search");
        assert!(brave.is_some());

        // Check it has env template for API key
        if let Some(config) = &brave.unwrap().install_config {
            assert!(config.env_template.is_some());
            assert!(config
                .env_template
                .as_ref()
                .unwrap()
                .contains_key("BRAVE_API_KEY"));
        }
    }

    #[test]
    fn test_official_registry_has_google_drive_with_wizard() {
        let registry = get_official_registry();
        let gdrive = registry.iter().find(|r| r.server.name == "Google Drive");
        assert!(gdrive.is_some());

        // Check it has wizard steps
        if let Some(config) = &gdrive.unwrap().install_config {
            assert!(config.wizard.is_some());
            assert!(config.wizard.as_ref().unwrap().len() >= 3); // At least 3 steps
        }
    }

    #[test]
    fn test_official_registry_items_have_install_config() {
        let registry = get_official_registry();
        for item in registry {
            assert!(
                item.install_config.is_some(),
                "Item {} should have install_config",
                item.server.name
            );
        }
    }

    #[test]
    fn test_official_registry_items_have_valid_commands() {
        let registry = get_official_registry();
        let valid_commands = ["npx", "python", "node", "uvx"];

        for item in registry {
            if let Some(config) = &item.install_config {
                assert!(
                    valid_commands.contains(&config.command.as_str()),
                    "Item {} has unexpected command: {}",
                    item.server.name,
                    config.command
                );
            }
        }
    }

    #[test]
    fn test_github_content_deserialization() {
        let json = r#"{"name": "gdrive", "type": "dir"}"#;
        let content: GitHubContent = serde_json::from_str(json).unwrap();
        assert_eq!(content.name, "gdrive");
        assert_eq!(content.content_type, "dir");
    }

    #[test]
    fn test_github_api_url_format() {
        assert!(GITHUB_API_URL.contains("api.github.com"));
        assert!(GITHUB_API_URL.contains("modelcontextprotocol"));
    }

    // === NPM Search Tests ===

    #[test]
    fn test_npm_search_url_format() {
        assert!(NPM_SEARCH_URL.contains("registry.npmjs.org"));
        assert!(NPM_SEARCH_URL.contains("search"));
    }

    #[test]
    fn test_npm_search_response_deserialization() {
        let json = r#"{
            "objects": [
                {
                    "package": {
                        "name": "test-mcp-server",
                        "version": "1.0.0",
                        "description": "A test MCP server",
                        "keywords": ["mcp", "server"],
                        "links": {
                            "npm": "https://npmjs.com/package/test-mcp-server",
                            "homepage": "https://example.com"
                        }
                    }
                }
            ]
        }"#;

        let result: NpmSearchResponse = serde_json::from_str(json).unwrap();
        assert_eq!(result.objects.len(), 1);
        assert_eq!(result.objects[0].package.name, "test-mcp-server");
        assert_eq!(result.objects[0].package.version, "1.0.0");
    }

    #[test]
    fn test_npm_package_with_minimal_fields() {
        let json = r#"{
            "objects": [
                {
                    "package": {
                        "name": "minimal-pkg",
                        "version": "0.1.0"
                    }
                }
            ]
        }"#;

        let result: NpmSearchResponse = serde_json::from_str(json).unwrap();
        assert_eq!(result.objects[0].package.name, "minimal-pkg");
        assert!(result.objects[0].package.description.is_none());
        assert!(result.objects[0].package.keywords.is_none());
    }

    // === PyPI Search Tests ===

    #[test]
    fn test_pypi_search_url_format() {
        assert!(PYPI_SEARCH_URL.contains("pypi.org"));
    }

    #[test]
    fn test_pypi_response_deserialization() {
        let json = r#"{
            "info": {
                "name": "mcp-server-test",
                "version": "1.0.0",
                "summary": "A test MCP server for Python",
                "home_page": "https://github.com/example/mcp-server-test",
                "keywords": "mcp server test"
            }
        }"#;

        let result: PypiSearchResponse = serde_json::from_str(json).unwrap();
        assert_eq!(result.info.name, "mcp-server-test");
        assert_eq!(result.info.version, "1.0.0");
        assert_eq!(
            result.info.summary,
            Some("A test MCP server for Python".to_string())
        );
    }

    #[test]
    fn test_pypi_response_with_project_urls() {
        let json = r#"{
            "info": {
                "name": "mcp-server-example",
                "version": "2.0.0",
                "project_urls": {
                    "Homepage": "https://example.com",
                    "Bug Tracker": "https://github.com/example/issues"
                }
            }
        }"#;

        let result: PypiSearchResponse = serde_json::from_str(json).unwrap();
        assert!(result.info.project_urls.is_some());
        let urls = result.info.project_urls.unwrap();
        assert_eq!(
            urls.get("Homepage"),
            Some(&"https://example.com".to_string())
        );
        assert_eq!(
            urls.get("Bug Tracker"),
            Some(&"https://github.com/example/issues".to_string())
        );
    }

    #[test]
    fn test_pypi_response_minimal_fields() {
        let json = r#"{
            "info": {
                "name": "minimal-mcp",
                "version": "0.1.0"
            }
        }"#;

        let result: PypiSearchResponse = serde_json::from_str(json).unwrap();
        assert_eq!(result.info.name, "minimal-mcp");
        assert!(result.info.summary.is_none());
        assert!(result.info.home_page.is_none());
    }

    #[test]
    fn test_detect_npm_package() {
        let url = "https://www.npmjs.com/package/my-server";
        let args = detect_config_from_url(url).expect("Should detect npm package");
        assert_eq!(args.name, "my-server");
        assert_eq!(args.command, Some("npx".to_string()));
        assert_eq!(
            args.args,
            Some(vec!["-y".to_string(), "my-server".to_string()])
        );
    }

    #[test]
    fn test_detect_official_mcp() {
        let url = "https://github.com/modelcontextprotocol/servers/tree/main/src/gdrive";
        let args = detect_config_from_url(url).expect("Should detect official repo");
        assert_eq!(args.name, "gdrive");
        assert_eq!(args.command, Some("npx".to_string()));
        assert_eq!(
            args.args,
            Some(vec![
                "-y".to_string(),
                "@modelcontextprotocol/server-gdrive".to_string()
            ])
        );
    }

    #[test]
    fn test_detect_generic_github() {
        let url = "https://github.com/example/my-repo";
        let args = detect_config_from_url(url).expect("Should detect generic github");
        assert_eq!(args.name, "my-repo");
        assert_eq!(args.command, Some("python".to_string()));
    }

    #[test]
    fn test_detect_unknown() {
        let url = "https://example.com/something";
        assert!(detect_config_from_url(url).is_none());
    }
}
