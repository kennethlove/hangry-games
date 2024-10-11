use dioxus::prelude::*;
use dioxus_logger::tracing::Level;
use hangry_games::gui::components::*;
use hangry_games::gui::router::Routes;
use hangry_games::gui::states::HGState;
use hangry_games::gui::functions::list_of_games;

fn main() {
    dioxus_logger::init(Level::INFO).expect("logger failed to init");
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
    use_context_provider(|| Signal::new(HGState { games: list_of_games() }));

    rsx! {
        div {
            class: "container mx-auto",
            header::Header {}
            Router::<Routes> {}
            img { src: mg!(image("assets/hangry-games.png").preload()) }
        }
    }
}
