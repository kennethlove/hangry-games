use crate::areas::Area;
use crate::events::TributeEvent;
use crate::items::Item;
use crate::models::game::{get_game, Game as GameModel};
use crate::models::{create_item, get_all_living_tributes, get_recently_dead_tributes, update_tribute, NewItem};
use crate::tributes::actions::TributeAction;
use crate::tributes::actors::Tribute;
use crate::tributes::statuses::TributeStatus;
use rand::prelude::SliceRandom;
use rand::Rng;
use std::fmt::Display;
use std::str::FromStr;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Game {
    pub id: Option<i32>,
    pub name: String,
    pub day: Option<i32>,
    pub closed_areas: Option<Vec<Area>>,
    pub status: GameStatus,
}

impl Game {
    pub fn as_str(&self) -> &str {
        self.name.as_str()
    }

    // Runs at the start of the game
    pub fn start(&self) {
        let knife = Item::new_random("knife".to_string());
        let game = get_game(self.name.as_str()).expect("Error loading game");
        let the_cornucopia = Area::from_str("cornucopia").expect("Error loading area");

        for _ in 0..24 {
            let mut new_knife = NewItem::from(knife.clone());
            new_knife.area_id = Some(the_cornucopia.id());
            new_knife.game_id = Some(game.id);
            create_item(new_knife);
        }
    }

    pub fn living_tributes(&self) -> Vec<Tribute> {
        let game = get_game(self.name.as_str()).expect("Error loading game");
        get_all_living_tributes(&game).iter().map(|t| Tribute::from(t.clone())).collect()
    }

    pub fn run_day_night_cycle(&mut self) {
        let game = get_game(self.name.as_str()).expect("Error loading game");
        self.day = Some(self.day.unwrap_or(0) + 1);
        game.set_day(self.day.unwrap());

        // Get all the living tributes
        let living_tributes = get_all_living_tributes(&game);

        // See if we have a winner or a dud game
        match living_tributes.len() {
            0 => {
                println!("=== 🎭 No one wins! ===");
                game.end();
                return;
            }
            1 => {
                println!("=== 🏆 The winner is {} ===", living_tributes[0].name);
                game.end();
                return;
            }
            _ => {}
        }

        // Make any announcements for the day
        match self.day {
            Some(1) => {
                println!("=== 🎉 The Hunger Games begin! 🎉 ===");
            }
            Some(3) => {
                println!("=== 😋 Day 3: Feast Day ===");
            }
            _ => {
                println!("=== ☀️ Day {} begins ===", self.day.unwrap());
            }
        }

        println!("=== {} tribute{} remain{} ===",
             living_tributes.len(),
             if living_tributes.len() == 1 { "" } else { "s" },
             if living_tributes.len() == 1 { "s" } else { "" }
        );

        // Run the day
        self.do_day_night_cycle(true);

        // Clean up any deaths
        self.clean_up_recent_deaths();

        // Run the night
        println!("=== 🌙 Night {} begins ===", self.day.unwrap());
        self.do_day_night_cycle(false);

        // Clean up any deaths
        self.clean_up_recent_deaths();

    }

    pub fn do_day_night_cycle(&mut self, day: bool) {
        let mut rng = rand::thread_rng();
        let day_event_frequency = 1.0 / 4.0;
        let night_event_frequency = 1.0 / 8.0;
        let game = get_game(self.name.as_str()).expect("Error loading game");

        // Clean up any deaths from the previous cycle's events
        Area::clean_up_area_events(self.id.unwrap());

        // Trigger any events for this cycle
        if self.day > Some(3) || !day {
            if rng.gen_bool(if day { day_event_frequency } else { night_event_frequency }) {
                Area::do_area_event(self.id.unwrap());
            }
        }

        // Get all the remaining tributes to run their appropriate actions
        let mut living_tributes = get_all_living_tributes(&game);

        // If there are too few tributes, close an area or two
        if living_tributes.len() > 1 && living_tributes.len() < 7 {
            Area::do_area_event(self.id.unwrap());
            if rng.gen_bool(living_tributes.len() as f64 / 12.0) {
                Area::do_area_event(self.id.unwrap());
            }
        }

        living_tributes.shuffle(&mut rng);
        for tribute in living_tributes {
            let mut tribute = Tribute::from(tribute.clone());

            // Use luck to decide if the tribute is caught by an event
            if !rng.gen_bool(tribute.luck.unwrap_or(0) as f64 / 100.0) {
                let event = TributeEvent::random();
                tribute.handle_event(event);
            }

            // If the event killed the tribute, move on
            if !tribute.is_alive() {
                dbg!("tribute event killed tribute");
                tribute.status = TributeStatus::RecentlyDead;
                continue
            };

            match (self.day, day) {
                (Some(1), true) => {
                    tribute.do_day_night(Some(TributeAction::Move(None)), Some(0.5), day);
                }
                (Some(3), true) => {
                    tribute.do_day_night(
                        Some(TributeAction::Move(Some(Area::Cornucopia.to_string()))),
                        Some(0.75),
                        day
                    );
                }
                (_, _) => {
                    tribute.do_day_night(None, None, day);
                }
            };
            update_tribute(tribute.id.unwrap(), tribute.into());
        }

    }
    pub fn clean_up_recent_deaths(&self) {
        let game = get_game(self.name.as_str()).expect("Error loading game");
        let dead_tributes = get_recently_dead_tributes(&game);

        println!("=== 💀 {} tribute{} died ===", dead_tributes.len(), if dead_tributes.len() == 1 { "" } else { "s" });

        for tribute in dead_tributes {
            tribute.dies();
            println!("🪦 {}", tribute.name);
        }
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl FromStr for Game {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let game = get_game(s).expect("Error loading game");
        Ok(Game::from(game))
    }
}

impl From<GameModel> for Game {
    fn from(game: GameModel) -> Self {
        let status = match game.ended_at {
            Some(_) => GameStatus::Finished,
            None => match game.day {
                Some(0) => GameStatus::NotStarted,
                _ => GameStatus::InProgress,
            },
        };

        Game {
            id: Some(game.id),
            name: game.name.clone(),
            day: Some(game.day.unwrap_or(0)),
            closed_areas: Some(game.closed_areas()),
            status,
        }
    }
}

impl From<String> for Game {
    fn from(s: String) -> Self {
        Self::from_str(s.as_str()).expect("Couldn't match that game")
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum GameStatus {
    NotStarted,
    InProgress,
    Finished,
}

impl Display for GameStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GameStatus::NotStarted => write!(f, "Not Started"),
            GameStatus::InProgress => write!(f, "In Progress"),
            GameStatus::Finished => write!(f, "Finished"),
        }
    }
}

impl FromStr for GameStatus {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "not started" => Ok(GameStatus::NotStarted),
            "in progress" => Ok(GameStatus::InProgress),
            "finished" => Ok(GameStatus::Finished),
            _ => Err(()),
        }
    }
}
