use dioxus::prelude::*;

#[component]
pub fn GameLog() -> Element {
    rsx! {
        div {
            class: "flow-root",
            h1 { "Game Log" }
        }
    }
}