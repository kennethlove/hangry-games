use std::path::Path;
use std::sync::Arc;
use dioxus::prelude::*;
use dioxus::{prelude::dioxus_elements::FileEngine};
use crate::games::Game;
use crate::models::get_game_by_id;
use crate::tributes::actors::Tribute;

#[derive(Clone, Debug)]
struct UploadedFile {
    name: String,
    contents: Vec<u8>
}

#[component]
pub fn CreateTribute(signal: Signal<Vec<Tribute>>, game_id: i32) -> Element {
    let game = Game::from(get_game_by_id(game_id).unwrap());
    let mut tribute_name = use_signal(String::new);
    let mut files_uploaded = use_signal(|| Vec::new() as Vec<UploadedFile>);

    let read_files = move |file_engine: Arc<dyn FileEngine>| async move {
        let files = file_engine.files();
        for file_name in &files {
            if let Some(contents) = file_engine.read_file(file_name).await {
                files_uploaded.write().push(UploadedFile {
                    name: file_name.clone(),
                    contents: contents.clone()
                });
            }
        }
    };

    let upload_files = move |evt: FormEvent| async move {
        if let Some(file_engine) = evt.files() {
            read_files(file_engine).await;
        }
    };

    rsx! {
        div {
            class: "mt-4",
            form {
                class: "flex justify-center",
                enctype: "multipart/form-data",
                onsubmit: move |event| {
                    let data = event.data.values();
                    let name = data.get("tribute_name").unwrap().first().unwrap();
                    let image = files_uploaded.read();
                    let image = image.first().unwrap().clone();

                    let extension = Path::new(&image.name).extension().unwrap().to_str().unwrap().to_lowercase();
                    let filename = format!("{}.{}", name.to_lowercase(), extension);
                    let avatar_path = format!("avatars/{}/", game_id);
                    let save_path = format!("./assets/{}/", avatar_path);
                    let tribute = game.add_tribute(name.clone(), Some(format!("{}{}", avatar_path, filename)));

                    std::fs::create_dir_all(&save_path).expect("Unable to create directory");
                    std::fs::write(format!("{}{}", save_path, filename), &image.contents).expect("Unable to write file");

                    signal.write().push(tribute.expect("Error creating tribute"));
                    tribute_name.set(String::from(""));
                },
                input {
                    r#type: "text",
                    class: "block w-half mr-2 text-sm px-2 text-gray-900 border border-orange-700 rounded-md bg-yellow-200 focus:outline-none placeholder-gray-900",
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
                    class: "block w-half text-sm px-2 py-2 text-gray-900 border border-orange-700 rounded-md cursor-pointer bg-yellow-200 focus:outline-none",
                    id: "file_input",
                    r#type: "file",
                    placeholder: "Upload Image",
                    accept: "image/png,image/gif,image/jpg",
                    multiple: false,

                    onchange: upload_files
                }

                button {
                    class: "bg-gradient-to-r from-orange-500 to-yellow-300 rounded-md text-red-800 orbitron-font font-semibold py-2 px-4 ml-2 border border-orange-700",
                    "Add Tribute"
                }
            }
        }
    }
}
