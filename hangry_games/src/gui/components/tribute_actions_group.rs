use dioxus::prelude::*;
use crate::gui::states::HGState;
use crate::gui::router::Routes;
use crate::tributes::actors::Tribute;

#[component]
pub fn TributeActionsGroup(tribute: Tribute, signal: Signal<Vec<Tribute>>) -> Element {
    rsx! {
        div {
            class: "inline-flex rounded-lg bg-gradient-to-r from-orange-500 to-yellow-300 p-1",
            TributeDeleteButton { tribute: tribute.clone(), signal: signal }
            TributeDetailsButton { tribute: tribute.clone() }
        }
    }
}

#[component]
fn TributeDeleteButton(tribute: Tribute, signal: Signal<Vec<Tribute>>) -> Element {
    rsx! {
        button {
            class: "inline-block rounded-md px-4 py-2 text-sm font-normal text-slate-800 hover:text-red-700 focus:relative",
            onclick: move |_| {
                Tribute::delete(tribute.id.unwrap());
                signal.write().retain(|t| t.id != tribute.id);
            },
            span {
                class: "material-symbols-outlined",
                "delete"
            }
        }
    }
}

#[component]
fn TributeDetailsButton(tribute: Tribute) -> Element {
    let _state = use_context::<Signal<HGState>>();
    let nav = navigator();

    rsx! {
        button {
            class: "inline-block rounded-md px-4 py-2 text-sm font-normal text-slate-800 hover:text-blue-700 focus:relative",
            onclick: move |_| {
                nav.push(Routes::TributeDetail { id: tribute.id.unwrap() });
            },
            span {
                class: "material-symbols-outlined",
                "zoom_in"
            }
        }
    }
}
