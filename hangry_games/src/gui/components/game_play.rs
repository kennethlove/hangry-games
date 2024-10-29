use dioxus::prelude::*;
use crate::games::{Game, GameStatus};
use crate::gui::router::Routes;
use crate::models::get_game_by_id;

#[component]
pub fn GamePlay(id: i32) -> Element {
    let mut game = Game::from(get_game_by_id(id).expect("Game not found"));
    let nav = navigator();
    game.run_day_night_cycle();

    if game.status == GameStatus::InProgress && game.living_tributes().len() <= 1 {
        game.status = GameStatus::Finished;
        game.end();
    }

    nav.replace(Routes::GameDayLog { id: game.id.unwrap(), day: game.day.unwrap_or(0) });

    rsx! { "Playing game" }
}