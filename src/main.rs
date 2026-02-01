#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing;

// Use the library crate
use open_mcp_manager::app::App;

fn main() {
    // Initialize logging
    dioxus_logger::init(tracing::Level::INFO).expect("failed to init logger");
    tracing::info!("starting app");

    // Launch the Dioxus Desktop app
    // Launch the Dioxus Desktop app
    LaunchBuilder::desktop()
        .with_cfg(dioxus::desktop::Config::new().with_custom_head(format!(
            r#"
                <style>{}</style>
                <style>{}</style>
            "#,
            include_str!("../public/tailwind.css"),
            include_str!("../public/style.css")
        )))
        .launch(App);
}
