use dioxus::prelude::*;

#[component]
pub fn Sidebar(active_tab: String, on_tab_change: EventHandler<String>) -> Element {
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
                SidebarLink {
                    label: "Dashboard",
                    icon: "server",
                    active: active_tab == "dashboard",
                    on_click: move |_| on_tab_change.call("dashboard".to_string())
                }
                SidebarLink {
                    label: "Research Hub",
                    icon: "lightbulb",
                    active: active_tab == "research",
                    on_click: move |_| on_tab_change.call("research".to_string())
                }
                SidebarLink {
                    label: "Settings",
                    icon: "cog",
                    active: active_tab == "settings_tab", // Renamed to avoid confusion with show_settings modal
                    on_click: move |_| on_tab_change.call("settings_tab".to_string())
                }
                SidebarLink {
                    label: "Logs",
                    icon: "terminal",
                    active: active_tab == "logs",
                    on_click: move |_| on_tab_change.call("logs".to_string())
                }
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
fn SidebarLink(label: String, icon: String, active: bool, on_click: EventHandler<()>) -> Element {
    let icon_svg = match icon.as_str() {
        "lightbulb" => rsx! {
            svg { class: "w-5 h-5", fill: "none", view_box: "0 0 24 24", stroke: "currentColor", stroke_width: "2",
                path { stroke_linecap: "round", stroke_linejoin: "round", d: "M9.663 17h4.673M12 3v1m6.364 1.636l-.707.707M21 12h-1M4 12H3m3.343-5.657l-.707-.707m2.828 9.9a5 5 0 117.072 0l-.548.547A3.374 3.374 0 0014 18.469V19a2 2 0 11-4 0v-.531c0-.895-.356-1.754-.988-2.386l-.548-.547z" }
            }
        },
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
            class: format!("{} {}", base_classes, active_classes),
            onclick: move |_| on_click.call(()),
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
