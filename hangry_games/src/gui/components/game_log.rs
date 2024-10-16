use dioxus::prelude::*;
use crate::games::Game;
use crate::gui::router::Routes;
use crate::models::{get_game_by_id, get_logs_for_game};

#[component]
pub fn GameLog(id: i32) -> Element {
    let game = Game::from(get_game_by_id(id).expect("Game not found"));
    let mut logs = get_logs_for_game(game.id.unwrap());
    logs.sort_by(|a, b| b.day.cmp(&a.day));

    rsx! {
        div {
            class: "flow-root",
            h1 { "Game Log" }
            div {
                ol {
                    for log in logs.iter() {
                        li { "{log.message} from day {log.day}" }
                    }
                }
            }
            Link {
                class: "text-blue-500 underline",
                to: Routes::GameDetail { id: game.id.unwrap() },
                "Back to Game"
            }
        }
    }
}