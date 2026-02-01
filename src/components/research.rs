use dioxus::prelude::*;

#[component]
pub fn Research() -> Element {
    let research_notes = crate::state::APP_STATE.read().research_notes;
    let mut show_new_note = use_signal(|| false);
    let mut research_input = use_signal(|| String::new());
    let mut is_researching = use_signal(|| false);
    let mut research_results = use_signal(|| Vec::<(String, String, String)>::new());

    rsx! {
        div { class: "flex-1 flex flex-col min-w-0 bg-transparent animate-fade-in",
            // Header
            div { class: "mb-8 flex flex-col md:flex-row md:items-end justify-between gap-4",
                div {
                    h1 { class: "text-4xl font-black text-white mb-2 tracking-tight", "Research Hub" }
                    p { class: "text-zinc-400 text-lg", "Perform deep research on URLs and discover new MCP capabilities." }
                }
                div { class: "flex items-center gap-2 px-4 py-2 bg-red-500/10 border border-red-500/20 rounded-2xl",
                    div { class: "w-2 h-2 rounded-full bg-red-500 animate-pulse" }
                    span { class: "text-red-400 text-xs font-bold uppercase tracking-wider", "AI Enhanced" }
                }
            }

            // Web Research Tool
            div { class: "p-8 rounded-[2.5rem] bg-zinc-900/50 border border-white-5 mb-12 relative overflow-hidden group",
                // Decorative background
                div { class: "absolute -top-24 -right-24 w-64 h-64 bg-red-500/10 blur-[100px] group-hover:bg-red-500/20 transition-all duration-700" }

                div { class: "relative z-10",
                    h3 { class: "text-2xl font-bold text-white mb-2", "Deep URL Researcher" }
                    p { class: "text-zinc-400 mb-6 max-w-2xl",
                        "Paste a GitHub repository or documentation URL to automatically extract installation steps, environment variables, and tool capabilities."
                    }

                    div { class: "flex flex-col md:flex-row gap-3",
                        input {
                            class: "flex-1 px-6 py-4 bg-black/40 border border-white-10 rounded-2xl text-white placeholder:text-zinc-600 focus:outline-none focus:border-red-500/50 transition-all shadow-inner",
                            placeholder: "https://github.com/...",
                            value: "{research_input}",
                            oninput: move |e| research_input.set(e.value())
                        }
                        button {
                            class: "px-8 py-4 bg-white text-black rounded-2xl font-bold hover:bg-zinc-200 transition-all flex items-center justify-center gap-2 active:scale-95 disabled:opacity-50",
                            disabled: is_researching() || research_input().is_empty(),
                            onclick: move |_| {
                                is_researching.set(true);
                                let _url = research_input();
                                spawn(async move {
                                    // Simulated high-fidelity research
                                    tokio::time::sleep(std::time::Duration::from_millis(2500)).await;
                                    research_results.set(vec![
                                        ("Brave Search".to_string(), "Web search engine integration for LLMs.".to_string(), "brave-search".to_string()),
                                        ("Docker Manager".to_string(), "Full container orchestration via MCP.".to_string(), "docker".to_string()),
                                        ("Slack Messenger".to_string(), "Read/write capability for Slack workspaces.".to_string(), "slack".to_string()),
                                    ]);
                                    is_researching.set(false);
                                });
                            },
                            if is_researching() {
                                div { class: "w-5 h-5 border-2 border-black/20 border-t-black rounded-full animate-spin" }
                            } else {
                                svg { class: "w-5 h-5", fill: "none", view_box: "0 0 24 24", stroke: "currentColor", stroke_width: "2",
                                    path { stroke_linecap: "round", stroke_linejoin: "round", d: "M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" }
                                }
                            }
                            span { if is_researching() { "Analyzing..." } else { "Research URL" } }
                        }
                    }
                }
            }

            if !research_results().is_empty() {
                div { class: "mb-12 animate-slide-up",
                    h3 { class: "text-xl font-bold text-white mb-6 flex items-center gap-2",
                        span { "Top Discovery Matches" }
                        span { class: "px-2 py-0.5 bg-zinc-800 text-zinc-500 text-[10px] rounded-md", "{research_results().len()}" }
                    }
                    div { class: "grid grid-cols-1 md:grid-cols-3 gap-4",
                        for (name, desc, slug) in research_results().iter() {
                            div { class: "p-6 rounded-[2rem] bg-zinc-900/30 border border-white-5 hover:border-red-500/30 transition-all cursor-pointer group",
                                h4 { class: "font-bold text-white mb-1 group-hover:text-red-400 transition-colors", "{name}" }
                                p { class: "text-xs text-zinc-500 mb-4 line-clamp-2", "{desc}" }
                                div { class: "flex items-center justify-between",
                                    span { class: "text-[10px] font-mono text-zinc-600", "mcp-server-{slug}" }
                                    button { class: "text-xs text-white px-3 py-1 bg-white/5 rounded-lg hover:bg-white/10 transition-all", "Import" }
                                }
                            }
                        }
                    }
                }
            }

            // Research Tools Grid
            div { class: "grid grid-cols-1 lg:grid-cols-2 gap-6 mb-12",
                // Thinking Tool Card
                div {
                    class: "p-6 rounded-3xl bg-zinc-900/50 border border-white-5 hover:border-red-500/30 transition-all group",
                    div { class: "flex items-start justify-between mb-4",
                        div {
                            class: "w-12 h-12 rounded-2xl bg-indigo-500/10 flex items-center justify-center text-indigo-400 mb-4 group-hover:scale-110 transition-transform",
                            svg { class: "w-6 h-6", fill: "none", view_box: "0 0 24 24", stroke: "currentColor", stroke_width: "2",
                                path { stroke_linecap: "round", stroke_linejoin: "round", d: "M9.663 17h4.673M12 3v1m6.364 1.636l-.707.707M21 12h-1M4 12H3m3.343-5.657l-.707-.707m2.828 9.9a5 5 0 117.072 0l-.548.547A3.374 3.374 0 0014 18.469V19a2 2 0 11-4 0v-.531c0-.895-.356-1.754-.988-2.386l-.548-.547z" }
                            }
                        }
                        span { class: "px-3 py-1 bg-indigo-500/10 text-indigo-400 text-[10px] font-bold uppercase tracking-wider rounded-full", "Methodology" }
                    }
                    h3 { class: "text-xl font-bold text-white mb-2", "Sequential Thinking" }
                    p { class: "text-sm text-zinc-400 leading-relaxed mb-6",
                        "A powerful meta-tool for breaking down complex problems. Use this to design complex server integrations or troubleshoot communication issues."
                    }
                    button {
                        class: "w-full py-3 bg-indigo-600 hover:bg-indigo-500 text-white rounded-xl font-bold transition-all shadow-lg shadow-indigo-500/20 active:scale-[0.98]",
                        onclick: move |_| {
                            // Logic to help them install/open sequential thinking
                            println!("Sequential Thinking clicked");
                        },
                        "Explore Methodology"
                    }
                }

                // Inventory Card
                div {
                    class: "p-6 rounded-3xl bg-zinc-900/50 border border-white-5 hover:border-red-500/30 transition-all group",
                    div { class: "flex items-start justify-between mb-4",
                        div {
                            class: "w-12 h-12 rounded-2xl bg-emerald-500/10 flex items-center justify-center text-emerald-400 mb-4 group-hover:scale-110 transition-transform",
                            svg { class: "w-6 h-6", fill: "none", view_box: "0 0 24 24", stroke: "currentColor", stroke_width: "2",
                                path { stroke_linecap: "round", stroke_linejoin: "round", d: "M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2m-3 7h3m-3 4h3m-6-4h.01M9 16h.01" }
                            }
                        }
                        span { class: "px-3 py-1 bg-emerald-500/10 text-emerald-400 text-[10px] font-bold uppercase tracking-wider rounded-full", "Inventory" }
                    }
                    h3 { class: "text-xl font-bold text-white mb-2", "Server Discovery" }
                    p { class: "text-sm text-zinc-400 leading-relaxed mb-6",
                        "Browse thousands of MCP servers across GitHub, NPM, and PyPI. All findings are cached in your local database for offline research."
                    }
                    button {
                        class: "w-full py-3 bg-emerald-600 hover:bg-emerald-500 text-white rounded-xl font-bold transition-all shadow-lg shadow-emerald-500/20 active:scale-[0.98]",
                        "Open Discovery Registry"
                    }
                }
            }

            // Research Notes Section
            div { class: "flex-1 flex flex-col min-h-0",
                div { class: "flex justify-between items-center mb-6",
                    h2 { class: "text-2xl font-bold text-white", "Research Notes" }
                    button {
                        class: "px-4 py-2 bg-white/5 border border-white-10 rounded-xl hover:bg-white/10 transition-all text-sm font-bold flex items-center gap-2",
                        onclick: move |_| show_new_note.set(true),
                        span { "Add Note" }
                        span { class: "text-zinc-500", "+" }
                    }
                }

                if research_notes.read().is_empty() {
                    div { class: "flex-1 flex flex-col items-center justify-center p-12 rounded-[2.5rem] border-2 border-dashed border-white-5",
                        div { class: "w-16 h-16 rounded-full bg-white-5 flex items-center justify-center text-zinc-600 mb-4", "📝" }
                        h3 { class: "text-xl font-bold text-zinc-400 mb-2", "No research notes yet" }
                        p { class: "text-zinc-500 text-center max-w-sm", "Document your architectural decisions, tool capabilities, and integration plans here." }
                    }
                } else {
                    div { class: "grid grid-cols-1 md:grid-cols-2 gap-4",
                        for note in research_notes.read().iter() {
                            div { class: "p-6 rounded-3xl bg-zinc-900/30 border border-white-5 hover:border-white-10 transition-all",
                                h4 { class: "font-bold text-lg mb-2", "{note.title}" }
                                p { class: "text-sm text-zinc-400 line-clamp-3 mb-4", "{note.content.clone().unwrap_or_default()}" }
                                div { class: "flex gap-2",
                                    for tag in note.tags.iter() {
                                        span { class: "px-2 py-0.5 bg-zinc-800 rounded text-[10px] text-zinc-500 font-mono", "#{tag}" }
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
