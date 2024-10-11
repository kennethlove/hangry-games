use dioxus::prelude::*;
use hangry_games::animals::Animal;
use manganis::*;
use hangry_games::games::{Game, GameStatus};
use hangry_games::models::{get_game, get_game_by_id, get_games};

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

fn new_animal() -> String {
    Animal::random().to_string()
}

fn list_of_games() -> Vec<Game> {
    get_games().iter().map(|g| Game::from(g.clone())).collect()
}

#[component]
fn Header() -> Element {
    rsx! {
        h1 {
            img { src: "{LOGO_IMG}", alt: "Hangry Games Logo", width: 52, height: 52 }
            "Hangry Games"
        }
    }
}

#[component]
fn Home() -> Element {
    rsx! {
        p { "Welcome to the Hangry Games!" }
        Games {}
    }
}

#[component]
fn AnimalName() -> Element {
    let mut animal = use_signal(||new_animal());

    rsx! {
        div {
            h2 { {animal} }
            button {
                onclick: move |_| { animal.set(new_animal()) },
                "Random"
            }
        }
    }
}

#[component]
fn Games() -> Element {
    let games = use_signal(||list_of_games());

    rsx! {
        div {
            h2 { "Games" }
            ul {
                for game in games.read().iter() {
                    GameItem { game: game.clone() }
                }
            }
        }
    }
}

#[component]
fn GameItem(game: Game) -> Element {
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
fn GameDetails() -> Element {
    let mut selected_game = use_context::<Signal<SelectedGame>>();
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

fn app() -> Element {
    use_context_provider(|| Signal::new(SelectedGame(None)));
    rsx! {
        div {
            Header {}
            Router::<Route> {}
        }
    }
}
pub const LOGO_IMG: ImageAsset = mg!(image("assets/hangry-games.png"));

