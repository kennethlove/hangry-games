use dioxus::prelude::*;
use crate::gui::router::Routes;
use crate::gui::components::tribute_list_item::TributeListItem;
use crate::tributes::actors::Tribute;

#[component]
pub fn TributeList(tributes: Signal<Vec<Tribute>>) -> Element {
    rsx! {
        div {
            class: "grid grid-cols-2 lg:grid-cols-4 xl:grid-cols-6 gap-1",
            for tribute in tributes.read().iter() {
                TributeListItem { tribute: tribute.clone(), signal: tributes.clone() }
            }
        }
    }
}

