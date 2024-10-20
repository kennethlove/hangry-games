use dioxus::prelude::*;
use crate::games::Game;
use crate::models::get_game_by_id;
use crate::tributes::actors::Tribute;

#[component]
pub fn CreateTribute(signal: Signal<Vec<Tribute>>, game_id: i32) -> Element {
    let game = Game::from(get_game_by_id(game_id).unwrap());
    let mut tribute_name = use_signal(String::new);

    rsx! {
        div {
            class: "mt-4",
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
                    class: "block w-half mr-2 text-sm px-2 text-gray-900 border border-orange-700 rounded-md bg-yellow-200 dark:text-gray-400 focus:outline-none dark:bb-gray-700 dark:border-gray-600 dark:placeholder-gray-400 placeholder-gray-900",
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
                input {
                    class: "block w-half text-sm px-2 py-2 text-gray-900 border border-orange-700 rounded-md cursor-pointer bg-yellow-200 dark:text-gray-400 focus:outline-none dark:bb-gray-700 dark:border-gray-600 dark:placeholder-gray-400",
                    id: "file_input",
                    r#type: "file",
                    placeholder: "Upload Image",
                }

                button {
                    class: "bg-gradient-to-r from-orange-500 to-yellow-300 rounded-md text-red-800 orbitron-font font-semibold py-2 px-4 ml-2",
                    "Add Tribute"
                }
            }
        }
    }
}
