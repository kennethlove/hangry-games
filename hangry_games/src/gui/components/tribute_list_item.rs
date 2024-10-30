use dioxus::prelude::*;
use crate::gui::router::Routes;
use crate::tributes::actors::Tribute;

#[component]
pub fn TributeListItem(tribute: Tribute, signal: Signal<Vec<Tribute>>) -> Element {
    let mut avatar = Some(
        format!("{}", tribute.avatar.as_ref().unwrap_or(&"hangry-games.png".to_string()))
    );

    if cfg!(target_family = "windows") {
        avatar = Some(
            format!("assets/{}", avatar.unwrap())
        );
    }

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
            class: "group relative block overflow-hidden rounded-full border-4 p-2 mb-2 bg-gray-800 bg-gradient-to-b {gradient_stop}",
            div {
                class: "flex flex-row gap-2",
                img {
                    class: "rounded-full border-3 {surrounding_border} mr-2 size-20 basis-20",
                    src: avatar.unwrap(),
                }

                div {
                    class: "w-1/2 mt-2 flex-grow",
                    h1 {
                        class: "text-lg text-orange-500 leading-none",
                        Link {
                            to: Routes::TributeDetail { id: tribute.id.unwrap() },
                            "{tribute.name}"
                        }
                    }
                    div {
                        class: "text-xs text-white flex flex-row gap-0",
                        span {
                            class: "text-orange-500 material-symbols-outlined text-sm",
                            "location_on"
                        }
                        span {
                            class: "whitespace-nowrap",
                            "{tribute.area.unwrap()}"
                        }
                    }
                    div {
                        class: "text-xs text-white flex flex-row gap-1",
                        span {
                            class: "text-orange-500 material-symbols-outlined text-sm",
                            "monitor_heart"
                        }
                        span {
                            class: "",
                            "{tribute.status}"
                        }
                    }
                }
                span {
                    class: "text-9xl text-white absolute top-0 right-0 opacity-25",
                    "{tribute.district}"
                }
                div {
                    class: "absolute w-full top-12 translate-y-4 transform opacity-0 transition-all group-hover:opacity-100",
                    ul {
                        class: "flex flex-row justify-end gap-4 pr-9",
                        li {
                            class: "lineheight-0 cursor-pointer",
                            span {
                                class: "text-white material-symbols-outlined",
                                title: "Edit Tribute",
                                "edit_square"
                            }
                        }
                        li {
                            class: "lineheight-0 cursor-pointer",
                            span {
                                class: "text-white material-symbols-outlined",
                                title: "Delete Tribute",
                                "delete"
                            }
                        }
                    }
                }
            }
        }
    }
}
