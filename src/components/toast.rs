use crate::models::{Notification, NotificationLevel};
use crate::state::{AppState, APP_STATE};
use dioxus::prelude::*;
use std::time::Duration;

#[component]
pub fn ToastContainer() -> Element {
    let notifications = APP_STATE.read().notifications.cloned();

    rsx! {
        div {
            class: "fixed bottom-5 right-5 z-50 flex flex-col gap-2 pointer-events-none",
            for note in notifications {
                Toast { key: "{note.id}", notification: note }
            }
        }
    }
}

#[component]
fn Toast(notification: Notification) -> Element {
    let mut is_visible = use_signal(|| false);
    let note_id = notification.id;

    use_future(move || async move {
        // Animate in
        is_visible.set(true);
        // Wait duration
        tokio::time::sleep(Duration::from_secs(notification.duration as u64)).await;
        // Animate out (optional, simplified here)
        APP_STATE.write(); // Keep write lock briefly if needed, but the method handles it
        AppState::remove_notification(note_id);
    });

    let bg_color = match notification.level {
        NotificationLevel::Info => "bg-zinc-800 border-zinc-700 text-zinc-200",
        NotificationLevel::Success => "bg-emerald-900/90 border-emerald-700 text-emerald-100",
        NotificationLevel::Warning => "bg-amber-900/90 border-amber-700 text-amber-100",
        NotificationLevel::Error => "bg-red-900/90 border-red-700 text-red-100",
    };

    let icon = match notification.level {
        NotificationLevel::Info => "ℹ️",
        NotificationLevel::Success => "✅",
        NotificationLevel::Warning => "⚠️",
        NotificationLevel::Error => "❌",
    };

    rsx! {
        div {
            class: "pointer-events-auto flex items-center gap-3 px-4 py-3 rounded-lg shadow-lg border backdrop-blur-md transition-all duration-300 transform translate-y-0 opacity-100 {bg_color} min-w-[300px]",
            // Initial animation state could be handled with checks on mounted, but for now simple render
            span { class: "text-lg", "{icon}" }
            div { class: "flex-1 text-sm font-medium", "{notification.message}" }
            button {
                class: "text-white/50 hover:text-white p-1 rounded-full",
                onclick: move |_| AppState::remove_notification(note_id),
                "✕"
            }
        }
    }
}
