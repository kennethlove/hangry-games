use dioxus::prelude::*;
use dioxus_logger::tracing::info;
use crate::gui::router::Routes;
use crate::gui::components::tribute_actions_group::TributeActionsGroup;
use crate::tributes::actors::Tribute;

#[component]
pub fn TributeListItem(tribute: Tribute, signal: Signal<Vec<Tribute>>) -> Element {
    let mut avatar = tribute.avatar.clone();
    if avatar.is_some() {
        avatar = Some(format!("{}", avatar.as_ref().unwrap()));
    } else {
        avatar = Some("https://images.unsplash.com/photo-1603871165848-0aa92c869fa1?ixlib=rb-1.2.1&ixid=MnwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8&auto=format&fit=crop&w=772&q=80".to_string());
    }

    rsx! {
        div {
            class: "rounded-full border-4 p-2 bg-gray-800 flex flex-row border-green-500 bg-gradient-to-b from-slate-700 to-slate-950",
            img {
                class: "rounded-full w-16 h-16",
                src: avatar.unwrap()
            }

            div {
                class: "ml-4 w-1/2 mt-2",
                h1 {
                    class: "text-lg text-orange-500 leading-6",
                    "{tribute.name}"
                }
                h2 {
                    class: "text-sm text-white",
                    "District {tribute.district}"
                }
            }
            div {
                class: "w-full text-sm text-white mt-2",
                dl {
                    class: "grid grid-cols-2",
                    dt {
                        class: "text-right pr-2 text-orange-500",
                        "Status"
                    }
                    dd {
                        class: "",
                        "{tribute.status}"
                    }
                    dt {
                        class: "text-right pr-2 text-orange-500",
                        "Location"
                    }
                    dd {
                        class: "",
                        "{tribute.area.unwrap()}"
                    }
                }
            }
        }
    }
}
