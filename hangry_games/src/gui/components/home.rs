use dioxus::prelude::*;
use crate::games::Game;
use crate::gui::components::{
    game_list::GameList,
    create_game::CreateGame,
};
use crate::gui::functions::list_of_games;

#[component]
pub fn Home() -> Element {
    let games_signal: Signal<Vec<Game>> = use_signal(||list_of_games());
    rsx! {
        p { "Welcome to the Hangry Games!" }
        div {
            GameList { games: games_signal.clone() }
        }
        div {
            {CreateGame()}
        }
    }
}

