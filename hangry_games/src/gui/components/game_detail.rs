use dioxus::prelude::*;
use crate::games::Game;
use crate::models::get_game_by_id;
use crate::gui::router::Routes;
use crate::gui::components::create_tribute::CreateTribute;
use crate::gui::components::tribute_table::TributeTable;
use crate::gui::components::tribute_actions_group::TributeActionsGroup;
use crate::gui::components::tribute_boxes::TributeBoxes;

#[component]
pub fn GameDetail(id: i32) -> Element {
    let game = Game::from(get_game_by_id(id).unwrap());
    let tributes = use_signal(||game.tributes());

    rsx! {
        div {
            class: "flex justify-between items-center",
            h2 {
                class: "text-2xl font-bold text-slate-900",
                "{game.name}"
            }
            h3 {
                class: "text-lg text-slate-700",
                "Day {game.day.unwrap_or(0)}"
            }
        }
        div {
            class: "mt-4",
            TributeBoxes { tributes: tributes.clone() }
        }

        if game.tributes().len() < 24 {
            CreateTribute {signal: tributes.clone(), game_id: game.id.unwrap()}
        }

        Link {
            class: "text-red-700 underline",
            to: Routes::Home { },
            "Back"
        }
    }
}
