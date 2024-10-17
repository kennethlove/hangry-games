use std::sync::{Arc, Mutex};
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

fn app() -> Element {
    use_context_provider(|| Signal::new(HGState { games: list_of_games() }));

    rsx! {
        head {
            link {
                rel: "stylesheet",
                href: "https://cdn.jsdelivr.net/npm/tailwindcss@2.0.2/dist/tailwind.min.css"
            }
        }
        div {
            class: "container mx-auto",
            header::Header {}
            Router::<Routes> {}
        }
    }
}

