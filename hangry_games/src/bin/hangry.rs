use std::collections::VecDeque;
use dioxus::prelude::*;
use hangry_games::animals::Animal;

fn main() {
    launch(app);
}

fn new_animal() -> String {
    Animal::random().to_string()
}

#[component]
fn Header() -> Element {
    rsx! {
        h1 { "Hangry Games" }
    }
}

#[component]
fn Animal() -> Element {
    let mut animal = use_signal(||new_animal());

    rsx! {
        div {
            h2 { {animal} }
            button {
                onclick: move |_| { animal.set(new_animal()) },
                "Random"
            }
        }
    }
}

fn app() -> Element {
    rsx! {
        div {
            Header {}
            Animal {}
        }
    }
}