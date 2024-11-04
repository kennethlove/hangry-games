use dioxus::prelude::*;
use crate::games::Game;
use crate::gui::states::HGState;
use crate::gui::router::Routes;
use crate::gui::components::button::Button;

#[component]
pub fn CreateGame() -> Element {
    let mut state = use_context::<Signal<HGState>>();
    let mut game_name = use_signal(String::new);
    let nav = navigator();

    rsx! {
        div {
            class: "mt-4",
            form {
                class: "flex flex-row justify-center gap-2",
                onsubmit: move |event| {
                    let data = event.data.values();
                    let game_name = data.get("game-name").unwrap().first().unwrap();
                    let game = Game::new(game_name);
                    state.write().games.push(game.clone());
                    nav.push(Routes::GameDetail { id: game.id.unwrap() });
                },
                label {
                    r#for: "game-name",
                    class: "sr-only",
                    "Game Name",
                }
                input {
                    class: "block w-half px-2 py-1 text-gray-900 border border-orange-700 rounded-md bg-yellow-200 focus:outline-none placeholder-gray-900",
                    r#type: "text",
                    placeholder: "Game Name",
                    id: "game-name",
                    name: "game-name",
                    value: "{game_name}",
                    oninput: move |event| game_name.set(event.value().clone()),
                    onkeypress: move |event| {
                        if event.key() == Key::Enter {
                            game_name.set(String::from(""))
                        }
                    }
                }
                Button { text: "Create Game" }
            }
        }
    }
}
