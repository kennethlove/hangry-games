use dioxus::prelude::*;
use crate::gui::components::{
    home::Home,
    game_detail::GameDetail,
};

// All of our routes will be a variant of this Route enum
#[derive(Routable, PartialEq, Clone)]
pub enum Routes {
    #[route("/")]
    Home {},
    #[route("/game")]
    GameDetail {},
}
