use dioxus::prelude::*;
use KeyCode::Enter;
use manganis::*;
use hangry_games::games::{Game};
use hangry_games::models::{get_game_by_id, get_games};

pub const LOGO_IMG: ImageAsset = mg!(image("./assets/hangry-games.png"));
pub const CSS: &str = mg!(file("./assets/tailwind.css"));

// All of our routes will be a variant of this Route enum
#[derive(Routable, PartialEq, Clone)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/game")]
    GameDetail {},
}

#[derive(Clone, Copy, Debug)]
struct SelectedGame(Option<i32>);

fn main() {
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
    let games_signal: Signal<Vec<Game>> = use_signal(||list_of_games());
    rsx! {
        p { "Welcome to the Hangry Games!" }
        div {
            {GameList()}
        }
        div {
            {CreateGame()}
        }
    }
}

#[component]
fn GameList() -> Element {
    let mut state = use_context::<Signal<HGState>>();

    rsx! {
        div {
            h2 { "Games" }
            ul {
                for game in state.read().games.iter() {
                    GameListItem { game: game.clone() }
                }
            }
        }
    }
}

#[component]
fn GameListItem(game: Game) -> Element {
    let mut selected_game = use_context::<Signal<SelectedGame>>();
    rsx! {
        li {
            Link {
                onclick: move |_| { selected_game.set(SelectedGame(Some(game.id.unwrap()))) },
                to: Route::GameDetail {},
                "{game.name}, {game.living_tributes().len()}/24 tributes"
            }
        }
    }
}

#[component]
fn GameDetail() -> Element {
    let selected_game = use_context::<Signal<SelectedGame>>();
    let game = Game::from(get_game_by_id(selected_game.read().0.unwrap()).unwrap());
    rsx! {
        div {
            h2 { "Game Detail" }
            h3 { "{game.name} on day {game.day.unwrap_or(0)}" }
            h4 { "Tributes" }
            ul {
                for tribute in game.living_tributes() {
                    li { "{tribute.name}" }
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
    use_context_provider(|| Signal::new(SelectedGame(None)));
    use_context_provider(|| Signal::new(HGState { games: list_of_games() }));

    rsx! {
        head {
            link {
                rel: "stylesheet",
                href: "https://cdn.jsdelivr.net/npm/tailwindcss@2.0.2/dist/tailwind.min.css"
            }
        }
        div {
            Header {}
            Router::<Route> {}
        }
    }
}

