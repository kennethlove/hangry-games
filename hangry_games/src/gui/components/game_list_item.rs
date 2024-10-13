use dioxus::prelude::*;
use crate::games::Game;
use crate::gui::states::SelectedGame;
use crate::gui::components::game_actions_group::GameActionsGroup;
use crate::gui::router::Routes;

#[component]
pub fn GameListItem(game: Game) -> Element {
    let mut selected_game = use_context::<Signal<SelectedGame>>();
    rsx! {
        tr {
            td {
                class: "whitespace-nowrap px-4 py-2 text-gray-700",
                Link {
                    onclick: move |_| { selected_game.set(SelectedGame(Some(game.id.unwrap()))) },
                    to: Routes::GameDetail {},
                    "{game.name}"
                }
            }
            td {
                "{game.day.unwrap_or(0)}"
            }
            td {
                "{game.living_tributes().len()}/24"
            }
            td {
                class: "flex-end",
                GameActionsGroup { game: game.clone() }
            }
        }
    }
}

