use dioxus::prelude::*;
use crate::games::Game;
use crate::gui::states::HGState;
use crate::gui::components::game_table_row::GameTableRow;
use crate::gui::components::{SelectedItem, ShowModal};

#[component]
pub fn GameTable() -> Element {
    let state = use_context::<Signal<HGState>>();
    use_context_provider(|| Signal::new(ShowModal { show: false}));
    use_context_provider(|| Signal::new(SelectedItem { id: -1 }));

    rsx! {
        div {
            ConfirmDeleteGameModal {}
            table {
                class: "min-w-full mt-4 divide-y divide-yellow-200/50",
                thead {
                    class: "text-sm text-left text-red-950 tracking-wide font-semibold",
                    tr {
                        class: "",
                        th {
                            class: "whitespace-nowrap font-semibold pl-4",
                            "Game Name"
                        }
                        th {
                            class: "font-semibold",
                            "Day"
                        }
                        th {
                            class: "whitespace-nowrap font-semibold",
                            "Living Tributes"
                        }
                        th {
                            class: "font-semibold text-right pr-4",
                            "Actions"
                        }
                    }
                }
                tbody {
                    class: "text-slate-800",
                    for game in state.read().games.iter() {
                        GameTableRow { game: game.clone() }
                    }
                }
            }
        }
    }
}

#[component]
fn ConfirmDeleteGameModal() -> Element {
    let mut show_modal = use_context::<Signal<ShowModal>>();
    let mut selected_game = use_context::<Signal<SelectedItem>>();
    let mut state = use_context::<Signal<HGState>>();
    let game_state = state.read();
    let game = game_state.games.iter().find(|t| t.id.unwrap() == selected_game.read().id).unwrap();

    rsx! {
        dialog {
            open: show_modal.read().show,
            class: "relative z-10",
            role: "alert",
            div { class: "fixed inset-0 bg-gray-500 bg-opacity-75 transition-opacity"}
            div {
                class: "fixed inset-0 z-10 w-screen overflow-y-hidden",
                div {
                    class: "flex items-center gap-4 min-h-full justify-center",
                    div {
                        class: "relative transform overflow-hidden",
                        div {
                            class: "mx-auto bg-white border border-orange-500 rounded-xl dark:bg-gray-800 p-4",
                            div {
                                class: "flex-1",
                                strong {
                                    class: "block font-medium text-gray-900 dark:text-gray-50",
                                    "Delete game?"
                                }
                                p {
                                    class: "mt-1 text-sm text-gray-700 dark:text-gray-300",
                                    {format!("Are you sure you want to delete {}?", game.name)}
                                }
                            }
                            div {
                                class: "flex justify-end gap-4 mt-4",
                                button {
                                    class: "block rounded-lg px-4 py-2 bg-orange-500",
                                    onclick: move |_| {
                                        Game::delete(selected_game.read().id);
                                        state.write().games.retain(|g| g.id.unwrap() != selected_game.read().id);
                                        selected_game.write().id = -1;
                                        show_modal.write().show = false;
                                    },
                                    span {
                                        class: "text-red-800 orbitron-font",
                                        "Yes"
                                    }
                                }
                                button {
                                    class: "block rounded-lg px-4 py-2 text-red-700 bg-gray-500",
                                    onclick: move |_| {
                                        selected_game.write().id = -1;
                                        show_modal.write().show = false;
                                    },
                                    span {
                                        class: "text-red-800 orbitron-font",
                                        "Cancel"
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
