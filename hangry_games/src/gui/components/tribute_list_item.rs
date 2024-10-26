use dioxus::prelude::*;
use dioxus_logger::tracing::info;
use serde::__private::de::IdentifierDeserializer;
use crate::gui::router::Routes;
use crate::gui::components::tribute_actions_group::TributeActionsGroup;
use crate::tributes::actors::Tribute;

#[component]
pub fn TributeListItem(tribute: Tribute, signal: Signal<Vec<Tribute>>) -> Element {
    let avatar = Some(
        format!("{}", tribute.avatar.as_ref().unwrap_or(&"hangry-games.png".to_string()))
    );

    let surrounding_border = match tribute.health {
        1..=25 => "border-red-500",
        26..=50 => "border-yellow-500",
        51..=75 => "border-green-500",
        76..=100 => "border-blue-500",
        _ => "border-gray-900",
    };

    let gradient_stop = match tribute.health {
        1..=25 => "from-gray-900 to-red-700",
        26..=50 => "from-gray-900 to-yellow-700",
        51..=75 => "from-gray-900 to-green-700",
        76..=100 => "from-gray-900 to-blue-700",
        _ => "from-gray-900 to-gray-700",
    };

    rsx! {
        div {
            class: "rounded-full border-4 p-2 mb-2 bg-gray-800 flex flex-row bg-gradient-to-b {gradient_stop}",
            img {
                class: "rounded-full border-2 {surrounding_border} mr-2 min-h-16 max-h-16 min-w-16 max-w-16",
                src: avatar.unwrap(),
            }

            div {
                class: "w-1/2 mt-2",
                h1 {
                    class: "text-lg text-orange-500 leading-none",
                    Link {
                        to: Routes::TributeDetail { id: tribute.id.unwrap() },
                        "{tribute.name}"
                    }
                }
                h2 {
                    class: "text-sm text-white",
                    "District {tribute.district}"
                }
            }
            div {
                class: "text-xs text-white mt-1",
                div {
                    class: "flex flex-row gap-1",
                    span {
                        class: "text-orange-500 material-symbols-outlined",
                        "monitor_heart"
                    }
                    span {
                        class: "mt-1",
                        "{tribute.status}"
                    }
                }
                div {
                    class: "flex flex-row",
                    span {
                        class: "text-orange-500 material-symbols-outlined",
                        "location_on"
                    }
                    span {
                        class: "mt-1",
                        "{tribute.area.unwrap()}"
                    }
                }
            }
        }
    }
}
