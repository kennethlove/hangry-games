use dioxus::prelude::*;
use crate::games::Game;
use crate::models::{fill_tributes, get_game_by_id};
use crate::gui::components::ShowModal;
use crate::gui::router::Routes;
use crate::gui::components::button::Button;
use crate::gui::components::create_tribute::CreateTribute;
use crate::gui::components::tribute_list::TributeList;
use crate::gui::components::fill_tributes_button::FillTributesButton;
use crate::tributes::actors::Tribute;

#[component]
pub fn GameDetail(id: i32) -> Element {
    let game = Game::from(get_game_by_id(id).unwrap());
    let tributes: Signal<Vec<Tribute>> = use_signal(|| game.tributes());
    use_context_provider(|| Signal::new(ShowModal { show: false}));

    rsx! {
        Link {
            to: Routes::Home {},
            class: "flex flex-row items-center gap-2 justify-center text-yellow-900 dark:text-yellow-500 underline",
            "Home"
        }
        div {
            class: "flex justify-between items-center",
            h2 {
                class: "text-2xl font-bold text-yellow-800 dark:text-yellow-800 orbitron-font tracking-wider",
                "Game ",
                span {
                    class: "font-normal text-red-700 dark:text-yellow-500 tracking-normal",
                    "{game.name}"
                },
            }
            if game.status == crate::games::GameStatus::InProgress ||
                (game.status == crate::games::GameStatus::NotStarted && tributes.read().len() == 24) {
                    Button {
                        text: "Play next day",
                        onclick: move |_| {
                            let nav = navigator();
                            nav.push(Routes::GamePlay { id: game.id.unwrap() });
                        },
                    }
            }
        }
        if game.day.unwrap_or(0) > 0 {
            div {
                class: "flex justify-between items-center",
                h4 {
                    class: "text-md text-yellow-800 dark:text-yellow-800 orbitron-font",
                    "Day Log"
                }
                for day in 1..=game.day.unwrap_or(0) {
                    Link {
                        class: "underline text-yellow-800 dark:text-yellow-500",
                        to: Routes::GameDayLog { id: game.id.unwrap(), day },
                        "{day}"
                    }
                }
                Link {
                    class: "underline text-yellow-800 dark:text-yellow-500",
                    to: Routes::GameLog { id: game.id.unwrap() },
                    "Full Log"
                }
            }
        }

        if game.status == crate::games::GameStatus::Finished {
            h4 {
                class: "text-xl text-red-800 dark:text-yellow-500 orbitron-font text-center mt-4",
                if game.winner().is_some() {
                    "{game.winner().unwrap().name} wins!"
                } else {
                    "No one wins!"
                }
            }
        }

        if tributes.read().len() < 24 {
            div {
                class: "items-justify mt-4 flex flex-row justify-start gap-2",
                CreateTribute {signal: tributes.clone(), game_id: game.id.unwrap()}
                span {
                    class: "leading-9 text-sm dark:text-slate-200 w-min",
                    "or"
                }
                FillTributesButton { }
            }
            ConfirmFillModal { id: game.id.unwrap(), tributes }
        }

        div {
            class: "mt-4",
            TributeList { tributes: tributes.clone(), game: game.clone() }
        }
    }
}

#[component]
fn ConfirmFillModal(id: i32, mut tributes: Signal<Vec<Tribute>>) -> Element {
    let mut state = use_context::<Signal<ShowModal>>();
    let game = get_game_by_id(id).unwrap();

    rsx! {
        dialog {
            open: state.read().show,
            class: "relative z-10",
            role: "alert",
            div { class: "v-full w-full fixed inset-0 bg-gray-500 bg-opacity-75 transition-opacity"}
            div {
                class: "fixed inset-0 z-10 w-screen overflow-y-hidden",
                div {
                    class: "flex items-center gap-4 min-h-full justify-center",
                    div {
                        class: "relative transform overflow-hidden",
                        div {
                            class: "mx-auto bg-white border border-orange-500 rounded-xl dark:bg-gray-800 p-4",
                            div {
                                class: "flex-1",
                                strong {
                                    class: "block font-medium text-gray-900 dark:text-gray-50",
                                    "Fill with tributes?"
                                }
                                p {
                                    class: "mt-1 text-sm text-gray-700 dark:text-gray-300",
                                    {format!("Are you sure you want to fill {} with tributes?", game.name)}
                                }
                            }
                            div {
                                class: "flex justify-end gap-4 mt-4",
                                Button {
                                    text: "Yes",
                                    onclick: move |_| {
                                        fill_tributes(&game);
                                        tributes.set(Game::from(game.clone()).tributes());
                                        state.write().show = false;
                                    }
                                }
                                Button {
                                    text: "No",
                                    extra_css_classes: "bg-gray-200 dark:bg-gray-500 bg-none".to_string(),
                                    onclick: move |_| {
                                        state.write().show = false;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}