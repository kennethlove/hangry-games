use dioxus::prelude::*;
use crate::games::Game;
use crate::gui::components::game_detail::ShowModal;
use crate::models::{fill_tributes, get_game_by_id};

#[component]
pub fn FillTributesButton() -> Element {
    let mut state = use_context::<Signal<ShowModal>>();

    rsx! {
        div {
            class: "mt-4",
            button {
                class: "bg-gradient-to-r from-orange-500 to-yellow-300 rounded-md text-red-800 orbitron-font font-semibold py-2 px-4 b-1 border border-orange-700",
                onclick: move |_| {
                    state.write().show = true;
                },
                "Fill game"
            }
        }
    }
}
