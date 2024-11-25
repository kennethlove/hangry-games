use dioxus::prelude::*;
use crate::games::Game;
use crate::gui::router::Routes;
use crate::models::{get_game_by_id, get_logs_for_game_day, LogEntry};

#[component]
pub fn GameLog(id: i32) -> Element {
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
        }
        div {
            h2 {
                class: "text-2xl font-bold text-yellow-800 orbitron-font tracking-wider",
                "Game ",
                span {
                    class: "font-normal text-red-800 dark:text-yellow-500 tracking-normal",
                    "{game.name}"
                }
            }
            div {
                class: "flex flex-direction-col justify-between",
                div {
                    for day in 1..=game.day.unwrap() {
                        div {
                            h2 {
                                class: "text-xl font-bold orbitron-font tracking-wider text-yellow-800",
                                id: "day-{day}",
                                "Day "
                                span {
                                    class: "font-normal text-red-800 dark:text-yellow-500 tracking-normal",
                                    "{day}"
                                }
                            }
                            ol {
                                class: "indent-4 mb-4 text-yellow-900 dark:text-yellow-200",
                                for log in get_logs_for_game_day(game.id.unwrap(), day).iter() {
                                    LogListItem { log: log.clone() }
                                }
                            }
                        }
                    }
                }
                div {
                    h2 {
                        class: "text-lg orbitron-font font-bold tracking-wider text-yellow-800",
                        "Days"
                    }
                    ol {
                        for day in 1..=game.day.unwrap() {
                            li {
                                class: "text-yellow-900 dark:text-yellow-500 underline",
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
    }
}

#[component]
pub fn LogListItem(log: LogEntry) -> Element {
    let classes = match log.tribute_id {
        Some(_) => "text-red-800 dark:text-yellow-500",
        None => "text-yellow-800 dark:text-red-500 text-center",
    };
    let tribute = log.tribute();
    if let Some(tribute) = tribute {
        rsx! {
            li {
                class: classes,
                "{log.message} by {tribute.name}"
            }
        }
    } else {
        rsx! {
            li {
                class: classes,
                "{log.message}"
            }
        }
    }
}