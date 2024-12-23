use dioxus::prelude::*;
use crate::games::Game;
use crate::gui::states::HGState;
use crate::gui::components::game_table_row::GameTableRow;
use crate::gui::components::{SelectedItem, ShowModal};
use crate::gui::components::button::Button;

#[component]
pub fn GameTable() -> Element {
    let state = use_context::<Signal<HGState>>();
    use_context_provider(|| Signal::new(ShowModal { show: false}));
    use_context_provider(|| Signal::new(SelectedItem { id: -1 }));

    rsx! {
        div {
            ConfirmDeleteGameModal {}
            table {
                class: "min-w-full mt-4 divide-y dark:divide-yellow-200 divide-yellow-500",
                thead {
                    class: "text-sm text-left dark:text-yellow-500 text-yellow-900 tracking-wide font-semibold",
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
                            class: "font-semibold",
                            "Winner"
                        }
                        th {
                            class: "font-semibold text-right pr-4",
                            "Actions"
                        }
                    }
                }
                tbody {
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
    if selected_game.read().id == -1 {
        return rsx! {};
    }
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
                                Button {
                                    text: "Yes",
                                    onclick: move |_| {
                                        Game::delete(selected_game.read().id);
                                        state.write().games.retain(|g| g.id.unwrap() != selected_game.read().id);
                                        selected_game.write().id = -1;
                                        show_modal.write().show = false;
                                    }
                                }
                                Button {
                                    text: "No",
                                    extra_css_classes: "bg-none bg-gray-200 dark:bg-gray-500".to_string(),
                                    onclick: move |_| {
                                        selected_game.write().id = -1;
                                        show_modal.write().show = false;
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
