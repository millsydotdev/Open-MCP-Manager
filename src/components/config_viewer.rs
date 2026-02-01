use crate::models::McpServer;
use dioxus::prelude::*;
use serde_json::json;

#[derive(PartialEq, Clone, Props)]
pub struct ConfigViewerProps {
    servers: Vec<McpServer>,
    on_close: EventHandler<()>,
}

#[derive(PartialEq, Clone, Copy)]
enum ConfigMode {
    Hub,
    Direct,
}

#[derive(PartialEq, Clone, Copy)]
enum TargetEditor {
    Claude,
    Cursor,
    Windsurf,
    OpenCode,
    Antigravity,
}

impl TargetEditor {
    fn name(&self) -> &'static str {
        match self {
            TargetEditor::Claude => "Claude",
            TargetEditor::Cursor => "Cursor",
            TargetEditor::Windsurf => "Windsurf",
            TargetEditor::OpenCode => "OpenCode",
            TargetEditor::Antigravity => "Antigravity",
        }
    }

    fn macos_path(&self) -> &'static str {
        match self {
            TargetEditor::Claude => {
                "~/Library/Application Support/Claude/claude_desktop_config.json"
            }
            TargetEditor::Cursor => "~/.cursor/mcp.json",
            TargetEditor::Windsurf => "~/.codeium/windsurf/mcp_config.json",
            TargetEditor::OpenCode => "opencode.jsonc (Project Root)",
            TargetEditor::Antigravity => "~/.gemini/antigravity/mcp_config.json",
        }
    }

    fn windows_path(&self) -> &'static str {
        match self {
            TargetEditor::Claude => "%APPDATA%\\Claude\\claude_desktop_config.json",
            TargetEditor::Cursor => {
                "%APPDATA%\\Cursor\\mcp.json or %USERPROFILE%\\.cursor\\mcp.json"
            }
            TargetEditor::Windsurf => "%USERPROFILE%\\.codeium\\windsurf\\mcp_config.json",
            TargetEditor::OpenCode => "opencode.jsonc (Project Root)",
            TargetEditor::Antigravity => "%USERPROFILE%\\.gemini\\antigravity\\mcp_config.json",
        }
    }

    fn download_filename(&self) -> &'static str {
        match self {
            TargetEditor::Claude => "claude_desktop_config.json",
            TargetEditor::Cursor => "mcp.json",
            TargetEditor::Windsurf => "mcp_config.json",
            TargetEditor::OpenCode => "opencode.jsonc",
            TargetEditor::Antigravity => "mcp_config.json",
        }
    }

    fn icon(&self) -> Element {
        match self {
            TargetEditor::Claude => rsx! {
                svg {
                    view_box: "0 0 24 24",
                    class: "w-4 h-4",
                    fill: "currentColor",
                    path {
                        d: "M4.709 15.955l4.72-2.647.08-.23-.08-.128H9.2l-.79-.048-2.698-.073-2.339-.097-2.266-.122-.571-.121L0 11.784l.055-.352.48-.321.686.06 1.52.103 2.278.158 1.652.097 2.449.255h.389l.055-.157-.134-.098-.103-.097-2.358-1.596-2.552-1.688-1.336-.972-.724-.491-.364-.462-.158-1.008.656-.722.881.06.225.061.893.686 1.908 1.476 2.491 1.833.365.304.145-.103.019-.073-.164-.274-1.355-2.446-1.446-2.49-.644-1.032-.17-.619a2.97 2.97 0 01-.104-.729L6.283.134 6.696 0l.996.134.42.364.62 1.414 1.002 2.229 1.555 3.03.456.898.243.832.091.255h.158V9.01l.128-1.706.237-2.095.23-2.695.08-.76.376-.91.747-.492.584.28.48.685-.067.444-.286 1.851-.559 2.903-.364 1.942h.212l.243-.242.985-1.306 1.652-2.064.73-.82.85-.904.547-.431h1.033l.76 1.129-.34 1.166-1.064 1.347-.881 1.142-1.264 1.7-.79 1.36.073.11.188-.02 2.856-.606 1.543-.28 1.841-.315.833.388.091.395-.328.807-1.969.486-2.309.462-3.439.813-.042.03.049.061 1.549.146.662.036h1.622l3.02.225.79.522.474.638-.079.485-1.215.62-1.64-.389-3.829-.91-1.312-.329h-.182v.11l1.093 1.068 2.006 1.81 2.509 2.33.127.578-.322.455-.34-.049-2.205-1.657-.851-.747-1.926-1.62h-.128v.17l.444.649 2.345 3.521.122 1.08-.17.353-.608.213-.668-.122-1.374-1.925-1.415-2.167-1.143-1.943-.14.08-.674 7.254-.316.37-.729.28-.607-.461-.322-.747.322-1.476.389-1.924.315-1.53.286-1.9.17-.632-.012-.042-.14.018-1.434 1.967-2.18 2.945-1.726 1.845-.414.164-.717-.37.067-.662.401-.589 2.388-3.036 1.44-1.882.93-1.086-.006-.158h-.055L4.132 18.56l-1.13.146-.487-.456.061-.746.231-.243 1.908-1.312-.006.006z",
                    }
                }
            },
            TargetEditor::Cursor => rsx! {
                svg { view_box: "0 0 467 534", class: "w-4 h-4",
                    // Cursor logo has specific 3D shading, so we use the official colors (or close approximations if we want to theme it, but user asked for official)
                    // We will map the hex codes from the source SVG.
                    path {
                        fill: "#72716d",
                        d: "M233.37,266.66l231.16,133.46c-1.42,2.46-3.48,4.56-6.03,6.03l-216.06,124.74c-5.61,3.24-12.53,3.24-18.14,0L8.24,406.15c-2.55-1.47-4.61-3.57-6.03-6.03l231.16-133.46h0Z",
                    }
                    path {
                        fill: "#55544f",
                        d: "M233.37,0v266.66L2.21,400.12c-1.42-2.46-2.21-5.3-2.21-8.24v-250.44c0-5.89,3.14-11.32,8.24-14.27L224.29,2.43c2.81-1.62,5.94-2.43,9.07-2.43h.01Z",
                    }
                    path {
                        fill: "#43413c",
                        d: "M464.52,133.2c-1.42-2.46-3.48-4.56-6.03-6.03L242.43,2.43c-2.8-1.62-5.93-2.43-9.06-2.43v266.66l231.16,133.46c1.42-2.46,2.21-5.3,2.21-8.24v-250.44c0-2.95-.78-5.77-2.21-8.24h-.01Z",
                    }
                    path {
                        fill: "#d6d5d2",
                        d: "M448.35,142.54c1.31,2.26,1.49,5.16,0,7.74l-209.83,363.42c-1.41,2.46-5.16,1.45-5.16-1.38v-239.48c0-1.91-.51-3.75-1.44-5.36l216.42-124.95h.01Z",
                    }
                    path {
                        fill: "#ffffff",
                        d: "M448.35,142.54l-216.42,124.95c-.92-1.6-2.26-2.96-3.92-3.92L20.62,143.83c-2.46-1.41-1.45-5.16,1.38-5.16h419.65c2.98,0,5.4,1.61,6.7,3.87Z",
                    }
                }
            },
            TargetEditor::Windsurf => rsx! {
                svg {
                    view_box: "0 0 1024 1024",
                    class: "w-4 h-4",
                    fill: "currentColor",
                    path {
                        d: "M897.246 286.869H889.819C850.735 286.808 819.017 318.46 819.017 357.539V515.589C819.017 547.15 792.93 572.716 761.882 572.716C743.436 572.716 725.02 563.433 714.093 547.85L552.673 317.304C539.28 298.16 517.486 286.747 493.895 286.747C457.094 286.747 423.976 318.034 423.976 356.657V515.619C423.976 547.181 398.103 572.746 366.842 572.746C348.335 572.746 329.949 563.463 319.021 547.881L138.395 289.882C134.316 284.038 125.154 286.93 125.154 294.052V431.892C125.154 438.862 127.285 445.619 131.272 451.34L309.037 705.2C319.539 720.204 335.033 731.344 352.9 735.392C397.616 745.557 438.77 711.135 438.77 667.278V508.406C438.77 476.845 464.339 451.279 495.904 451.279H495.995C515.02 451.279 532.857 460.562 543.785 476.145L705.235 706.661C718.659 725.835 739.327 737.218 763.983 737.218C801.606 737.218 833.841 705.9 833.841 667.308V508.376C833.841 476.815 859.41 451.249 890.975 451.249H897.276C901.233 451.249 904.43 448.053 904.43 444.097V294.021C904.43 290.065 901.233 286.869 897.276 286.869H897.246Z",
                    }
                }
            },
            TargetEditor::OpenCode => rsx! {
                svg {
                    view_box: "0 0 24 24",
                    class: "w-4 h-4",
                    fill: "none",
                    stroke: "currentColor",
                    stroke_width: "2",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    polyline { points: "4 17 10 11 4 5" }
                    line {
                        x1: "12",
                        y1: "19",
                        x2: "20",
                        y2: "19",
                    }
                }
            },
            TargetEditor::Antigravity => rsx! {
                svg {
                    view_box: "0 0 18 18",
                    class: "w-4 h-4",
                    fill: "currentColor",
                    path {
                        fill_rule: "evenodd",
                        clip_rule: "evenodd",
                        d: "M5.56944 5.56944C5.90833 2.43556 8.59667 0 11.7633 0C15.2067 0.00245126 17.9975 2.79327 18 6.23667C18 9.40222 15.5644 12.0917 12.4306 12.4306C12.1022 15.5644 9.40333 18 6.23667 18C2.79327 17.9975 0.00245126 15.2067 0 11.7633C0 8.59778 2.43556 5.90833 5.56944 5.56944ZM12.4839 11.0323C14.5812 10.7134 16.2278 9.06675 16.5484 6.96774L12.4839 6.96774V11.0323ZM1.45161 11.0323L1.45296 11.0234L5.51613 11.0234V6.96774C3.41788 7.28748 1.77207 8.93078 1.45296 11.0234H1.45161V11.0323ZM6.24194 16.5484C8.64155 16.5484 10.6793 14.8194 11.0323 12.4839L1.45161 12.4839C1.80453 14.8194 3.84232 16.5484 6.24194 16.5484ZM11.0323 6.96774V11.0323H6.96774V6.96774H11.0323ZM6.96774 5.51613L16.5484 5.51613C16.1955 3.18064 14.1577 1.45161 11.7581 1.45161C9.35845 1.45161 7.32066 3.18064 6.96774 5.51613Z",
                    }
                }
            },
        }
    }
}

pub fn ConfigViewer(props: ConfigViewerProps) -> Element {
    let mut mode = use_signal(|| ConfigMode::Hub);
    let mut editor = use_signal(|| TargetEditor::Claude);
    let mut copied = use_signal(|| false);

    // TODO: Dynamically get origin if possible, or use a default compatible with how the hub is exposed.
    // For Dioxus desktop, we might need a specific port if we implement the SSE server in Rust.
    // For now, mirroring the legacy behavior which used window.location.origin.
    let origin = "http://localhost:3000"; // Placeholder, standard for many dev setups.

    let config_json = use_memo(move || match mode() {
        ConfigMode::Hub => {
            json!({
                "mcpServers": {
                    "mcp-manager-hub": {
                        "url": format!("{}/api/mcp/sse", origin)
                    }
                }
            })
        }
        ConfigMode::Direct => {
            let mut servers_map = serde_json::Map::new();
            for server in props.servers.iter().filter(|s| s.is_active) {
                let mut server_config = serde_json::Map::new();

                if let Some(cmd) = &server.command {
                    server_config.insert("command".to_string(), json!(cmd));
                }
                if let Some(args) = &server.args {
                    server_config.insert("args".to_string(), json!(args));
                }
                if let Some(env) = &server.env {
                    if !env.is_empty() {
                        server_config.insert("env".to_string(), json!(env));
                    }
                }

                servers_map.insert(
                    server.name.clone(),
                    serde_json::Value::Object(server_config),
                );
            }

            json!({
                "mcpServers": servers_map
            })
        }
    });

    let config_string = serde_json::to_string_pretty(&*config_json.read()).unwrap_or_default();
    let config_string_copy = config_string.clone(); // Clone for copy closure
    let config_string_download = config_string.clone(); // Clone for download closure

    // Capture current editor filename for the download closure
    let current_filename = editor.read().download_filename();

    let copy_to_clipboard = move |_| {
        let val = config_string_copy.clone();
        spawn(async move {
            // Dioxus eval for clipboard access
            // Note: In Dioxus Desktop 0.6+, clipboard might be accessible via specific crate or eval.
            // Using eval for browser/webview compatibility.
            let eval = document::eval(&format!(
                r#"
                 navigator.clipboard.writeText(`{}`);
                 return true;
                 "#,
                val.replace("`", "\\`") // Basic escape
            ));
            let _ = eval.await;
        });
        copied.set(true);
        // Reset "copied" state after 2 seconds
        let mut copied_signal = copied;
        spawn(async move {
            use std::time::Duration;
            use tokio::time::sleep;
            sleep(Duration::from_secs(2)).await;
            copied_signal.set(false);
        });
    };

    let download_config = move |_| {
        let val = config_string_download.clone();
        let filename = current_filename;
        spawn(async move {
            let eval = document::eval(&format!(
                r#"
                 const blob = new Blob([`{}`], {{ type: "application/json" }});
                 const url = URL.createObjectURL(blob);
                 const a = document.createElement("a");
                 a.href = url;
                 a.download = "{}";
                 document.body.appendChild(a);
                 a.click();
                 document.body.removeChild(a);
                 URL.revokeObjectURL(url);
                 return true;
                 "#,
                val.replace("`", "\\`"),
                filename
            ));
            let _ = eval.await;
        });
    };

    let active_class = "flex items-center gap-2 px-6 py-2.5 text-sm font-bold rounded-xl transition-all bg-white text-red-600 shadow-sm";
    let inactive_class = "flex items-center gap-2 px-6 py-2.5 text-sm font-bold rounded-xl transition-all text-zinc-500 hover:text-zinc-300";

    let editor_btn_base = "px-4 py-2 text-xs font-semibold rounded-lg transition-colors border";
    let editor_active = "bg-red-500/20 text-red-500 border-red-500/30";
    let editor_inactive =
        "bg-zinc-900 text-zinc-500 border-transparent hover:text-zinc-300 hover:bg-zinc-800";

    rsx! {
        div { class: "fixed inset-0 z-50 flex items-center justify-center bg-black/60 backdrop-blur-sm p-4 animate-fade-in",
            div { class: "w-full max-w-3xl overflow-hidden rounded-[2.5rem] bg-zinc-950 border border-zinc-800 shadow-2xl animate-scale-in",
                // Header
                div { class: "flex items-center justify-between border-b border-zinc-900 p-8",
                    div {
                        h2 { class: "text-2xl font-bold text-white", "Editor Configuration" }
                        p { class: "text-sm text-zinc-400",
                            "Choose how you want to integrate with your editor."
                        }
                    }
                    button {
                        class: "rounded-full p-2 hover:bg-zinc-900 transition-colors text-zinc-400",
                        onclick: move |_| props.on_close.call(()),
                        "✕"
                    }
                }

                div { class: "p-8 space-y-8",
                    // Mode Switcher
                    div { class: "flex flex-col items-center gap-6",
                        div { class: "flex gap-4 p-1.5 bg-zinc-900 rounded-2xl w-fit",
                            button {
                                class: if *mode.read() == ConfigMode::Hub { active_class } else { inactive_class },
                                onclick: move |_| mode.set(ConfigMode::Hub),
                                "⚡ Hub Mode"
                            }
                            button {
                                class: if *mode.read() == ConfigMode::Direct { active_class } else { inactive_class },
                                onclick: move |_| mode.set(ConfigMode::Direct),
                                "📚 Direct Mode"
                            }
                        }

                        // Editor Selector
                        div { class: "flex flex-wrap justify-center gap-2",
                            {
                                [
                                    TargetEditor::Claude,
                                    TargetEditor::Cursor,
                                    TargetEditor::Windsurf,
                                    TargetEditor::OpenCode,
                                    TargetEditor::Antigravity,
                                ]
                                    .into_iter()
                                    .map(|target| {
                                        let is_active = *editor.read() == target;
                                        let current_class = if is_active {
                                            editor_active
                                        } else {
                                            editor_inactive
                                        };
                                        let full_class = format!("{} {}", editor_btn_base, current_class);
                                        rsx! {
                                            button {
                                                class: "{full_class} flex items-center gap-2",
                                                onclick: move |_| editor.set(target),
                                                {target.icon()}
                                                "{target.name()}"
                                            }
                                        }
                                    })
                            }
                        }
                    }

                    // Info Box
                    div { class: "flex items-start gap-4 p-4 rounded-2xl bg-red-500/5 border border-red-500/10",
                        p { class: "text-sm text-red-400 leading-relaxed",
                            if *mode.read() == ConfigMode::Hub {
                                "Connects your editor to this manager. Changes here are automatically reflected in your editor without manual file updates."
                            } else {
                                "Generates a complete list of all active servers. You'll need to re-copy this file whenever you add or remove servers."
                            }
                        }
                    }

                    // Code / Config Display
                    div { class: "relative group",
                        pre { class: "max-h-[300px] overflow-auto rounded-3xl bg-black p-6 text-xs font-mono text-zinc-300 border border-zinc-800",
                            "{config_string}"
                        }
                        div { class: "absolute right-4 top-4 flex gap-2",
                            button {
                                class: "rounded-xl bg-zinc-800 p-3 text-zinc-400 hover:bg-zinc-700 hover:text-white transition-all active:scale-95",
                                onclick: copy_to_clipboard,
                                title: "Copy to clipboard",
                                if *copied.read() {
                                    "✓"
                                } else {
                                    "📋"
                                }
                            }
                            button {
                                class: "rounded-xl bg-zinc-800 p-3 text-zinc-400 hover:bg-zinc-700 hover:text-white transition-all active:scale-95",
                                onclick: download_config,
                                title: "Download JSON",
                                "⬇️"
                            }
                        }
                    }

                    // Path Helpers
                    div { class: "grid grid-cols-2 gap-4",
                        div { class: "p-5 rounded-3xl bg-zinc-900/50 border border-zinc-900",
                            h4 { class: "text-xs font-bold uppercase tracking-widest text-zinc-500 mb-3",
                                "macOS Location"
                            }
                            code { class: "text-[11px] font-mono text-zinc-300 break-all leading-relaxed",
                                "{editor.read().macos_path()}"
                            }
                        }
                        div { class: "p-5 rounded-3xl bg-zinc-900/50 border border-zinc-900",
                            h4 { class: "text-xs font-bold uppercase tracking-widest text-zinc-500 mb-3",
                                "Windows Location"
                            }
                            code { class: "text-[11px] font-mono text-zinc-300 break-all leading-relaxed",
                                "{editor.read().windows_path()}"
                            }
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dioxus::dioxus_core::VirtualDom;

    #[test]
    fn test_config_viewer_renders() {
        fn test_app() -> Element {
            let servers = vec![McpServer {
                id: "test-id".to_string(),
                name: "test-server".to_string(),
                server_type: "stdio".to_string(),
                command: Some("npx".to_string()),
                args: Some(vec!["-y".to_string(), "server".to_string()]),
                url: None,
                env: None,
                description: None,
                is_active: true,
                created_at: "2024-01-01T00:00:00Z".to_string(),
                updated_at: "2024-01-01T00:00:00Z".to_string(),
            }];

            rsx! {
                ConfigViewer { servers, on_close: move |_| {} }
            }
        }

        let mut vdom = VirtualDom::new(test_app);
        vdom.rebuild_in_place();
        let html = dioxus_ssr::render(&vdom);

        assert!(html.contains("Editor Configuration"));
        assert!(html.contains("Hub Mode"));
        assert!(html.contains("Direct Mode"));
    }
}
