use dioxus::prelude::*;

#[component]
pub fn Header() -> Element {
    rsx! {
        h1 {
            "Hangry Games"
        }
    }
}

