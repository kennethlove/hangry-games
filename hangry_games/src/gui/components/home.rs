use dioxus::prelude::*;
use crate::gui::components::{
    game_table::GameTable,
    create_game::CreateGame,
};

#[component]
pub fn Home() -> Element {
    rsx! {
        div {
            class: "",
            div {
                GameTable {}
            }
            div {
                CreateGame {}
            }
        }
    }
}

