use dioxus::prelude::*;

#[component]
pub fn Sidebar() -> Element {
    rsx! {
        aside {
            class: "w-72 flex flex-col glass border-r-0 border-r border-white-5 relative z-10",

            // Logo area
            div {
                class: "h-20 flex items-center px-8 border-b border-white-5",
                div {
                    class: "flex items-center gap-3",
                    // Simple logo icon placeholder (or could use an SVG)
                    div {
                        class: "w-8 h-8 rounded-lg bg-gradient-to-tr from-red-600 to-red-500 shadow-lg shadow-red-500/20 flex items-center justify-center text-white font-bold text-lg",
                        "O"
                    }
                    span {
                        class: "font-bold text-xl tracking-tight text-white",
                        "OpenMCP"
                    }
                }
            }

            // Nav
            nav {
                class: "flex-1 p-4 space-y-2 mt-4",
                SidebarLink { label: "Servers", icon: "server", active: true }
                SidebarLink { label: "Settings", icon: "cog", active: false }
                SidebarLink { label: "Logs", icon: "terminal", active: false }
            }

            // Footer
            div {
                class: "p-6 border-t border-white-5",
                div {
                    class: "flex items-center gap-3 p-3 rounded-xl bg-white-5 border border-white-5",
                    div {
                        class: "w-2 h-2 rounded-full bg-green-500 shadow-[0_0_8px_rgba(34,197,94,0.6)] animate-pulse"
                    }
                    div {
                        class: "flex flex-col",
                        span { class: "text-xs font-semibold text-zinc-300", "System Online" }
                        span { class: "text-[10px] text-zinc-500 font-mono", "v0.1.0 Alpha" }
                    }
                }
            }
        }
    }
}

#[component]
fn SidebarLink(label: String, icon: String, active: bool) -> Element {
    // Icons (using simple characters/emojis for now or could potentiall use svg paths if we want consistency)
    // For this rewrite I'll use simple mapping or SVG paths. Let's stick to simple text/emoji or better yet, SVG paths.
    // Actually, let's use Lucide-like SVG icons inline for maximum quality.

    let icon_svg = match icon.as_str() {
        "server" => rsx! {
            svg { class: "w-5 h-5", fill: "none", view_box: "0 0 24 24", stroke: "currentColor", stroke_width: "2",
                path { d: "M4 6a2 2 0 012-2h12a2 2 0 012 2v12a2 2 0 01-2 2H6a2 2 0 01-2-2V6z" }
                path { d: "M10 6v12" }
                path { d: "M4 12h16" }
            }
        },
        "cog" => rsx! {
            svg { class: "w-5 h-5", fill: "none", view_box: "0 0 24 24", stroke: "currentColor", stroke_width: "2",
               path { stroke_linecap: "round", stroke_linejoin: "round", d: "M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z" }
               path { stroke_linecap: "round", stroke_linejoin: "round", d: "M15 12a3 3 0 11-6 0 3 3 0 016 0z" }
            }
        },
        "terminal" => rsx! {
             svg { class: "w-5 h-5", fill: "none", view_box: "0 0 24 24", stroke: "currentColor", stroke_width: "2",
                path { stroke_linecap: "round", stroke_linejoin: "round", d: "M4 17l6-6-6-6m8 14h8" }
             }
        },
        _ => rsx! { div {} },
    };

    let base_classes = "group flex items-center gap-3 px-4 py-3 rounded-xl text-sm font-medium transition-all duration-200 cursor-pointer border border-transparent";
    let active_classes = if active {
        "bg-red-500/10 text-red-500 border-red-500/20 shadow-[0_0_15px_rgba(220,38,38,0.15)]"
    } else {
        "text-zinc-400 hover:text-zinc-100 hover:bg-white-8 hover:border-white-5"
    };

    rsx! {
        div {
            class: "{base_classes} {active_classes}",
            div {
                class: format!("transition-transform duration-200 group-hover:scale-110 {}", if active { "text-red-500" } else { "text-zinc-500 group-hover:text-zinc-300" }),
                {icon_svg}
            }
            span { "{label}" }

            if active {
                div {
                    class: "ml-auto w-1.5 h-1.5 rounded-full bg-red-500 shadow-[0_0_8px_rgba(239,68,68,0.8)]"
                }
            }
        }
    }
}
