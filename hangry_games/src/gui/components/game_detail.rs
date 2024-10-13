use dioxus::prelude::*;
use crate::games::Game;
use crate::models::get_game_by_id;
use crate::gui::router::Routes;
use crate::gui::states::SelectedGame;

#[component]
pub fn GameDetail() -> Element {
    let selected_game = use_context::<Signal<SelectedGame>>();
    let game = Game::from(get_game_by_id(selected_game.read().0.unwrap()).unwrap());
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
                            for tribute in game.living_tributes() {
                                li {
                                    class: "flex items-center py-3",
                                    "{tribute.name}"
                                }
                            }
                        }
                    }
                }
            }
        }

        if game.tributes().len() == 24 {
            div {
                class: "flow-root",
                h2 {
                    class: "text-lg font-medium text-gray-900",
                    "Game is Full"
                }
            }
        }

        Link {
            to: Routes::Home {},
            class: "underline text-blue-500",
            "Home"
        }
    }
}
