use dioxus::prelude::*;
use crate::games::Game;
use crate::gui::router::Routes;
use crate::models::{get_game_by_id, get_logs_for_game};

#[component]
pub fn GamePlay(id: i32) -> Element {
    let mut game = Game::from(get_game_by_id(id).expect("Game not found"));
    let mut logs = get_logs_for_game(game.id.unwrap());
    logs.sort_by(|a, b| b.day.cmp(&a.day));

    let game_day_output = game.run_day_night_cycle();
    dbg!(game_day_output);

    rsx! {
        div {
            class: "flow-root",
            h1 { "Game Play" }
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