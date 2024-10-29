use dioxus::prelude::*;

#[component]
pub fn Header() -> Element {
    rsx! {
        h1 {
            class: "orbitron-font text-3xl font-bold sm:text-4xl text-center text-black mb-4",
            "The Hangry Games"
        }
    }
}

