use dioxus::prelude::*;
use crate::games::Game;
use crate::models::get_game_by_id;
use crate::tributes::actors::Tribute;

#[component]
pub fn CreateTribute(signal: Signal<Vec<Tribute>>, game_id: i32) -> Element {
    let game = Game::from(get_game_by_id(game_id).unwrap());
    let mut tribute_name = use_signal(String::new);
    let tributes = use_signal(||game.tributes());

    rsx! {
        div {
            class: "bg-white overflow-hidden sm:rounded-lg mt-4",
            form {
                class: "flex justify-center",
                onsubmit: move |event| {
                    let data = event.data.values();
                    let name = data.get("tribute_name").unwrap().first().unwrap();
                    let tribute = game.add_tribute(name.clone());
                    signal.write().push(tribute.expect("Error creating tribute"));
                    tribute_name.set(String::from(""));
                },
                input {
                    r#type: "text",
                    class: "shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline",
                    id: "tribute_name",
                    name: "tribute_name",
                    placeholder: "Tribute Name",
                    value: "{tribute_name}",
                    oninput: move |event| tribute_name.set(event.value().clone()),
                    onkeypress: move |event| {
                        if event.key() == Key::Enter {
                            tribute_name.set(String::from(""))
                        }
                    }
                }
                button {
                    class: "bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded",
                    "Add Tribute"
                }
            }
        }
    }
}
