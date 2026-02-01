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
            class: "h-20 flex items-center justify-between px-8 bg-transparent z-10",

            // Left side (Breadcrumbs or Page Title - Optional, can be empty for now or show 'Dashboard')
            div {
                class: "flex items-center gap-2",
                h1 {
                    class: "text-2xl font-bold text-white tracking-tight",
                    "Dashboard"
                }
            }

            // Actions
            div {
                class: "flex items-center gap-4",

                // Registry
                button {
                    class: "flex items-center gap-2 px-4 py-2.5 rounded-xl text-sm font-semibold text-zinc-400 hover:text-white hover:bg-white-8 transition-all border border-transparent hover:border-white-5",
                    onclick: move |_| props.on_registry.call(()),
                    svg { class: "w-4 h-4", fill: "none", view_box: "0 0 24 24", stroke: "currentColor", stroke_width: "2",
                        path { stroke_linecap: "round", stroke_linejoin: "round", d: "M21 12a9 9 0 01-9 9m9-9a9 9 0 00-9-9m9 9H3m9 9a9 9 0 01-9-9m9 9c1.657 0 3-4.03 3-9s-1.343-9-3-9m0 18c-1.657 0-3-4.03-3-9s1.343-9 3-9m-9 9a9 9 0 019-9" }
                    }
                    "Registry"
                }

                // Export Config
                button {
                    class: "flex items-center gap-2 px-4 py-2.5 rounded-xl text-sm font-semibold text-zinc-400 hover:text-white hover:bg-white-8 transition-all border border-transparent hover:border-white-5",
                    onclick: move |_| props.on_export.call(()),
                    svg { class: "w-4 h-4", fill: "none", view_box: "0 0 24 24", stroke: "currentColor", stroke_width: "2",
                        path { stroke_linecap: "round", stroke_linejoin: "round", d: "M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-4l-4 4m0 0l-4-4m4 4V4" }
                    }
                    "Export"
                }

                // Add Server (Primary Action)
                button {
                    class: "ml-2 flex items-center gap-2 px-5 py-2.5 bg-gradient-to-r from-red-600 to-red-500 text-white rounded-xl text-sm font-bold shadow-lg shadow-red-500/25 hover:shadow-red-500/40 hover:scale-[1.02] transition-all active:scale-95 border border-red-500/20",
                    onclick: move |_| props.on_add_server.call(()),
                    svg { class: "w-4 h-4", fill: "none", view_box: "0 0 24 24", stroke: "currentColor", stroke_width: "2",
                        path { stroke_linecap: "round", stroke_linejoin: "round", d: "M12 4v16m8-8H4" }
                    }
                    "Add Server"
                }

                div { class: "w-px h-8 bg-white-10 mx-2" }

                ThemeToggle {}
            }
        }
    }
}
