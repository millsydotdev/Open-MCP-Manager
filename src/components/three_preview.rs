use crate::state::APP_STATE;
use dioxus::prelude::*;

#[component]
pub fn ThreePreview() -> Element {
    let servers = APP_STATE.read().servers;
    let running_handlers = APP_STATE.read().running_handlers;

    let server_count = servers.read().len();

    // Simple grid layout
    let cols = 4;
    let node_radius = 40.0;
    let gap = 120.0;

    rsx! {
        div {
            class: "h-64 w-full bg-zinc-950 rounded-xl flex items-center justify-center relative overflow-hidden border border-zinc-800",

            if server_count == 0 {
                div { class: "text-zinc-500", "No servers to visualize." }
            } else {
                svg {
                    width: "100%",
                    height: "100%",
                    view_box: "0 0 800 300",

                    // Central "Hub" node
                    line { x1: "400", y1: "150", x2: "400", y2: "150", stroke: "#6366f1", stroke_width: "2", opacity: "0.5" } // anchor

                    for (i, server) in servers.read().iter().enumerate() {
                        {
                            let row = i / cols;
                            let col = i % cols;
                            let x = 150.0 + (col as f64 * gap);
                            let y = 80.0 + (row as f64 * gap);

                            let is_running = running_handlers.read().contains_key(&server.id);
                            let stroke_color = if is_running { "#22c55e" } else { "#3f3f46" };

                            rsx! {
                                g {
                                    key: "{server.id}",
                                    // Connection to center (just for visual flair)
                                    line {
                                        x1: "400", y1: "150",
                                        x2: "{x}", y2: "{y}",
                                        stroke: "#3f3f46",
                                        stroke_width: "1",
                                        opacity: "0.3"
                                    }

                                    // Node
                                    circle {
                                        cx: "{x}",
                                        cy: "{y}",
                                        r: "{node_radius}",
                                        fill: "#18181b",
                                        stroke: "{stroke_color}",
                                        stroke_width: "3"
                                    }

                                    // Label
                                    text {
                                        x: "{x}",
                                        y: "{y}",
                                        text_anchor: "middle",
                                        dy: "0.3em",
                                        fill: "#cbd5e1",
                                        font_size: "10",
                                        font_family: "monospace",
                                        "{server.name}"
                                    }
                                }
                            }
                        }
                    }

                    // Center Hub
                    circle { cx: "400", cy: "150", r: "20", fill: "#4f46e5", opacity: "0.8" }
                    text { x: "400", y: "150", text_anchor: "middle", dy: "0.3em", fill: "white", font_size: "10", "MCP" }
                }
            }

            div { class: "absolute bottom-2 right-2 text-xs text-zinc-600",
                "Visualization Active"
            }
        }
    }
}
