use dioxus::prelude::*;
use crate::games::Game;
use crate::models::{fill_tributes, get_game_by_id};
use crate::gui::router::Routes;
use crate::gui::components::create_tribute::CreateTribute;
use crate::gui::components::tribute_table::TributeTable;
use crate::gui::components::tribute_actions_group::TributeActionsGroup;
use crate::gui::components::tribute_boxes::TributeBoxes;
use crate::gui::components::tribute_list::TributeList;
use crate::gui::components::fill_tributes_button::FillTributesButton;
use crate::gui::functions::list_of_games;
use crate::gui::states::HGState;
use crate::tributes::actors::Tribute;

#[derive(Clone, Debug)]
pub struct ShowModal { pub(crate) show: bool }

#[component]
pub fn GameDetail(id: i32) -> Element {
    let game = Game::from(get_game_by_id(id).unwrap());
    let mut tributes: Signal<Vec<Tribute>> = use_signal(|| game.tributes());
    use_context_provider(|| Signal::new(ShowModal { show: false}));

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

        if tributes.read().len() < 24 {
            div {
                class: "flex flex-row justify-between items-center",
                CreateTribute {signal: tributes.clone(), game_id: game.id.unwrap()}
                span {
                    class: "align-bottom text-sm text-slate-700 mt-4",
                    "or"
                }
                FillTributesButton { }
            }
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

        ConfirmFillModal { id: game.id.unwrap(), tributes }

    }
}

#[component]
fn ConfirmFillModal(id: i32, mut tributes: Signal<Vec<Tribute>>) -> Element {
    let mut state = use_context::<Signal<ShowModal>>();
    let game = get_game_by_id(id).unwrap();

    rsx! {
        dialog {
            open: state.read().show,
            class: "rounded-xl border border-orange-500 bg-white p-4 dark:bg-gray-800",
            role: "alert",
            div {
                class: "flex items-start gap-4",
                div {
                    class: "flex-1",
                    strong {
                        class: "block font-medium text-gray-900 dark:text-gray-50",
                        "Fill game?"
                    }
                    p {
                        class: "mt-1 text-sm text-gray-700 dark:text-gray-300",
                        "Are you sure you want to fill the game with random tributes?"
                    }
                }
            }
            div {
                class: "flex justify-end gap-4 mt-4",
                button {
                    class: "block rounded-lg px-4 py-2 bg-orange-500",
                    onclick: move |_| {
                        fill_tributes(&game);
                        tributes.set(Game::from(game.clone()).tributes());
                        state.write().show = false;
                    },
                    span {
                        class: "text-red-800 font-semibold orbitron-font",
                        "Fill"
                    }
                }
                button {
                    class: "block rounded-lg px-4 py-2 text-red-700 bg-gray-500",
                    onclick: move |_| {
                        state.write().show = false;
                    },
                    span {
                        class: "text-red-800 font-semibold orbitron-font",
                        "Close"
                    }
                }
            }
        }
    }
}