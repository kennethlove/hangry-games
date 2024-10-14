use dioxus::prelude::*;
use crate::games::Game;
use crate::models::get_game_by_id;
use crate::gui::router::Routes;
use crate::gui::states::SelectedGame;
use crate::gui::components::create_tribute::CreateTribute;

#[component]
pub fn GameDetail() -> Element {
    let selected_game = use_context::<Signal<SelectedGame>>();
    let game = Game::from(get_game_by_id(selected_game.read().0.unwrap()).unwrap());
    let tributes = use_signal(||game.tributes());
    let living_tributes = use_signal(||game.living_tributes());
    let dead_tributes = game.dead_tributes();

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
                        "{game.name}"
                    }
                }
                div {
                    class: "grid grid-cols-1 gap-1 py-3 sm:grid-cols-3 sm:gap-4",
                    dt {
                        class: "font-medium text-gray-900",
                        "Day"
                    }
                    dd {
                        class: "text-gray-700 sm:col-span-2",
                        "{game.day.unwrap_or(0)}"
                    }
                }
                div {
                    class: "grid grid-cols-1 gap-1 py-3 sm:grid-cols-3 sm:gap-4",
                    dt {
                        class: "font-medium text-gray-900",
                        "Living Tributes"
                    }
                    dd {
                        class: "text-gray-700 sm:col-span-2",
                        ul {
                            class: "divide-y divide-gray-200",
                            for tribute in living_tributes.read().iter() {
                                li {
                                    class: "flex items-center py-3",
                                    "{tribute.name} from District {tribute.district}"
                                }
                            }
                        }
                    }
                }
                div {
                    class: "grid grid-cols-1 gap-1 py-3 sm:grid-cols-3 sm:gap-4",
                    dt {
                        class: "font-medium text-gray-900",
                        "Dead Tributes"
                    }
                    dd {
                        class: "text-gray-700 sm:col-span-2",
                        ul {
                            class: "divide-y divide-gray-200",
                            for tribute in dead_tributes.iter() {
                                li {
                                    class: "flex items-center py-3",
                                    "{tribute.name} from District {tribute.district}"
                                }
                            }
                        }
                    }
                }
            }
        }

        if game.tributes().len() < 24 {
            CreateTribute {signal: living_tributes.clone()}
        }

        Link {
            to: Routes::Home {},
            class: "underline text-blue-500",
            "Home"
        }
    }
}
