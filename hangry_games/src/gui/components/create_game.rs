use dioxus::prelude::*;
use crate::games::Game;
use crate::gui::states::{HGState, SelectedGame};
use crate::gui::router::Routes;

#[component]
pub fn CreateGame() -> Element {
    let mut state = use_context::<Signal<HGState>>();
    let mut game_name = use_signal(String::new);
    let nav = navigator();

    rsx! {
        div {
            class: "bg-white overflow-hidden sm:rounded-lg mt-4",
            form {
                onsubmit: move |event| {
                    let data = event.data.values();
                    let game_name = data.get("game_name").unwrap().first().unwrap();
                    let game = Game::new(game_name);
                    let mut selected_game = use_context::<Signal<SelectedGame>>();
                    selected_game.set(SelectedGame(Some(game.id.unwrap())));
                    state.write().games.push(game);
                    nav.push(Routes::GameDetail {});
                },
                label {
                    r#for: "game_name",
                    class: "relative block rounded-md border border-gray-200 shadow-sm focus-within:border-blue-600 focus-within:ring-1 focus-within:ring-blue-600",

                    input {
                        class: "peer border-none bg-transparent placeholder-transparent focus:border-transparent focus:outline-none focus:ring-0",
                        placeholder: "Game Name",
                        r#type: "text",
                        placeholder: "Game Name",
                        id: "game_name",
                        name: "game_name",
                        value: "{game_name}",
                        oninput: move |event| game_name.set(event.value().clone()),
                        onkeypress: move |event| {
                            if event.key() == Key::Enter {
                                game_name.set(String::from(""))
                            }
                        }
                    }

                    span {
                        class: "pointer-events-none absolute start-2.5 top-0 -translate-y-1/2 bg-white p-0.5 text-xs text-gray-700 transition-all peer-placeholder-shown:text-sm peer-focus:top-0 peer-focus:text-xs",
                        "Game Name"
                    }
                }
                button {
                    class: "inline-block rounded border border-indigo-600 bg-indigo-600 px-12 py-3 text-sm font-medium text-white hover:bg-transparent hover:text-indigo-600 focus:outline-none focus:ring active:text-indigo-500",
                    onclick: move |_| {
                    },
                    "Create Game"
                }
            }
        }
    }
}
