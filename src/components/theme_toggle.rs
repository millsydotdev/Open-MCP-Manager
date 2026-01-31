use dioxus::document::eval;
use dioxus::prelude::*;

#[component]
pub fn ThemeToggle() -> Element {
    // Simple toggle leveraging Tailwind's 'dark' class on HTML element
    // In Dioxus Desktop, we interact with the webview's document
    let mut is_dark = use_signal(|| false);

    let toggle_theme = move |_| {
        let new_val = !is_dark();
        is_dark.set(new_val);

        // Inject JS to toggle class
        // This is the "Dioxus way" for simple DOM manipulation in desktop
        let js = if new_val {
            "document.documentElement.classList.add('dark');"
        } else {
            "document.documentElement.classList.remove('dark');"
        };

        let _ = eval(js);
    };

    rsx! {
        button {
            class: "p-2 rounded-full hover:bg-zinc-100 dark:hover:bg-zinc-800 transition-colors",
            onclick: toggle_theme,
            if is_dark() {
                "🌙" // Moon icon
            } else {
                "☀️" // Sun icon
            }
        }
    }
}
