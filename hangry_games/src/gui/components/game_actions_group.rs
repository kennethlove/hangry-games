use dioxus::prelude::*;
use crate::games::Game;
use crate::gui::states::{HGState, SelectedGame};
use crate::gui::router::Routes;

#[component]
pub fn GameActionsGroup(game: Game) -> Element {
    rsx! {
        div {
            class: "inline-flex rounded-lg border border-gray-100 bg-gray-100 p-1",
            GameDeleteButton { game: game.clone() }
            GameDetailsButton { game: game.clone() }
            GamePlayButton { game: game.clone() }
        }
    }
}

#[component]
fn GameDeleteButton(game: Game) -> Element {
    let mut state = use_context::<Signal<HGState>>();
    rsx! {
        button {
            class: "inline-block rounded-md px-4 py-2 text-sm text-gray-500 hover:text-red-700 focus:relative",
            onclick: move |_| {
                Game::delete(game.id.unwrap());
                state.write().games.retain(|g| g.id != game.id);
            },
            "Delete"
        }
    }
}

#[component]
fn GameDetailsButton(game: Game) -> Element {
    let _state = use_context::<Signal<HGState>>();
    let nav = navigator();

    rsx! {
        button {
            class: "inline-block rounded-md px-4 py-2 text-sm text-gray-500 hover:text-blue-700 focus:relative",
            onclick: move |_| {
                let mut selected_game = use_context::<Signal<SelectedGame>>();
                selected_game.set(SelectedGame(Some(game.id.unwrap())));
                nav.push(Routes::GameDetail {});
            },
            "Details"
        }
    }
}

#[component]
fn GamePlayButton(game: Game) -> Element {
    let _state = use_context::<Signal<HGState>>();
    rsx! {
        button {
            class: "inline-block rounded-md px-4 py-2 text-sm text-gray-500 hover:text-green-700 focus:relative",
            onclick: move |_| {
            },
            "Play"
        }
    }
}
