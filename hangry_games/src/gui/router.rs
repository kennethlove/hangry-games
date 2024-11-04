use dioxus::prelude::*;
use crate::gui::components::{
    home::Home,
    game_detail::GameDetail,
    game_day_log::GameDayLog,
    game_log::GameLog,
    game_play::GamePlay,
    tribute_detail::TributeDetail,
    tribute_edit::TributeEdit,
};

// All of our routes will be a variant of this Route enum
#[derive(Routable, PartialEq, Clone)]
pub enum Routes {
    #[route("/")]
    Home {},
    #[route("/game/:id")]
    GameDetail { id: i32 },
    #[route("/game/:id/play")]
    GamePlay { id: i32 },
    #[route("/game/:id/log")]
    GameLog { id: i32 },
    #[route("/game/:id/log/day/:day")]
    GameDayLog { id: i32, day: i32 },
    #[route("/tribute/:id")]
    TributeDetail { id: i32 },
    #[route("/tribute/:id/edit")]
    TributeEdit { id: i32 },
}
