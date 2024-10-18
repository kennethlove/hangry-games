use dioxus::prelude::*;
use crate::gui::states::HGState;
use crate::gui::components::game_table_row::GameTableRow;

#[component]
pub fn GameTable() -> Element {
    let state = use_context::<Signal<HGState>>();

    rsx! {
        div {
            table {
                class: "min-w-full divide-y-2 divide-gray-200 bg-white text-sm",
                thead {
                    class: "ltr:text-left rtl:text-right",
                    tr {
                        th {
                            class: "whitespace-nowrap px-4 py-2 font-medium text-gray-900",
                            "Game Name"
                        }
                        th {
                            class: "whitespace-nowrap px-4 py-2 font-medium text-gray-900",
                            "Day"
                        }
                        th {
                            class: "whitespace-nowrap px-4 py-2 font-medium text-gray-900",
                            "Tributes"
                        }
                        th {
                            class: "px-4 py-2",
                            "Actions"
                        }
                    }
                }
                tbody {
                    class: "divide-y divide-gray-200",
                    for game in state.read().games.iter() {
                        GameTableRow { game: game.clone() }
                    }
                }
            }
        }
    }
}

