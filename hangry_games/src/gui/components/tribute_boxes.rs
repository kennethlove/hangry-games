use dioxus::prelude::*;
use crate::gui::router::Routes;
use crate::gui::components::tribute_box::TributeBox;
use crate::tributes::actors::Tribute;

#[component]
pub fn TributeBoxes(tributes: Signal<Vec<Tribute>>) -> Element {
    rsx! {
        div {
            class: "grid grid-cols-2 gap-4 md:grid-cols-2 lg:grid-cols-4 xl:grid-cols-6",
            for tribute in tributes.read().iter() {
                TributeBox { tribute: tribute.clone(), signal: tributes.clone() }
            }
        }
    }
}
