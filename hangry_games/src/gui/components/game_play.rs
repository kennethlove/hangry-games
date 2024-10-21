use dioxus::prelude::*;
use crate::games::Game;
use crate::gui::router::Routes;
use crate::models::{get_game_by_id, get_logs_for_game_day};

#[component]
pub fn GamePlay(id: i32) -> Element {
    let game = Game::from(get_game_by_id(id).expect("Game not found"));

    rsx! {
        div {
            class: "flow-root",
            h1 { "Game Play" }
            for day in 1..=game.day.unwrap() {
                div {
                    h2 {
                        class: "text-xl font-bold",
                        "Day {day}"
                    }
                    ol {
                        for log in get_logs_for_game_day(game.id.unwrap(), day).iter() {
                            li { "{log.message}" }
                        }
                    }
                }
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