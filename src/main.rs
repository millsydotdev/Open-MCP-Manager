#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing;

mod app;
mod components;
mod db;
mod models;
mod process;
mod state;

use app::App;

fn main() {
    // Initialize logging
    dioxus_logger::init(tracing::Level::INFO).expect("failed to init logger");
    tracing::info!("starting app");

    // Launch the Dioxus Desktop app
    LaunchBuilder::desktop().launch(App);
}
