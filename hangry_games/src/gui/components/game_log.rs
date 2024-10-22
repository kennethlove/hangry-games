use dioxus::prelude::*;
use crate::games::Game;
use crate::gui::router::Routes;
use crate::models::{get_game_by_id, get_logs_for_game_day};

#[component]
pub fn GameLog(id: i32) -> Element {
    let game = Game::from(get_game_by_id(id).unwrap());

    rsx! {
        div {
            h2 {
                class: "text-2xl font-bold text-slate-900 orbitron-font tracking-wider",
                "Game ",
                Link {
                    to: Routes::GameDetail { id: game.id.unwrap() },
                    class: "font-normal text-red-700 tracking-normal",
                    "{game.name}"
                },
            }
            div {
                class: "flex flex-direction-col justify-between",
                div {
                    for day in 1..=game.day.unwrap() {
                        div {
                            h2 {
                                class: "text-xl font-bold orbitron-font tracking-wider",
                                "Day {day}"
                            }
                            ol {
                                class: "indent-4 mb-4",
                                for log in get_logs_for_game_day(game.id.unwrap(), day).iter() {
                                    li { "{log.message}" }
                                }
                            }
                        }
                    }
                }
                div {
                    h2 {
                        class: "text-lg orbitron-font font-bold tracking-wider",
                        "Days"
                    }
                    ol {
                        for day in 1..=game.day.unwrap() {
                            li {
                                class: "text-red-700 underline",
                                Link {
                                    to: Routes::GameDayLog { id: game.id.unwrap(), day },
                                    "Day {day}"
                                }
                            }
                        }
                    }
                }
            }
        }
        Link {
            class: "text-red-700 underline",
            to: Routes::Home {},
            "Back to Home"
        }
    }
}