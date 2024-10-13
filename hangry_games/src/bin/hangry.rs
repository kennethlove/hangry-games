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
        h1 { "Hangry Games" }
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
