use dioxus::prelude::*;
use crate::games::Game;
use crate::gui::components::tribute_list_item::TributeListItem;
use crate::tributes::actors::Tribute;
use crate::gui::components::{SelectedItem, ShowModal};
use crate::gui::components::button::Button;

#[component]
pub fn TributeList(tributes: Signal<Vec<Tribute>>, game: Game) -> Element {
    use_context_provider(|| Signal::new(ShowModal { show: false}));
    use_context_provider(|| Signal::new(SelectedItem { id: -1 }));

    rsx! {
        div {
            class: "grid xl:grid-cols-2 xl:gap-4",
            ConfirmDeleteTributeModal { id: 0, tributes: tributes.clone() }
            for tribute_pair in tributes.read().chunks(2) {
                div {
                    class: "grid grid-cols-2 gap-1",
                    span {
                        class:"flex items-center col-span-2 mb-2",
                        span { class:"h-px flex-1 bg-red-800 dark:bg-yellow-500" }
                        span { class:"shrink-0 px-6 text-red-800 dark:text-yellow-500", "District {tribute_pair[0].district}" }
                        span { class:"h-px flex-1 bg-red-800 dark:bg-yellow-500" }
                    }

                    for tribute in tribute_pair {
                        TributeListItem { tribute: tribute.clone(), signal: tributes.clone(), game: game.clone() }
                    }
                }
            }
        }
    }
}

#[component]
fn ConfirmDeleteTributeModal(id: i32, mut tributes: Signal<Vec<Tribute>>) -> Element {
    let mut state = use_context::<Signal<ShowModal>>();
    let mut selected_tribute = use_context::<Signal<SelectedItem>>();
    let tributes_read = tributes.read();
    if selected_tribute.read().id == -1 {
        return rsx! {};
    }
    let tribute = tributes_read.iter().find(|t| t.id.unwrap() == selected_tribute.read().id).unwrap();

    rsx! {
        dialog {
            open: state.read().show,
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
                                    "Delete tribute?"
                                }
                                p {
                                    class: "mt-1 text-sm text-gray-700 dark:text-gray-300",
                                    {format!("Are you sure you want to delete {}?", tribute.name)}
                                }
                            }
                            div {
                                class: "flex justify-end gap-4 mt-4",
                                Button {
                                    text: "Yes",
                                    onclick: move |_| {
                                        Tribute::delete(selected_tribute.read().id);
                                        tributes.write().retain(|t| t.id.unwrap() != selected_tribute.read().id);
                                        selected_tribute.write().id = -1;
                                        state.write().show = false;
                                    }
                                }
                                Button {
                                    text: "No",
                                    extra_css_classes: "bg-gray-500 bg-none".to_string(),
                                    onclick: move |_| {
                                        selected_tribute.write().id = -1;
                                        state.write().show = false;
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
