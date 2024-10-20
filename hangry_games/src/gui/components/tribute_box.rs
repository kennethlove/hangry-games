use dioxus::prelude::*;
use crate::gui::router::Routes;
use crate::gui::components::tribute_actions_group::TributeActionsGroup;
use crate::tributes::actors::Tribute;

#[component]
pub fn TributeBox(tribute: Tribute, signal: Signal<Vec<Tribute>>) -> Element {
    rsx! {
        a {
            class: "group relative block bg-black",
            img {
                class: "absolute inset-0 h-full w-full object-cover opacity-75 transition-opacity group-hover:opacity-50",
                src: "https://images.unsplash.com/photo-1603871165848-0aa92c869fa1?ixlib=rb-1.2.1&ixid=MnwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8&auto=format&fit=crop&w=772&q=80"
            }

            div {
                class: "relative p-4 sm:p-6 lg:p-8",
                p {
                    class: "text-sm font-medium uppercase tracking-widest text-orange-500",
                    "District {tribute.district}"
                }
                p {
                    class: "text-xl font-bold text-white sm:text-2xl",
                    "{tribute.name}"
                }
                div {
                    class: "mt-32 sm:mt-48 lg:mt-64",
                    div {
                        class: "translate-y-8 transform opacity-0 transition-all group-hover:translate-y-0 group-hover:opacity-100",
                        p {
                            class: "w-full text-center text-sm text-slate-400 mb-4 font-normal",
                            span { class: "material-symbols-outlined text-sm", "favorite" }, " {tribute.status} ",
                            span { class: "material-symbols-outlined text-sm", "vital_signs" }, " {tribute.health} ",
                            span { class: "material-symbols-outlined text-sm", "cognition_2" }, " {tribute.sanity} ",
                        }
                        TributeActionsGroup { tribute: tribute.clone(), signal: signal.clone() }
                    }
                }
            }
        }
    }
}
