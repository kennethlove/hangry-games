use std::path::Path;
use std::sync::Arc;
use dioxus::prelude::*;
use dioxus::{prelude::dioxus_elements::FileEngine};
use crate::games::Game;
use crate::gui::components::UploadedFile;
use crate::models::get_game_by_id;
use crate::tributes::actors::Tribute;
use crate::gui::components::button::Button;


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
        form {
            class: "flex flex-row justify-items-stretch gap-2",
            enctype: "multipart/form-data",
            onsubmit: move |event| {
                let data = event.data.values();
                let name = data.get("tribute_name").unwrap().first().unwrap();
                let image = files_uploaded.read();
                let mut new_avatar_path = None;

                if image.len() != 0 {
                    let image = image.first().unwrap().clone();

                    let extension = Path::new(&image.name).extension().unwrap().to_str().unwrap().to_lowercase();
                    let filename = format!("{}.{}", name.to_lowercase(), extension);
                    let avatar_path = format!("avatars/{}/", game_id);
                    let save_path = format!("./assets/{}/", avatar_path);

                    std::fs::create_dir_all(&save_path).expect("Unable to create directory");
                    std::fs::write(format!("{}{}", save_path, filename), &image.contents).expect("Unable to write file");
                    new_avatar_path = Some(format!("{}{}", avatar_path, filename));
                }
                let tribute = game.add_tribute(name.clone(), new_avatar_path);

                signal.write().push(tribute.expect("Error creating tribute"));
                tribute_name.set(String::from(""));
            },
            input {
                r#type: "text",
                class: "w-full rounded-md border border-orange-700 bg-yellow-200 px-2 py-1 text-gray-900 placeholder-gray-900 focus:outline-none",
                id: "tribute_name",
                name: "tribute_name",
                placeholder: "Name",
                value: "{tribute_name}",
                oninput: move |event| tribute_name.set(event.value().clone()),
                onkeypress: move |event| {
                    if event.key() == Key::Enter {
                        tribute_name.set(String::from(""))
                    }
                }
            }
            input {
                class: "w-full cursor-pointer rounded-md border border-orange-700 bg-yellow-200 px-2 py-1 text-gray-900 focus:outline-none",
                id: "file_input",
                r#type: "file",
                placeholder: "Upload Image",
                accept: "image/png,image/gif,image/jpg",
                multiple: false,

                onchange: upload_files
            }

            Button { text: "Add Tribute" }
        }
    }
}
