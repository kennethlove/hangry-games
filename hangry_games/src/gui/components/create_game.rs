use dioxus::prelude::*;
use crate::games::Game;
use crate::gui::states::HGState;
use crate::gui::router::Routes;

#[component]
pub fn CreateGame() -> Element {
    let mut state = use_context::<Signal<HGState>>();
    let mut game_name = use_signal(String::new);
    let nav = navigator();

    rsx! {
        div {
            class: "mt-4",
            form {
                class: "flex justify-center",
                onsubmit: move |event| {
                    let data = event.data.values();
                    let game_name = data.get("game_name").unwrap().first().unwrap();
                    let game = Game::new(game_name);
                    state.write().games.push(game.clone());
                    nav.push(Routes::GameDetail { id: game.id.unwrap() });
                },
                label {
                    r#for: "game_name",
                    class: "",

                    input {
                        class: "shadow appearance-none border rounded-sm py-2 px-3 text-red-800 leading-tight focus:outline-none focus:shadow-outline",
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
                }
                button {
                    class: "bg-gradient-to-r from-orange-500 to-yellow-300 rounded-sm text-red-800 orbitron-font font-semibold py-2 px-4 ml-2",
                    onclick: move |_| {
                    },
                    "Create Game"
                }
            }
        }
    }
}
