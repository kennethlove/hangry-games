use dioxus::prelude::*;
use crate::games::{Game, GameStatus};
use crate::gui::components::game_actions_group::GameActionsGroup;

#[component]
pub fn GameTableRow(game: Game) -> Element {
    rsx! {
        tr {
            class: "group text-yellow-700 dark:text-yellow-700 hover:text-yellow-900 dark:hover:text-yellow-300",
            td {
                class: "whitespace-nowrap pl-4 rounded-l-lg",
                "{game.name}"
            }
            td {
                "{game.day.unwrap_or(0)}"
            }
            td {
                "{game.living_tributes().len()}/24"
            }
            td {
                {
                    match game.winner() {
                        Some(winner) => winner.name,
                        None => {
                            match game.status {
                                GameStatus::Finished => "No winner".to_string(),
                                _ => "".to_string(),
                            }
                        }
                    }
                }
            }
            td {
                class: "flex justify-end pr-2 py-2",
                GameActionsGroup { game: game.clone() }
            }
        }
    }
}

