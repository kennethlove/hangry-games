use dioxus::prelude::*;
use crate::gui::router::Routes;
use crate::gui::components::tribute_actions_group::TributeActionsGroup;
use crate::tributes::actors::Tribute;

#[component]
pub fn TributeBox(tribute: Tribute, signal: Signal<Vec<Tribute>>) -> Element {
    rsx! {
        div {
            class: "rounded-lg bg-gradient-to-r from-orange-500 to-yellow-300 p-4",
            div {
                class: "flex justify-between",
                h2 {
                    class: "text-2xl font-bold text-slate-900",
                    "{tribute.name}"
                },
                p {
                    class: "text-lg text-slate-700",
                    "{tribute.district}"
                },
            }
            p {
                class: "text-sm text-slate-700 mb-4 font-semibold",
                span { class: "material-symbols-outlined text-sm", "favorite" }, " {tribute.status} ",
                span { class: "material-symbols-outlined text-sm", "vital_signs" }, " {tribute.health} ",
                span { class: "material-symbols-outlined text-sm", "cognition_2" }, " {tribute.sanity} ",
            }
            TributeActionsGroup { tribute: tribute.clone(), signal: signal.clone() }
        }
    }
}
