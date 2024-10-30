use dioxus::prelude::*;
use crate::gui::components::tribute_list_item::TributeListItem;
use crate::tributes::actors::Tribute;

#[component]
pub fn TributeList(tributes: Signal<Vec<Tribute>>) -> Element {
    rsx! {
        div {
            class: "grid xl:grid-cols-2 xl:gap-4",
            for tribute_pair in tributes.read().chunks(2) {
                div {
                    class: "grid grid-cols-2 gap-1",
                    span {
                        class:"flex items-center col-span-2 mb-2",
                        span { class:"h-px flex-1 bg-black" }
                        span { class:"shrink-0 px-6", "District {tribute_pair[0].district}" }
                        span { class:"h-px flex-1 bg-black" }
                    }

                    for tribute in tribute_pair {
                        TributeListItem { tribute: tribute.clone(), signal: tributes.clone() }
                    }
                }
            }
        }
    }
}

