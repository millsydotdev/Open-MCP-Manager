use dioxus::prelude::*;

#[component]
pub fn Sidebar() -> Element {
    rsx! {
        aside {
            class: "w-64 bg-white dark:bg-zinc-950 border-r border-zinc-200 dark:border-zinc-800 flex flex-col",

            // Logo area
            div {
                class: "h-16 flex items-center px-6 border-b border-zinc-100 dark:border-zinc-900",
                span { class: "font-black text-lg text-indigo-600 tracking-tight", "OPEN MCP" }
            }

            // Nav
            nav {
                class: "flex-1 p-4 space-y-1",
                SidebarLink { label: "Servers", active: true }
                SidebarLink { label: "Settings", active: false }
                SidebarLink { label: "Logs", active: false }
            }

            // Footer
            div {
                class: "p-4 border-t border-zinc-100 dark:border-zinc-900 text-xs text-zinc-400",
                "v0.1.0 Alpha"
            }
        }
    }
}

#[component]
fn SidebarLink(label: String, active: bool) -> Element {
    let base_classes = "flex items-center gap-3 px-3 py-2 rounded-lg text-sm font-medium transition-colors cursor-pointer";
    let active_classes = if active {
        "bg-indigo-50 text-indigo-600 dark:bg-indigo-900/20 dark:text-indigo-400"
    } else {
        "text-zinc-600 dark:text-zinc-400 hover:bg-zinc-50 dark:hover:bg-zinc-900"
    };

    rsx! {
        div {
            class: "{base_classes} {active_classes}",
            "{label}"
        }
    }
}
