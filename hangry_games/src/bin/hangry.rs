use dioxus::prelude::*;
use dioxus_logger::tracing::Level;
use hangry_games::gui::components::*;
use hangry_games::gui::router::Routes;
use hangry_games::gui::states::HGState;
use hangry_games::gui::functions::list_of_games;

fn main() {
    dioxus_logger::init(Level::INFO).expect("logger failed to init");
    launch(app);
}

fn list_of_games() -> Vec<Game> {
    get_games().iter().map(|g| Game::from(g.clone())).collect()
}

#[component]
fn Header() -> Element {
    rsx! {
        h1 {
            "Hangry Games"
        }
    }
}

#[component]
fn Home() -> Element {
    rsx! {
        p { "Welcome to the Hangry Games!" }
        div {
            class: "grid grid-cols-2",
            GameList {}
        }
        div {
            class: "grid grid-cols-2",
            CreateGame {}
        }
    }
}

#[component]
fn GameList() -> Element {
    let mut state = use_context::<Signal<HGState>>();

    rsx! {
        div {
            h2 { "Games" }
            table {
                class: "min-w-full divide-y-2 divide-gray-200 bg-white text-sm",
                thead {
                    class: "ltr:text-left rtl:text-right",
                    tr {
                        th {
                            class: "whitespace-nowrap px-4 py-2 font-medium text-gray-900",
                            "Name"
                        }
                        th {
                            class: "whitespace-nowrap px-4 py-2 font-medium text-gray-900",
                            "Day"
                        }
                        th {
                            class: "px-4 py-2"
                        }
                    }
                }
                tbody {
                    class: "divide-y divide-gray-200",
                    for game in state.read().games.iter() {
                        GameListItem { game: game.clone() }
                    }
                }
            }
        }
    }
}

#[component]
fn GameListItem(game: Game) -> Element {
    let mut selected_game = use_context::<Signal<SelectedGame>>();
    rsx! {
        tr {
            td {
                class: "whitespace-nowrap px-4 py-2 text-gray-700",
                Link {
                    onclick: move |_| { selected_game.set(SelectedGame(Some(game.id.unwrap()))) },
                    to: Route::GameDetail {},
                    "{game.name}, {game.living_tributes().len()}/24 tributes"
                }
            }
            td {
                "{game.day.unwrap_or(0)}"
            }
            td {
                class: "flex-end",
                GameActionsGroup { game: game.clone() }
            }
        }
    }
}

#[component]
fn GameActionsGroup(game: Game) -> Element {
    rsx! {
        div {
            class: "inline-flex rounded-lg border border-gray-100 bg-gray-100 p-1",
            GameDeleteButton { game: game.clone() }
            GameDetailsButton { game: game.clone() }
            GamePlayButton { game: game.clone() }
        }
    }
}

#[component]
fn GameDeleteButton(game: Game) -> Element {
    let mut state = use_context::<Signal<HGState>>();
    rsx! {
        button {
            class: "inline-block rounded-md px-4 py-2 text-sm text-gray-500 hover:text-red-700 focus:relative",
            onclick: move |_| {
                Game::delete(game.id.unwrap());
                state.write().games.retain(|g| g.id != game.id);
            },
            "Delete"
        }
    }
}

#[component]
fn GameDetailsButton(game: Game) -> Element {
    let mut state = use_context::<Signal<HGState>>();
    let nav = navigator();

    rsx! {
        button {
            class: "inline-block rounded-md px-4 py-2 text-sm text-gray-500 hover:text-blue-700 focus:relative",
            onclick: move |_| {
                let mut selected_game = use_context::<Signal<SelectedGame>>();
                selected_game.set(SelectedGame(Some(game.id.unwrap())));
                nav.push(Route::GameDetail {});
            },
            "Details"
        }
    }
}

#[component]
fn GamePlayButton(game: Game) -> Element {
    let mut state = use_context::<Signal<HGState>>();
    rsx! {
        button {
            class: "inline-block rounded-md px-4 py-2 text-sm text-gray-500 hover:text-green-700 focus:relative",
            onclick: move |_| {
            },
            "Play"
        }
    }
}

#[component]
fn GameDetail() -> Element {
    let selected_game = use_context::<Signal<SelectedGame>>();
    let game = Game::from(get_game_by_id(selected_game.read().0.unwrap()).unwrap());
    rsx! {
        div {
            class: "flow-root",
            dl {
                class: "-my-3 divide-y divide-gray-100 text-sm",
                div {
                    class: "grid grid-cols-1 gap-1 py-3 sm:grid-cols-3 sm:gap-4",
                    dt {
                        class: "font-medium text-gray-900",
                        "Name"
                    }
                    dd {
                        class: "text-gray-700 sm:col-span-2",
                        "{game.name}"
                    }
                }
                div {
                    class: "grid grid-cols-1 gap-1 py-3 sm:grid-cols-3 sm:gap-4",
                    dt {
                        class: "font-medium text-gray-900",
                        "Day"
                    }
                    dd {
                        class: "text-gray-700 sm:col-span-2",
                        "{game.day.unwrap_or(0)}"
                    }
                }
                div {
                    class: "grid grid-cols-1 gap-1 py-3 sm:grid-cols-3 sm:gap-4",
                    dt {
                        class: "font-medium text-gray-900",
                        "Living Tributes"
                    }
                    dd {
                        class: "text-gray-700 sm:col-span-2",
                        ul {
                            class: "divide-y divide-gray-200",
                            for tribute in game.living_tributes() {
                                li {
                                    class: "flex items-center py-3",
                                    "{tribute.name}"
                                }
                            }
                        }
                    }
                }
            }
        }

        Link {
            to: Route::Home {},
            "Home"
        }
    }
}

#[component]
fn CreateGame() -> Element {
    let mut state = use_context::<Signal<HGState>>();
    let mut game_name = use_signal(String::new);
    let nav = navigator();

    rsx! {
        div {
            h2 { "Create Game" }
            form {
                onsubmit: move |event| {
                    let data = event.data.values();
                    let game_name = data.get("game_name").unwrap().first().unwrap();
                    let game = Game::new(game_name);
                    let mut selected_game = use_context::<Signal<SelectedGame>>();
                    selected_game.set(SelectedGame(Some(game.id.unwrap())));
                    state.write().games.push(game);
                    nav.push(Route::GameDetail {});
                },
                input {
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
                button { "Create Game" }
            }
        }
    }
}

#[derive(Debug)]
struct HGState {
    games: Vec<Game>,
}

fn app() -> Element {
    use_context_provider(|| Signal::new(HGState { games: list_of_games() }));

    rsx! {
        div {
            class: "container mx-auto",
            header::Header {}
            Router::<Routes> {}
            img { src: mg!(image("assets/hangry-games.png").preload()) }
        }
    }
}
