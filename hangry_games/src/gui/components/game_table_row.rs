use dioxus::prelude::*;
use crate::games::Game;
use crate::gui::components::game_actions_group::GameActionsGroup;

#[component]
pub fn GameTableRow(game: Game) -> Element {
    rsx! {
        tr {
            class: "group hover:text-orange-200",
            td {
                class: "whitespace-nowrap pl-4",
                "{game.name}"
            }
            td {
                "{game.day.unwrap_or(0)}"
            }
            td {
                "{game.tributes().len()}/24"
            }
            td {
                class: "flex justify-end pr-2 py-2",
                GameActionsGroup { game: game.clone() }
            }
        }
    }
}

