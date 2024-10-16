use dioxus::prelude::*;
use crate::gui::states::HGState;
use crate::gui::router::Routes;
use crate::tributes::actors::Tribute;

#[component]
pub fn TributeActionsGroup(tribute: Tribute, signal: Signal<Vec<Tribute>>) -> Element {
    rsx! {
        div {
            class: "inline-flex rounded-lg border border-gray-100 bg-gray-100 p-1",
            TributeDeleteButton { tribute: tribute.clone(), signal: signal }
            TributeDetailsButton { tribute: tribute.clone() }
        }
    }
}

#[component]
fn TributeDeleteButton(tribute: Tribute, signal: Signal<Vec<Tribute>>) -> Element {
    rsx! {
        button {
            class: "inline-block rounded-md px-4 py-2 text-sm text-gray-500 hover:text-red-700 focus:relative",
            onclick: move |_| {
                Tribute::delete(tribute.id.unwrap());
                signal.write().retain(|t| t.id != tribute.id);
            },
            "Delete"
        }
    }
}

#[component]
fn TributeDetailsButton(tribute: Tribute) -> Element {
    let _state = use_context::<Signal<HGState>>();
    let nav = navigator();

    rsx! {
        button {
            class: "inline-block rounded-md px-4 py-2 text-sm text-gray-500 hover:text-blue-700 focus:relative",
            onclick: move |_| {
                nav.push(Routes::TributeDetail { id: tribute.id.unwrap() });
            },
            "Details"
        }
    }
}
