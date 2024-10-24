use dioxus::prelude::*;
use crate::games::Game;
use crate::models::get_game_by_id;
use crate::gui::router::Routes;
use crate::gui::components::create_tribute::CreateTribute;
use crate::gui::components::tribute_table::TributeTable;
use crate::gui::components::tribute_actions_group::TributeActionsGroup;
use crate::gui::components::tribute_boxes::TributeBoxes;
use crate::gui::components::tribute_list::TributeList;

#[component]
pub fn GameDetail(id: i32) -> Element {
    let game = Game::from(get_game_by_id(id).unwrap());
    let tributes = use_signal(||game.tributes());

    rsx! {
        div {
            class: "flex justify-between items-center",
            h2 {
                class: "text-2xl font-bold text-slate-900 orbitron-font tracking-wider",
                "Game ",
                span {
                    class: "font-normal text-slate-700 tracking-normal",
                    "{game.name}"
                },
            }
            h3 {
                class: "text-lg text-slate-700 orbitron-font font-bold tracking-wider",
                span {
                    class: "font-normal text-slate-700 tracking-normal",
                    "Day "
                }
                "{game.day.unwrap_or(0)}",
            }
        }
        div {
            class: "flex justify-between items-center",
            h4 {
                class: "text-md text-slate-700 orbitron-font",
                "Day Log"
            }
            for day in 1..=game.day.unwrap_or(0) {
                Link {
                    class: "underline text-red-700",
                    to: Routes::GameDayLog { id: game.id.unwrap(), day },
                    "{day}"
                }
            }
            Link {
                class: "underline text-red-700",
                to: Routes::GameLog { id: game.id.unwrap() },
                "Full Log"
            }
        }

        if game.tributes().len() < 24 {
            CreateTribute {signal: tributes.clone(), game_id: game.id.unwrap()}
        }

        div {
            class: "mt-4",
            TributeList { tributes: tributes.clone() }
        }

        Link {
            class: "text-red-700 underline",
            to: Routes::Home { },
            "Back to Home"
        }
    }
}
