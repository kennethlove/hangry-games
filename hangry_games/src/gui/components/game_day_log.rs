use dioxus::prelude::*;
use crate::games::Game;
use crate::gui::router::Routes;
use crate::models::{get_game_by_id, get_logs_for_game_day};

#[component]
pub fn GameDayLog(id: i32, day: i32) -> Element {
    let game = Game::from(get_game_by_id(id).unwrap());

    rsx! {
        Link {
            to: Routes::Home {},
            class: "flex flex-row items-center gap-2 justify-center",
            "Home"
        }
        div {
            class: "flex flex-row items-center gap-2 justify-center text-yellow-900 dark:text-yellow-500 divide-x divide-yellow-900 dark:divide-yellow-500 mb-4 underline",
            Link {
                to: Routes::Home {},
                "Home"
            }
            Link {
                to: Routes::GameDetail { id: game.id.unwrap() },
                class: "pl-2",
                "Back to game"
            }
            Link {
                to: Routes::GameLog { id: game.id.unwrap() },
                class: "pl-2",
                "Back to log"
            }
        }
        div {
            h2 {
                class: "text-2xl font-bold text-yellow-900 dark:text-yellow-700 orbitron-font tracking-wider",
                "Game ",
                span {
                    class: "font-normal text-red-700 dark:text-yellow-500 tracking-normal",
                    "{game.name}"
                }
            }
            div {
                h2 {
                    class: "text-xl font-bold orbitron-font tracking-wider text-yellow-800",
                    "Day "
                    span {
                        class: "font-normal text-red-800 dark:text-yellow-500 tracking-normal",
                        "{day}"
                    }
                }
                ol {
                    class: "indent-4 mb-4 text-yellow-900 dark:text-yellow-200",
                    for log in get_logs_for_game_day(game.id.unwrap(), day).iter() {
                        li { "{log.message}" }
                    }
                }
            }
        }
    }
}