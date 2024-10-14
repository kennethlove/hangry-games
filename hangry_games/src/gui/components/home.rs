use dioxus::prelude::*;
use crate::games::Game;
use crate::gui::components::{
    game_table::GameTable,
    create_game::CreateGame,
};
use crate::gui::functions::list_of_games;

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

