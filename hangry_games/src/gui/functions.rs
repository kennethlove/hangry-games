use crate::games::Game;
use crate::models::get_games;

pub fn list_of_games() -> Vec<Game> {
    get_games(Some(10)).iter().map(|g| Game::from(g.clone())).collect()
}
