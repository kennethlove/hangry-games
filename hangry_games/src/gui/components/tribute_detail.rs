use dioxus::prelude::*;
use crate::models::get_tribute_by_id;
use crate::tributes::actors::Tribute;
use crate::gui::router::Routes;

#[component]
pub fn TributeDetail(id: i32) -> Element {
    let nav = navigator();
    let tribute = Tribute::from(get_tribute_by_id(id));

    rsx! {
        div {
            class: "flow-root",
            dl {
                class: "-my-3 divide-y divide-gray-100 text-sm",
                div {
                    class: "grid grid-cols-1 gap-1 py-3 sm:grid-cols-3 sm:gap-4",
                    dt {
                        class: "font-medium text-gray-900",
                        "Name"
                    }
                    dd {
                        class: "text-gray-700 sm:col-span-2",
                        "{tribute.name}"
                    }
                }
                div {
                    class: "grid grid-cols-1 gap-1 py-3 sm:grid-cols-3 sm:gap-4",
                    dt {
                        class: "font-medium text-gray-900",
                        "District"
                    }
                    dd {
                        class: "text-gray-700 sm:col-span-2",
                        "{tribute.district}"
                    }
                }
                div {
                    class: "grid grid-cols-1 gap-1 py-3 sm:grid-cols-3 sm:gap-4",
                    dt {
                        class: "font-medium text-gray-900",
                        "Status"
                    }
                    dd {
                        class: "text-gray-700 sm:col-span-2",
                        "{tribute.status}"
                    }
                }
                div {
                    class: "grid grid-cols-1 gap-1 py-3 sm:grid-cols-3 sm:gap-4",
                    dt {
                        class: "font-medium text-gray-900",
                        "Attributes"
                    }
                    dd {
                        class: "text-gray-700 sm:col-span-2",
                        dl {
                            class: "-my-3 divide-y divide-gray-100 text-sm grid grid-cols-2 gap-1",
                            dt {
                                class: "font-medium text-gray-900",
                                "Health"
                            }
                            dd {
                                class: "text-gray-700",
                                "{tribute.health}/100"
                            }
                            dt {
                                class: "font-medium text-gray-900",
                                "Sanity"
                            }
                            dd {
                                class: "text-gray-700",
                                "{tribute.sanity}/100"
                            }
                            dt {
                                class: "font-medium text-gray-900",
                                "Movement"
                            }
                            dd {
                                class: "text-gray-700",
                                "{tribute.movement}/100"
                            }
                            dt {
                                class: "font-medium text-gray-900",
                                "Strength"
                            }
                            dd {
                                class: "text-gray-700",
                                "{tribute.strength.unwrap()}/50"
                            }
                            dt {
                                class: "font-medium text-gray-900",
                                "Defense"
                            }
                            dd {
                                class: "text-gray-700",
                                "{tribute.defense.unwrap()}/50"
                            }
                            dt {
                                class: "font-medium text-gray-900",
                                "Bravery"
                            }
                            dd {
                                class: "text-gray-700",
                                "{tribute.bravery.unwrap()}/100"
                            }
                            dt {
                                class: "font-medium text-gray-900",
                                "Loyalty"
                            }
                            dd {
                                class: "text-gray-700",
                                "{tribute.loyalty.unwrap()}/100"
                            }
                            dt {
                                class: "font-medium text-gray-900",
                                "Speed"
                            }
                            dd {
                                class: "text-gray-700",
                                "{tribute.speed.unwrap()}/100"
                            }
                            dt {
                                class: "font-medium text-gray-900",
                                "Intelligence"
                            }
                            dd {
                                class: "text-gray-700",
                                "{tribute.intelligence.unwrap()}/100"
                            }
                            dt {
                                class: "font-medium text-gray-900",
                                "Persuasion"
                            }
                            dd {
                                class: "text-gray-700",
                                "{tribute.persuasion.unwrap()}/100"
                            }
                            dt {
                                class: "font-medium text-gray-900",
                                "Luck"
                            }
                            dd {
                                class: "text-gray-700",
                                "{tribute.luck.unwrap()}/100"
                            }
                            dt {
                                class: "font-medium text-gray-900",
                                "Dexterity"
                            }
                            dd {
                                class: "text-gray-700",
                                "{tribute.dexterity.unwrap()}/100"
                            }
                        }
                    }
                }
                div {
                    class: "grid grid-cols-1 gap-1 py-3 sm:grid-cols-3 sm:gap-4",
                    dt {
                        class: "font-medium text-gray-900",
                        "Statistics"
                    }
                    dd {
                        class: "text-gray-700 sm:col-span-2",
                        dl {
                            class: "-my-3 divide-y divide-gray-100 text-sm grid grid-cols-2 gap-1",
                            dt {
                                class: "font-medium text-gray-900",
                                "Kills"
                            }
                            dd {
                                class: "text-gray-700",
                                "{tribute.kills.unwrap_or(0)}"
                            }
                            dt {
                                class: "font-medium text-gray-900",
                                "Wins"
                            }
                            dd {
                                class: "text-gray-700",
                                "{tribute.wins.unwrap_or(0)}"
                            }
                            dt {
                                class: "font-medium text-gray-900",
                                "Defeats"
                            }
                            dd {
                                class: "text-gray-700",
                                "{tribute.defeats.unwrap_or(0)}"
                            }
                            dt {
                                class: "font-medium text-gray-900",
                                "Draws"
                            }
                            dd {
                                class: "text-gray-700",
                                "{tribute.draws.unwrap_or(0)}"
                            }
                            dt {
                                class: "font-medium text-gray-900",
                                "Games"
                            }
                            dd {
                                class: "text-gray-700",
                                "{tribute.games.unwrap_or(0)}"
                            }
                        }
                    }
                }
                div {
                    class: "grid grid-cols-1 gap-1 py-3 sm:grid-cols-3 sm:gap-4",
                    dt {
                        class: "font-medium text-gray-900",
                        "Death"
                    }
                    dd {
                        class: "text-gray-700 sm:col-span-2",
                        dl {
                            class: "-my-3 divide-y divide-gray-100 text-sm grid grid-cols-2 gap-1",
                            dt {
                                class: "font-medium text-gray-900",
                                "Day Killed"
                            }
                            dd {
                                class: "text-gray-700",
                                "{tribute.day_killed.unwrap_or(0)}"
                            }
                            dt {
                                class: "font-medium text-gray-900",
                                "Killed By"
                            }
                            dd {
                                class: "text-gray-700",
                                "{tribute.killed_by.unwrap_or(\"unknown\".to_string())}"
                            }
                        }
                    }
                }
            }
        }

        Link {
            class: "text-blue-700 underline",
            to: Routes::GameDetail { id: tribute.game_id.unwrap() },
            "Back"
        }
    }
}