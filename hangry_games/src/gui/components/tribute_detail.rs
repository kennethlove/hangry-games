use dioxus::prelude::*;
use crate::models::{get_game_by_id, get_tribute_by_id};
use crate::tributes::actors::Tribute;
use crate::gui::router::Routes;

#[component]
pub fn TributeDetail(id: i32) -> Element {
    let tribute = Tribute::from(get_tribute_by_id(id));
    let avatar = tribute.avatar();
    let game = get_game_by_id(tribute.game_id.unwrap()).expect("Game not found");

    rsx! {
        Link {
            to: Routes::Home {},
            class: "flex flex-row items-center gap-2 justify-center",
            "Home"
        }
        div {
            class: "flex flex-row justify-left items-top gap-4",
            img {
                class: "rounded-m size-64",
                src: avatar,
            }
            div {
                h1 {
                    class: "text-3xl font-bold text-slate-900 orbitron-font tracking-wider",
                    "{tribute.name}"
                }
                h2 {
                    class: "text-xl text-slate-900 orbitron-font font-bold tracking-wider",
                    "District ",
                    span {
                        class: "font-normal text-slate-700 tracking-normal",
                        "{tribute.district}"
                    },
                }
                h3 {
                    class: "text-large font-bold text-slate-700 orbitron-font tracking-wider",
                    "Game ",
                    Link {
                        to: Routes::GameDetail { id: game.id },
                        class: "font-normal text-red-700 tracking-normal",
                        "{game.name}"
                    },
                }
                dl {
                    class: "mt-4 grid grid-cols-3 gap-1",
                    dt {
                        class: "font-medium text-gray-900 text-right pr-4",
                        "Status"
                    }
                    dd {
                        class: "col-span-2",
                        "{tribute.status}"
                    }
                    dt {
                        class: "font-medium text-gray-900 text-right pr-4",
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
                                "{tribute.health}/100"
                            }
                            dt {
                                "Sanity"
                            }
                            dd {
                                "{tribute.sanity}/100"
                            }
                            dt {
                                "Movement"
                            }
                            dd {
                                "{tribute.movement}/100"
                            }
                            dt {
                                "Strength"
                            }
                            dd {
                                "{tribute.strength.unwrap()}/50"
                            }
                            dt {
                                "Defense"
                            }
                            dd {
                                "{tribute.defense.unwrap()}/50"
                            }
                            dt {
                                "Bravery"
                            }
                            dd {
                                "{tribute.bravery.unwrap()}/100"
                            }
                            dt {
                                "Loyalty"
                            }
                            dd {
                                "{tribute.loyalty.unwrap()}/100"
                            }
                            dt {
                                "Speed"
                            }
                            dd {
                                "{tribute.speed.unwrap()}/100"
                            }
                            dt {
                                "Intelligence"
                            }
                            dd {
                                "{tribute.intelligence.unwrap()}/100"
                            }
                            dt {
                                "Persuasion"
                            }
                            dd {
                                "{tribute.persuasion.unwrap()}/100"
                            }
                            dt {
                                "Luck"
                            }
                            dd {
                                "{tribute.luck.unwrap()}/100"
                            }
                            dt {
                                "Dexterity"
                            }
                            dd {
                                "{tribute.dexterity.unwrap()}/100"
                            }
                        }
                    }
                    dt {
                        class: "font-medium text-gray-900 text-right pr-4",
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
                                "{tribute.kills.unwrap_or(0)}"
                            }
                            dt {
                                "Wins"
                            }
                            dd {
                                "{tribute.wins.unwrap_or(0)}"
                            }
                            dt {
                                "Defeats"
                            }
                            dd {
                                "{tribute.defeats.unwrap_or(0)}"
                            }
                            dt {
                                "Draws"
                            }
                            dd {
                                "{tribute.draws.unwrap_or(0)}"
                            }
                            dt {
                                "Games"
                            }
                            dd {
                                "{tribute.games.unwrap_or(0)}"
                            }
                        }
                    }
                    if !tribute.is_alive() {
                        dt {
                            class: "font-medium text-gray-900 text-right pr-4",
                            "Death"
                        }
                        dd {
                            dl {
                                class: "grid grid-cols-2 gap-1",
                                dt {
                                    "Day Killed"
                                }
                                dd {
                                    "{tribute.day_killed.unwrap_or(0)}"
                                }
                                dt {
                                    "Killed By"
                                }
                                dd {
                                    if tribute.killed_by.is_none() {
                                        ""
                                    } else {
                                        "{tribute.killed_by.unwrap()}"
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