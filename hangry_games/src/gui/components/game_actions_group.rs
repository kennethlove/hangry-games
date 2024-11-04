use dioxus::prelude::*;
use crate::games::{Game, GameStatus};
use crate::gui::components::{SelectedItem, ShowModal};
use crate::gui::router::Routes;

#[component]
pub fn GameActionsGroup(game: Game) -> Element {
    rsx! {
        div {
            class: "inline-flex items-end rounded-lg bg-gradient-to-r from-orange-500 to-yellow-200 p-1 divide-x divide-yellow-200/50 border border-yellow-200/50 h-min opacity-50 group-hover:opacity-80",
            GameDeleteButton { game: game.clone() }
            GameDetailsButton { game: game.clone() }
            GameLogButton { game: game.clone() }
            GamePlayButton { game: game.clone() }
        }
    }
}

#[component]
fn GameDeleteButton(game: Game) -> Element {
    let mut show_modal = use_context::<Signal<ShowModal>>();
    let mut selected_game = use_context::<Signal<SelectedItem>>();

    rsx! {
        button {
            class: "inline-block p-2 text-sm font-normal text-slate-800 hover:text-red-700 focus:relative",
            title: "Delete Game",
            onclick: move |_| {
                selected_game.write().id = game.id.unwrap();
                show_modal.write().show = true;
            },
            span {
                class: "material-symbols-outlined",
                "delete"
            }
        }
    }
}

#[component]
fn GameDetailsButton(game: Game) -> Element {
    let nav = navigator();

    rsx! {
        button {
            class: "inline-block p-2 text-sm font-normal text-slate-800 hover:text-blue-700 focus:relative w-full",
            title: "View Game Details",
            onclick: move |_| {
                nav.push(Routes::GameDetail { id: game.id.unwrap() });
            },
            span {
                class: "material-symbols-outlined",
                "zoom_in"
            }
        }
    }
}

#[component]
fn GamePlayButton(game: Game) -> Element {
    let nav = navigator();
    let mut classes = "inline-block p-2 text-sm font-normal text-slate-800 hover:text-green-700 focus:relative w-full".to_string();
    if game.tributes().len() < 24 || game.status == GameStatus::Finished {
        classes += " hidden";
    }
    rsx! {
        button {
            class: classes,
            title: "Play Next Day",
            onclick: move |_| {
                if game.status == GameStatus::NotStarted {
                    game.start();
                }
                nav.push(Routes::GamePlay { id: game.id.unwrap() });
            },
            span {
                class: "material-symbols-outlined",
                "play_arrow"
            }
        }
    }
}
#[component]
fn GameLogButton(game: Game) -> Element {
    let nav = navigator();
    let mut classes = "inline-block p-2 text-sm font-normal text-slate-800 hover:text-green-700 focus:relative w-full".to_string();
    if game.tributes().len() != 24 || game.status == GameStatus::NotStarted {
        classes += " hidden";
    }
    rsx! {
        button {
            class: classes,
            title: "View Game Log",
            onclick: move |_| {
                nav.push(Routes::GameLog { id: game.id.unwrap() });
            },
            span {
                class: "material-symbols-outlined",
                "web_stories"
            }
        }
    }
}
