use dioxus::prelude::*;

#[component]
pub fn GamePlay() -> Element {
    rsx! {
        div {
            class: "flow-root",
            h1 { "Game Play" }
        }
    }
}