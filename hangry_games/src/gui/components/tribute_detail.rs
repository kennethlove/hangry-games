use dioxus::prelude::*;
use crate::models::{get_game_by_id, get_tribute_by_id};
use crate::tributes::actors::Tribute;
use crate::gui::router::Routes;

#[component]
pub fn TributeDetail(id: i32) -> Element {
    let tribute = use_signal(|| Tribute::from(get_tribute_by_id(id)));
    let avatar = tribute.read().avatar();
    let game = get_game_by_id(tribute.read().game_id.unwrap()).expect("Game not found");

    rsx! {
        Link {
            to: Routes::Home {},
            class: "flex flex-row items-center gap-2 justify-center",
            "Home"
        }
        div {
            class: "flex flex-row items-center gap-2 justify-center text-yellow-900 dark:text-yellow-500 divide-x divide-yellow-900 dark:divide-yellow-500 mb-4 underline",
            Link {
                to: Routes::Home {},
                "Home"
            }
            Link {
                to: Routes::GameDetail { id: tribute.read().game_id.unwrap() },
                class: "pl-2",
                "Back to game"
            }
        }
        div {
            class: "flex flex-row justify-left items-top gap-4",
            img {
                class: "rounded-m size-64 mr-4",
                src: avatar,
            }
            div {
                class: "dark:text-gray-300 dark:bg-yellow-100/20 backdrop-blur-sm py-4 px-6 rounded-lg",
                h1 {
                    class: "text-3xl font-bold orbitron-font tracking-wider dark:text-yellow-500",
                    "{tribute.read().name}"
                }
                h2 {
                    class: "text-xl orbitron-font font-bold tracking-wider dark:text-yellow-800",
                    "District ",
                    span {
                        class: "font-normal dark:text-yellow-500 tracking-normal",
                        "{tribute.read().district}"
                    },
                }
                h3 {
                    class: "text-large font-bold orbitron-font tracking-wider dark:text-yellow-800",
                    "Game ",
                    span {
                        class: "font-normal dark:text-yellow-500 tracking-normal",
                        "{game.name}"
                    }
                }
                dl {
                    class: "mt-4 grid grid-cols-3 gap-1",
                    dt {
                        class: "font-medium text-right pr-4",
                        "Status"
                    }
                    dd {
                        class: "col-span-2",
                        "{tribute.read().status}"
                    }
                    dt {
                        class: "font-medium text-right pr-4",
                        "Attributes"
                    }
                    dd {
                        class: "col-span-2",
                        dl {
                            class: "grid grid-cols-2 gap-1",
                            dt {
                                "Health"
                            }
                            dd {
                                "{tribute.read().health}/100"
                            }
                            dt {
                                "Sanity"
                            }
                            dd {
                                "{tribute.read().sanity}/100"
                            }
                            dt {
                                "Movement"
                            }
                            dd {
                                "{tribute.read().movement}/100"
                            }
                            dt {
                                "Strength"
                            }
                            dd {
                                "{tribute.read().strength.unwrap()}/50"
                            }
                            dt {
                                "Defense"
                            }
                            dd {
                                "{tribute.read().defense.unwrap()}/50"
                            }
                            dt {
                                "Bravery"
                            }
                            dd {
                                "{tribute.read().bravery.unwrap()}/100"
                            }
                            dt {
                                "Loyalty"
                            }
                            dd {
                                "{tribute.read().loyalty.unwrap()}/100"
                            }
                            dt {
                                "Speed"
                            }
                            dd {
                                "{tribute.read().speed.unwrap()}/100"
                            }
                            dt {
                                "Intelligence"
                            }
                            dd {
                                "{tribute.read().intelligence.unwrap()}/100"
                            }
                            dt {
                                "Persuasion"
                            }
                            dd {
                                "{tribute.read().persuasion.unwrap()}/100"
                            }
                            dt {
                                "Luck"
                            }
                            dd {
                                "{tribute.read().luck.unwrap()}/100"
                            }
                            dt {
                                "Dexterity"
                            }
                            dd {
                                "{tribute.read().dexterity.unwrap()}/100"
                            }
                        }
                    }
                    dt {
                        class: "font-medium text-right pr-4",
                        "Statistics"
                    }
                    dd {
                        class: "col-span-2",
                        dl {
                            class: "grid grid-cols-2 gap-1",
                            dt {
                                "Kills"
                            }
                            dd {
                                "{tribute.read().kills.unwrap_or(0)}"
                            }
                            dt {
                                "Wins"
                            }
                            dd {
                                "{tribute.read().wins.unwrap_or(0)}"
                            }
                            dt {
                                "Defeats"
                            }
                            dd {
                                "{tribute.read().defeats.unwrap_or(0)}"
                            }
                            dt {
                                "Draws"
                            }
                            dd {
                                "{tribute.read().draws.unwrap_or(0)}"
                            }
                            dt {
                                "Games"
                            }
                            dd {
                                "{tribute.read().games.unwrap_or(0)}"
                            }
                        }
                    }
                    if !tribute.read().is_alive() {
                        dt {
                            class: "font-medium text-right pr-4",
                            "Death"
                        }
                        dd {
                            dl {
                                class: "grid grid-cols-2 gap-1",
                                dt {
                                    "Day Killed"
                                }
                                dd {
                                    "{tribute.read().day_killed.unwrap_or(0)}"
                                }
                                dt {
                                    "Killed By"
                                }
                                dd {
                                    {
                                        let mut _killer = "Unknown";
                                        if tribute.read().killed_by.is_some() {
                                            _killer = tribute.read().killed_by.as_ref().unwrap();
                                        }
                                        "{_killer}"
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}