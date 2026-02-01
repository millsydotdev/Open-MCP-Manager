use crate::components::{
    ConfigViewer, Explorer, Navbar, ServerConsole, ServerList, Sidebar, ToastContainer,
};
use crate::models::{CreateServerArgs, McpServer};
use crate::state::{use_app_state, APP_STATE};
use dioxus::prelude::*;

pub fn App() -> Element {
    use_app_state();

    let mut show_explorer = use_signal(|| false);
    let mut show_console = use_signal(|| None::<McpServer>);
    let mut show_settings = use_signal(|| None::<Option<McpServer>>); // None=Closed, Some(None)=Add, Some(Some(s))=Edit
    let mut show_config = use_signal(|| false);

    let open_console = move |server: McpServer| {
        show_console.set(Some(server));
    };

    let edit_server = move |server: McpServer| {
        show_settings.set(Some(Some(server)));
    };

    let install_server = move |args: CreateServerArgs| {
        spawn(async move {
            let _ = crate::state::AppState::add_server(args).await;
        });
        show_explorer.set(false);
    };

    let save_server = move |args: CreateServerArgs| {
        if let Some(Some(srv)) = show_settings() {
            // Update
            let id = srv.id.clone();
            spawn(async move {
                let update_args = crate::models::UpdateServerArgs {
                    name: Some(args.name),
                    server_type: Some(args.server_type),
                    command: args.command, // Already Option
                    args: args.args,       // Already Option
                    env: args.env,         // Already Option
                    url: args.url,
                    description: args.description,
                    is_active: None,
                };
                let _ = crate::state::AppState::update_server(id, update_args).await;
            });
        } else {
            // Create
            spawn(async move {
                let _ = crate::state::AppState::add_server(args).await;
            });
        }
        show_settings.set(None);
    };

    let delete_server_handler = move |id: String| {
        spawn(async move {
            // Stop process if running
            let _ = crate::state::AppState::stop_server_process(&id).await;
            let _ = crate::state::AppState::delete_server(id).await;
        });
        show_settings.set(None);
    };

    rsx! {
        // Tailwind
        // Local Tailwind (Static v2)
        link { rel: "stylesheet", href: "tailwind.css" }

        // Custom Styles (Fonts, Variables, Glassmorphism)
        link { rel: "stylesheet", href: "style.css" }

        div {
            class: "flex h-screen bg-app-dark text-white font-sans overflow-hidden relative selection:bg-red-500/30",

            ToastContainer {}

            Sidebar {}

            main {
                class: "flex-1 flex flex-col relative min-w-0 bg-gradient-to-br from-app-dark to-app-secondary",

                Navbar {
                    on_add_server: move |_| show_settings.set(Some(None)),
                    on_registry: move |_| show_explorer.set(true),
                    on_export: move |_| show_config.set(true),
                }

                div {
                    class: "flex-1 overflow-y-auto p-8 scroll-smooth z-0 custom-scrollbar",
                    ServerList {
                        on_open_console: open_console,
                        on_edit_server: edit_server
                    }
                }
            }

            // Modals layer
            if show_explorer() {
                Explorer {
                    on_install: install_server,
                    on_close: move |_| show_explorer.set(false)
                }
            }

            if let Some(opts) = show_settings() {
                crate::components::Settings {
                    server: opts,
                    on_close: move |_| show_settings.set(None),
                    on_save: save_server,
                    on_delete: delete_server_handler
                }
            }

            if let Some(srv) = show_console() {
                ServerConsole {
                    server: srv,
                    on_close: move |_| show_console.set(None)
                }
            }

            if show_config() {
                ConfigViewer {
                    servers: APP_STATE.read().servers.read().clone(),
                    on_close: move |_| show_config.set(false)
                }
            }
        }
    }
}
