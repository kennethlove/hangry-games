use dioxus::prelude::*;
use crate::gui::components::{
    game_table::GameTable,
    create_game::CreateGame,
};
use crate::gui::router::Routes;

#[component]
pub fn Home() -> Element {
    rsx! {
        CreateGame {}
        GameTable {}
    }
}

