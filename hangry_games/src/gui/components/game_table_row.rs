use dioxus::prelude::*;
use crate::games::Game;
use crate::gui::components::game_actions_group::GameActionsGroup;
use crate::gui::router::Routes;

#[component]
pub fn GameTableRow(game: Game) -> Element {
    rsx! {
        tr {
            td {
                class: "whitespace-nowrap px-4 py-2 text-gray-700",
                Link {
                    to: Routes::GameDetail { id: game.id.unwrap() },
                    "{game.name}"
                }
            }
            td {
                "{game.day.unwrap_or(0)}"
            }
            td {
                "{game.tributes().len()}/24"
            }
            td {
                class: "flex-end",
                GameActionsGroup { game: game.clone() }
            }
        }
    }
}

