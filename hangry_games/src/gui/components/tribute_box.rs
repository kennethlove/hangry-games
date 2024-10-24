use dioxus::prelude::*;
use dioxus_logger::tracing::info;
use crate::gui::router::Routes;
use crate::gui::components::tribute_actions_group::TributeActionsGroup;
use crate::tributes::actors::Tribute;

#[component]
pub fn TributeBox(tribute: Tribute, signal: Signal<Vec<Tribute>>) -> Element {
    let mut avatar = tribute.avatar.clone();
    if avatar.is_some() {
        avatar = Some(format!("{}", avatar.as_ref().unwrap()));
    } else {
        avatar = Some("https://images.unsplash.com/photo-1603871165848-0aa92c869fa1?ixlib=rb-1.2.1&ixid=MnwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8&auto=format&fit=crop&w=772&q=80".to_string());
    }

    rsx! {
        div {
            class: "group relative block bg-gradient-to-b to-orange-500 from-yellow-300 rounded-md overflow-hidden",
            img {
                class: "absolute inset-0 h-full w-full object-cover opacity-100 transition-opacity group-hover:opacity-75",
                src: avatar.unwrap()
            }

            div {
                class: "relative p-4 sm:p-6 lg:p-8",
                p {
                    class: "text-sm font-medium uppercase tracking-widest text-orange-500",
                    "District {tribute.district}"
                }
                p {
                    class: "text-xl font-bold text-white sm:text-2xl drop-shadow",
                    "{tribute.name}"
                }
                div {
                    class: "mt-32 sm:mt-48 lg:mt-64",
                    div {
                        class: "translate-y-8 transform opacity-0 transition-all group-hover:translate-y-0 group-hover:opacity-100",
                        p {
                            class: "w-full text-center text-sm text-slate-200 mb-4 font-normal divide-x divide-gray-300",
                            span { class: "material-symbols-outlined text-sm", "favorite" }, " {tribute.status} ",
                            span { class: "material-symbols-outlined text-sm ml-2 pl-2", "vital_signs" }, " {tribute.health} ",
                            span { class: "material-symbols-outlined text-sm ml-2 pl-2", "cognition_2" }, " {tribute.sanity} ",
                        }
                        TributeActionsGroup { tribute: tribute.clone(), signal: signal.clone() }
                    }
                }
            }
        }
    }
}
