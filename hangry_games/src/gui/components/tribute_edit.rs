use std::path::Path;
use std::rc::Rc;
use std::str::FromStr;
use std::sync::Arc;
use dioxus::html::FileEngine;
use dioxus::prelude::*;
use crate::gui::components::UploadedFile;
use crate::gui::router::Routes;
use crate::models::{get_tribute_by_id, UpdateTribute};
use crate::tributes::actors::Tribute;
use crate::gui::components::input_with_label::InputWithLabel;
use crate::tributes::statuses::TributeStatus;
use crate::animals::Animal;
use strum::IntoEnumIterator;

#[component]
pub fn TributeEdit(id: i32) -> Element {
    let nav = navigator();
    let mut tribute = use_signal(|| Tribute::from(get_tribute_by_id(id)));
    let mut tribute_name = use_signal(|| tribute.read().name.clone());
    let mut files_uploaded = use_signal(|| Vec::new() as Vec<crate::gui::components::UploadedFile>);
    let tribute_status = tribute.read().status.clone();
    dbg!(tribute_status);

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
            form {
                class: "grid grid-cols-2 justify-items-center gap-x-4 gap-y-2",
                enctype: "multipart/form-data",
                onsubmit: move |event| {
                    let data = event.data.values();
                    let name = data.get("tribute-name").unwrap().first().unwrap();
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
                    class: "justify-self-end pr-16",
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
                    class: "flex flex-row flex-wrap gap-2 h-min",
                    div {
                        class: "flex flex-row flex-nowrap gap-2 w-full mb-2",
                        InputWithLabel {
                            label: "Name".to_string(),
                            value: tribute.read().name.clone(),
                            name: "tribute-name".to_string(),
                            placeholder: "Name".to_string(),
                            oninput: move |evt: Rc<FormData>| tribute.write().name = evt.value().clone(),
                            extra_css_classes: Some("w-72".to_string())
                        }
                        InputWithLabel {
                            label: "District".to_string(),
                            value: tribute.read().district.clone(),
                            name: "tribute-district".to_string(),
                            placeholder: "District".to_string(),
                            oninput: move |evt: Rc<FormData>| tribute.write().district = evt.value().parse::<i32>().unwrap()
                        }
                    }
                    div {
                        class: "grid grid-row gap-2 grid-cols-4 w-full",
                        InputWithLabel {
                            label: "Health".to_string(),
                            value: tribute.read().health.clone(),
                            name: "tribute-health".to_string(),
                            placeholder: "Health".to_string(),
                            oninput: move |evt: Rc<FormData>| tribute.write().health = evt.value().parse::<i32>().unwrap()
                        }
                        InputWithLabel {
                            label: "Sanity".to_string(),
                            value: tribute.read().sanity.clone(),
                            name: "tribute-sanity".to_string(),
                            placeholder: "Sanity".to_string(),
                            oninput: move |evt: Rc<FormData>| tribute.write().sanity = evt.value().parse::<i32>().unwrap()
                        }
                        InputWithLabel {
                            label: "Movement".to_string(),
                            value: tribute.read().movement.clone(),
                            name: "tribute-movement".to_string(),
                            placeholder: "Movement".to_string(),
                            oninput: move |evt: Rc<FormData>| tribute.write().movement = evt.value().parse::<i32>().unwrap()
                        }
                        InputWithLabel {
                            label: "Bravery".to_string(),
                            value: tribute.read().bravery.clone().unwrap_or(0).to_string(),
                            name: "tribute-bravery".to_string(),
                            placeholder: "Bravery".to_string(),
                            oninput: move |evt: Rc<FormData>| tribute.write().bravery = Some(evt.value().parse::<i32>().unwrap())
                        }
                        InputWithLabel {
                            label: "Loyalty".to_string(),
                            value: tribute.read().loyalty.clone().unwrap_or(0).to_string(),
                            name: "tribute-loyalty".to_string(),
                            placeholder: "Loyalty".to_string(),
                            oninput: move |evt: Rc<FormData>| tribute.write().loyalty = Some(evt.value().parse::<i32>().unwrap())
                        }
                        InputWithLabel {
                            label: "Speed".to_string(),
                            value: tribute.read().speed.clone().unwrap_or(0).to_string(),
                            name: "tribute-speed".to_string(),
                            placeholder: "Speed".to_string(),
                            oninput: move |evt: Rc<FormData>| tribute.write().speed = Some(evt.value().parse::<i32>().unwrap())
                        }
                        InputWithLabel {
                            label: "Intelligence".to_string(),
                            value: tribute.read().intelligence.clone().unwrap_or(0).to_string(),
                            name: "tribute-intelligence".to_string(),
                            placeholder: "Intelligence".to_string(),
                            oninput: move |evt: Rc<FormData>| tribute.write().intelligence = Some(evt.value().parse::<i32>().unwrap())
                        }
                        InputWithLabel {
                            label: "Persuasion".to_string(),
                            value: tribute.read().persuasion.clone().unwrap_or(0).to_string(),
                            name: "tribute-persuasion".to_string(),
                            placeholder: "Persuasion".to_string(),
                            oninput: move |evt: Rc<FormData>| tribute.write().persuasion = Some(evt.value().parse::<i32>().unwrap())
                        }
                        InputWithLabel {
                            label: "Luck".to_string(),
                            value: tribute.read().luck.clone().unwrap_or(0).to_string(),
                            name: "tribute-luck".to_string(),
                            placeholder: "Luck".to_string(),
                            oninput: move |evt: Rc<FormData>| tribute.write().luck = Some(evt.value().parse::<i32>().unwrap())
                        }
                        InputWithLabel {
                            label: "strength".to_string(),
                            value: tribute.read().strength.clone().unwrap_or(0).to_string(),
                            name: "tribute-strength".to_string(),
                            placeholder: "strength".to_string(),
                            oninput: move |evt: Rc<FormData>| tribute.write().strength = Some(evt.value().parse::<i32>().unwrap())
                        }
                        InputWithLabel {
                            label: "defense".to_string(),
                            value: tribute.read().defense.clone().unwrap_or(0).to_string(),
                            name: "tribute-defense".to_string(),
                            placeholder: "defense".to_string(),
                            oninput: move |evt: Rc<FormData>| tribute.write().defense = Some(evt.value().parse::<i32>().unwrap())
                        }
                        InputWithLabel {
                            label: "dexterity".to_string(),
                            value: tribute.read().dexterity.clone().unwrap_or(0).to_string(),
                            name: "tribute-dexterity".to_string(),
                            placeholder: "dexterity".to_string(),
                            oninput: move |evt: Rc<FormData>| tribute.write().dexterity = Some(evt.value().parse::<i32>().unwrap())
                        }
                    }
                }
                button {
                    class: "mr-32 orbitron-font w-min whitespace-nowrap rounded-md border border-orange-700 bg-gradient-to-r from-orange-500 to-yellow-300 px-2 py-1 text-red-800 flex-grow",
                    r#type: "submit",
                    "Update Tribute"
                }
            }
        }
    }
}
