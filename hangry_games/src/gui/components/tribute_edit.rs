use std::path::Path;
use std::sync::Arc;
use dioxus::html::FileEngine;
use dioxus::prelude::*;
use crate::games::Game;
use crate::gui::components::UploadedFile;
use crate::gui::router::Routes;
use crate::models::{get_game_by_id, get_tribute_by_id, UpdateTribute};
use crate::tributes::actors::Tribute;

#[component]
pub fn TributeEdit(id: i32) -> Element {
    let nav = navigator();
    let tribute = use_signal(|| Tribute::from(get_tribute_by_id(id)));
    let mut tribute_name = use_signal(|| tribute.read().name.clone());
    let mut files_uploaded = use_signal(|| Vec::new() as Vec<crate::gui::components::UploadedFile>);

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
            class: "flex flex-row items-center gap-2 justify-center",
            Link {
                to: Routes::Home {},
                "Home"
            }
            Link {
                to: Routes::GameDetail { id: tribute.read().game_id.unwrap() },
                "Back to game"
            }
        }
        div {
            class: "grid grid-row justify-left items-top gap-4",
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
                        let avatar_path = format!("avatars/{}/", tribute.read().game_id.unwrap());
                        let save_path = format!("./assets/{}/", avatar_path);

                        std::fs::create_dir_all(&save_path).expect("Unable to create directory");
                        std::fs::write(format!("{}{}", save_path, filename), &image.contents).expect("Unable to write file");
                        new_avatar_path = Some(format!("{}{}", avatar_path, filename));
                    }

                    let update = UpdateTribute {
                        id,
                        name: name.clone(),
                        district: tribute.read().district,
                        health: tribute.read().health,
                        sanity: tribute.read().sanity,
                        movement: tribute.read().movement,
                        area_id: Some(tribute.read().area.clone().unwrap().id()),
                        game_id: tribute.read().game_id.unwrap(),
                        day_killed: tribute.read().day_killed,
                        kills: tribute.read().kills,
                        wins: tribute.read().wins,
                        defeats: tribute.read().defeats,
                        draws: tribute.read().draws,
                        games: tribute.read().games,
                        bravery: tribute.read().bravery,
                        loyalty: tribute.read().loyalty,
                        speed: tribute.read().speed,
                        intelligence: tribute.read().intelligence,
                        persuasion: tribute.read().persuasion,
                        luck: tribute.read().luck,
                        strength: tribute.read().strength,
                        defense: tribute.read().defense,
                        killed_by: tribute.read().killed_by.clone(),
                        is_hidden: tribute.read().is_hidden,
                        dexterity: tribute.read().dexterity,
                        status: tribute.read().status.to_string(),
                        avatar: if new_avatar_path.is_some() { Some(new_avatar_path.unwrap()) } else { tribute.read().avatar.clone() },
                    };
                    Tribute::update(&tribute.read(), update);

                    tribute_name.set(String::from(""));
                    nav.push(Routes::GameDetail { id: tribute.read().game_id.unwrap() });
                },

                div {
                    img {
                        class: "rounded-lg size-64",
                        src: tribute.read().avatar()
                    }
                    input {
                        class: "w-64 cursor-pointer rounded-md border border-orange-700 bg-yellow-200 px-2 py-1 text-gray-900 focus:outline-none mt-2",
                        id: "file_input",
                        r#type: "file",
                        placeholder: "Upload Image",
                        accept: "image/png,image/gif,image/jpg",
                        multiple: false,

                        onchange: upload_files
                    }
                }
            div {
                h1 {
                    class: "text-3xl font-bold text-slate-900 orbitron-font tracking-wider",
                    "{tribute_name}"
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
                }
                h2 {
                    class: "text-xl text-slate-900 orbitron-font font-bold tracking-wider",
                    "District ",
                    span {
                        class: "font-normal text-slate-700 tracking-normal",
                        "{tribute.read().district}"
                    }
                }
            }
        }
            button {
                class: "orbitron-font w-min whitespace-nowrap rounded-md border border-orange-700 bg-gradient-to-r from-orange-500 to-yellow-300 px-2 py-1 text-red-800",
                "Update Tribute"
            }
        }
    }
}
