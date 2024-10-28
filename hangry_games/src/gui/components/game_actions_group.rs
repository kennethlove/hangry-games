use dioxus::prelude::*;
use crate::games::{Game, GameStatus};
use crate::gui::states::HGState;
use crate::gui::router::Routes;

#[component]
pub fn GameActionsGroup(game: Game) -> Element {
    rsx! {
        div {
            class: "inline-flex rounded-lg bg-gradient-to-r from-orange-500 to-yellow-300 p-1, divide-x w-full",
            GameDeleteButton { game: game.clone() }
            GameDetailsButton { game: game.clone() }
            GameLogButton { game: game.clone() }
            GamePlayButton { game: game.clone() }
        }
    }
}

#[component]
fn GameDeleteButton(game: Game) -> Element {
    let mut state = use_context::<Signal<HGState>>();
    rsx! {
        button {
            class: "inline-block px-4 py-2 text-sm font-normal text-slate-800 hover:text-red-700 focus:relative w-full",
            title: "Delete Game",
            onclick: move |_| {
                Game::delete(game.id.unwrap());
                state.write().games.retain(|g| g.id != game.id);
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
            class: "inline-block px-4 py-2 text-sm font-normal text-slate-800 hover:text-blue-700 focus:relative w-full",
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
    let mut classes = "inline-block px-4 py-2 text-sm font-normal text-slate-800 hover:text-green-700 focus:relative w-full".to_string();
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
    let mut classes = "inline-block px-4 py-2 text-sm font-normal text-slate-800 hover:text-green-700 focus:relative w-full".to_string();
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
