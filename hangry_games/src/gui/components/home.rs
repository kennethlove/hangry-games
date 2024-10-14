use dioxus::prelude::*;
use crate::gui::components::{
    game_table::GameTable,
    create_game::CreateGame,
};

#[component]
pub fn Home() -> Element {
    rsx! {
        p { "Welcome to the Hangry Games!" }
        div {
            GameTable {}
        }
        div {
            CreateGame {}
        }
    }
}

