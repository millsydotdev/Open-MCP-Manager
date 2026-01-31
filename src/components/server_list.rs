use crate::components::ServerCard;
use crate::models::McpServer;
use crate::state::APP_STATE;
use dioxus::prelude::*;

#[derive(PartialEq, Clone, Props)]
pub struct ServerListProps {
    on_open_console: EventHandler<McpServer>,
    on_edit_server: EventHandler<McpServer>,
}

pub fn ServerList(props: ServerListProps) -> Element {
    let servers = APP_STATE.read().servers.clone();

    rsx! {
        div {
            class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-6",
            if servers.read().is_empty() {
                 div {
                     class: "col-span-full flex flex-col items-center justify-center py-20 text-center text-zinc-500",
                     div { class: "text-4xl mb-4 opacity-20", "📭" }
                     p { class: "text-lg font-medium", "No servers found" }
                     p { class: "text-sm", "Click 'Explorer' or 'Add Server' to get started." }
                 }
            } else {
                {
                    let servers_vec = servers.read().clone();
                    rsx! {
                        for (i, server) in servers_vec.iter().enumerate() {
                            div {
                                class: "animate-fade-in-up",
                                style: format!("animation-delay: {}ms", i * 50),
                                ServerCard {
                                    key: "{server.id}",
                                    server: server.clone(),
                                    on_console_click: {
                                        let s = server.clone();
                                        move |_| (props.on_open_console)(s.clone())
                                    },
                                    on_edit_click: {
                                        let s = server.clone();
                                        move |_| (props.on_edit_server)(s.clone())
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
