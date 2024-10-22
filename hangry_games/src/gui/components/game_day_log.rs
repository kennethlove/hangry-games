use dioxus::prelude::*;
use crate::games::Game;
use crate::gui::router::Routes;
use crate::models::{get_game_by_id, get_logs_for_game_day};

#[component]
pub fn GameDayLog(id: i32, day: i32) -> Element {
    let game = Game::from(get_game_by_id(id).expect("Game not found"));

    rsx! {
        div {
            class: "flow-root",
            h1 { "Game Play" }
            h2 {
                class: "text-xl font-bold",
                "Day {day}"
            }
            h3 {
                "{game.id.unwrap()}"
            }
            ol {
                for log in get_logs_for_game_day(game.id.unwrap(), day).iter() {
                    li { "{log.message}" }
                }
            }
            Link {
                class: "text-blue-500 underline",
                to: Routes::GamePlay { id: game.id.unwrap() },
                "Back to Game Play"
            }
            Link {
                class: "text-blue-500 underline",
                to: Routes::GameDetail { id: game.id.unwrap() },
                "Back to Game"
            }
            Link {
                class: "text-blue-500 underline",
                to: Routes::Home {},
                "Back to Home"
            }
        }
    }
}