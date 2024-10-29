use dioxus::prelude::*;
use crate::gui::components::game_detail::ShowModal;

#[component]
pub fn FillTributesButton() -> Element {
    let mut state = use_context::<Signal<ShowModal>>();

    rsx! {
        button {
            class: "orbitron-font b-1 w-min whitespace-nowrap rounded-md border border-orange-700 bg-gradient-to-r from-orange-500 to-yellow-300 px-2 py-1 text-red-800",
            onclick: move |_| {
                state.write().show = true;
            },
            "Fill game"
        }
    }
}
