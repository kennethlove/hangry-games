use dioxus::prelude::*;
use crate::models::get_tribute_by_id;
use crate::tributes::actors::Tribute;

#[component]
pub fn TributeEdit(id: i32) -> Element {
    let tribute = Tribute::from(get_tribute_by_id(id));

    rsx! {
        div {
            h1 { "Edit Tribute" }
        }
    }
}