use dioxus::prelude::*;
use crate::games::Game;
use crate::models::get_game_by_id;
use crate::gui::router::Routes;
use crate::gui::components::create_tribute::CreateTribute;
use crate::gui::components::tribute_table::TributeTable;
use crate::gui::components::tribute_actions_group::TributeActionsGroup;
use crate::gui::components::tribute_table::TributeTable;

#[component]
pub fn GameDetail(id: i32) -> Element {
    let game = Game::from(get_game_by_id(id).unwrap());
    let tributes = use_signal(||game.tributes());

    rsx! {
        div {
            class: "flow-root",
            dl {
                class: "-my-3 divide-y divide-gray-100 text-sm",
                div {
                    class: "grid grid-cols-1 gap-1 py-3 sm:grid-cols-3 sm:gap-4",
                    dt {
                        class: "font-medium text-gray-900",
                        "Name"
                    }
                    dd {
                        class: "text-gray-700 sm:col-span-2",
                        "{game.name}"
                    }
                }
                div {
                    class: "grid grid-cols-1 gap-1 py-3 sm:grid-cols-3 sm:gap-4",
                    dt {
                        class: "font-medium text-gray-900",
                        "Day"
                    }
                    dd {
                        class: "text-gray-700 sm:col-span-2",
                        "{game.day.unwrap_or(0)}"
                    }
                }
                div {
                    class: "grid grid-cols-1 gap-1 py-3 sm:grid-cols-3 sm:gap-4",
                    dt {
                        class: "font-medium text-gray-900",
                        "Tributes"
                    }
                    dd {
                        class: "text-gray-700 sm:col-span-2",
                        TributeTable { tributes: tributes.clone() }
                    }
                }
            }
        }

        if game.tributes().len() < 24 {
            CreateTribute {signal: tributes.clone(), game_id: game.id.unwrap()}
        }

        Link {
            to: Routes::Home {},
            class: "underline text-blue-500",
            "Home"
        }
    }
}
