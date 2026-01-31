use crate::components::ThemeToggle;
use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct NavbarProps {
    on_export: EventHandler<()>,
    on_add_server: EventHandler<()>,
    on_registry: EventHandler<()>,
}

pub fn Navbar(props: NavbarProps) -> Element {
    rsx! {
        nav {
            class: "h-16 border-b border-zinc-200 dark:border-zinc-800 flex items-center justify-between px-6 bg-white/50 dark:bg-zinc-900/50 backdrop-blur-sm sticky top-0 z-10",

            // Branding
            div {
                class: "flex items-center gap-2",
                span { class: "text-2xl", "📦" }
                span {
                    class: "text-xl font-bold tracking-tight text-zinc-900 dark:text-white",
                    "Open MCP Manager"
                }
            }

            // Actions
            div {
                class: "flex items-center gap-4",

                // Add Server
                button {
                    class: "flex items-center gap-2 px-4 py-2 bg-indigo-600 text-white rounded-lg text-sm font-bold hover:bg-indigo-500 shadow-lg shadow-indigo-500/20 transition-all",
                    onclick: move |_| props.on_add_server.call(()),
                    span { "+" }
                    "Add Server"
                }

                // Registry
                button {
                    class: "flex items-center gap-2 px-4 py-2 bg-white dark:bg-zinc-800 border border-zinc-200 dark:border-zinc-700 rounded-lg text-sm font-bold hover:bg-zinc-50 dark:hover:bg-zinc-700 transition-colors",
                    onclick: move |_| props.on_registry.call(()),
                    span { "🌍" }
                    "Registry"
                }

                // Export Config
                button {
                    class: "flex items-center gap-2 px-4 py-2 bg-white dark:bg-zinc-800 border border-zinc-200 dark:border-zinc-700 rounded-lg text-sm font-bold hover:bg-zinc-50 dark:hover:bg-zinc-700 transition-colors",
                    onclick: move |_| props.on_export.call(()),
                    span { "⚙️" }
                    "Export Config"
                }

                ThemeToggle {}
            }
        }
    }
}
