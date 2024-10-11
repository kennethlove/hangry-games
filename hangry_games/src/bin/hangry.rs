use dioxus::html::head;
use dioxus::prelude::*;
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
        img {
            class: "sm:w-4 sm:h-12",
            src: "{LOGO_IMG}", alt: "Hangry Games Logo", width: 52, height: 52 }
        h1 {
            class: "sm:w-12 text-3xl font-bold",
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
    let games = use_signal(||list_of_games());

    rsx! {
        div {
            h2 { "Games" }
            ul {
                for game in games.read().iter() {
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
    rsx! {
        div {
            h2 { "Create Game" }
            form {
                input { r#type: "text", placeholder: "Game Name" }
                button { "Create Game" }
            }
        }
    }
}

fn app() -> Element {
    use_context_provider(|| Signal::new(SelectedGame(None)));
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

