use crate::games::Game;

#[derive(Clone, Copy, Debug)]
pub struct SelectedGame(pub Option<i32>);

#[derive(Debug)]
pub struct HGState {
    pub games: Vec<Game>,
}
