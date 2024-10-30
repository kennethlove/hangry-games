use dioxus::prelude::*;
use crate::gui::components::tribute_list_item::TributeListItem;
use crate::models::{get_game_by_id, get_tribute_by_id};
use crate::tributes::actors::Tribute;
use crate::gui::components::ShowModal;

#[derive(Clone, Debug)]
pub(crate) struct SelectedTribute { pub id: i32 }

#[component]
pub fn TributeList(tributes: Signal<Vec<Tribute>>) -> Element {
    use_context_provider(|| Signal::new(ShowModal { show: false}));
    use_context_provider(|| Signal::new(SelectedTribute { id: -1 }));

    rsx! {
        div {
            class: "grid xl:grid-cols-2 xl:gap-4",
            ConfirmDeleteTributeModal { id: 0, tributes: tributes.clone() }
            for tribute_pair in tributes.read().chunks(2) {
                div {
                    class: "grid grid-cols-2 gap-1",
                    span {
                        class:"flex items-center col-span-2 mb-2",
                        span { class:"h-px flex-1 bg-black" }
                        span { class:"shrink-0 px-6", "District {tribute_pair[0].district}" }
                        span { class:"h-px flex-1 bg-black" }
                    }

                    for tribute in tribute_pair {
                        TributeListItem { tribute: tribute.clone(), signal: tributes.clone() }
                    }
                }
            }
        }
    }
}

#[component]
fn ConfirmDeleteTributeModal(id: i32, mut tributes: Signal<Vec<Tribute>>) -> Element {
    let mut state = use_context::<Signal<ShowModal>>();
    let selected_tribute = use_context::<Signal<SelectedTribute>>();
    let tributes = tributes.read();
    let tribute = tributes.iter().find(|t| t.id.unwrap() == selected_tribute.read().id).unwrap();

    rsx! {
        dialog {
            open: state.read().show,
            class: "relative z-10 rounded-xl border border-orange-500 bg-white p-4 dark:bg-gray-800",
            role: "alert",
            div { class: "fixed inset-0 bg-gray-500 bg-opacity-75 transition-opacity"}
            div {
                class: "fixed inset-0 z-10 w-screen overflow-y-hidden",
                div {
                    class: "flex items-start gap-4 min-h-full items-end justify-center",
                    div {
                        class: "flex-1",
                        strong {
                            class: "block font-medium text-gray-900 dark:text-gray-50",
                            "Delete tribute?"
                        }
                        p {
                            class: "mt-1 text-sm text-gray-700 dark:text-gray-300",
                            {format!("Are you sure you want to delete {}?", tribute.name)}
                        }
                    }
                }
                div {
                    class: "flex justify-end gap-4 mt-4",
                    button {
                        class: "block rounded-lg px-4 py-2 bg-orange-500",
                        onclick: move |_| {
                            state.write().show = false;
                        },
                        span {
                            class: "text-red-800 font-semibold orbitron-font",
                            "Yes"
                        }
                    }
                    button {
                        class: "block rounded-lg px-4 py-2 text-red-700 bg-gray-500",
                        onclick: move |_| {
                            state.write().show = false;
                        },
                        span {
                            class: "text-red-800 font-semibold orbitron-font",
                            "Cancel"
                        }
                    }
                }
            }
        }
    }
}
