use super::actions::{AttackOutcome, AttackResult, TributeAction};
use super::brains::TributeBrain;
use super::statuses::TributeStatus;
use crate::areas::Area;
use crate::events::TributeEvent;
use crate::models;
use crate::models::tribute::UpdateTribute;
use rand::prelude::*;
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq)]
pub struct Tribute {
    pub id: Option<i32>,
    pub game_id: Option<i32>,
    pub name: String,
    pub health: i32,
    pub sanity: i32,
    pub movement: i32,
    pub district: i32,
    pub brain: TributeBrain,
    pub area: Option<Area>,
    pub day_killed: Option<i32>,
    pub killed_by: Option<String>,
    pub kills: Option<i32>,
    pub wins: Option<i32>,
    pub defeats: Option<i32>,
    pub draws: Option<i32>,
    pub games: Option<i32>,
    pub bravery: Option<i32>,
    pub loyalty: Option<i32>,
    pub speed: Option<i32>,
    pub intelligence: Option<i32>,
    pub persuasion: Option<i32>,
    pub luck: Option<i32>,
    pub strength: Option<i32>,
    pub defense: Option<i32>,
    pub is_hidden: Option<bool>,
    pub dexterity: Option<i32>,
    pub status: TributeStatus
}


impl Tribute {
    /// Creates a new Tribute with full health, sanity, and movement.
    pub fn new(name: String, district: Option<i32>) -> Self {
        let brain = TributeBrain::new();
        let mut rng = thread_rng();
        let district = district.unwrap_or(0);
        Self {
            id: None,
            game_id: None,
            name: name.clone(),
            health: 100,
            sanity: 100,
            movement: 100,
            district,
            area: Some(Area::default()),
            brain,
            day_killed: None,
            killed_by: None,
            kills: Some(0),
            wins: Some(0),
            defeats: Some(0),
            draws: Some(0),
            games: Some(0),
            bravery: Some(rng.gen_range(1..=100)),
            loyalty: Some(rng.gen_range(1..=100)),
            speed: Some(rng.gen_range(1..=100)),
            intelligence: Some(rng.gen_range(1..=100)),
            persuasion: Some(rng.gen_range(1..=100)),
            luck: Some(rng.gen_range(1..=100)),
            strength: Some(rng.gen_range(1..=50)),
            defense: Some(rng.gen_range(1..=50)),
            is_hidden: Some(false),
            dexterity: Some(rng.gen_range(1..=100)),
            status: TributeStatus::Healthy,
        }
    }

    /// Reduces health, triggers death if health reaches 0.
    pub fn takes_physical_damage(&mut self, damage: i32) {
        self.health = std::cmp::max(0, self.health - damage);
    }

    /// Reduces mental health.
    pub fn takes_mental_damage(&mut self, damage: i32) {
        self.sanity = std::cmp::max(0, self.sanity - damage);
    }

    /// Restores health.
    pub fn heals(&mut self, health: i32) {
        self.health = std::cmp::min(100, self.health + health);
    }

    /// Restores mental health.
    pub fn heals_mental_damage(&mut self, health: i32) {
        self.sanity = std::cmp::min(100, self.sanity + health);
    }

    /// Consumes movement and removes hidden status.
    pub fn moves(&mut self) {
        self.movement = std::cmp::max(0, self.movement - self.speed.unwrap());
        self.is_hidden = Some(false);
    }

    /// Restores movement.
    pub fn short_rests(&mut self) {
        println!("💤 {} rests", self.name);
        self.movement = 100;
    }

    pub fn long_rests(&mut self) {
        println!("💤 {} rests and recovers a little health and sanity", self.name);
        self.short_rests();
        self.heals(5);
        self.heals_mental_damage(5);
    }

    /// Marks the tribute as recently dead and reveals them.
    pub fn dies(&mut self) {
        self.status = TributeStatus::RecentlyDead;
        self.is_hidden = Some(false);
    }

    /// Moves the tribute from one area to another, removes hidden status.
    pub fn changes_area(&mut self, area: Area) {
        self.area = Some(area);
        self.is_hidden = Some(false);
    }

    /// Removes the tribute from the game arena, removes hidden status.
    pub fn leaves_area(&mut self) {
        self.area = None;
        self.is_hidden = Some(false);
    }

    /// Hides the tribute from view.
    pub fn hides(&mut self) {
        self.is_hidden = Some(true);
        println!("🫥 {} tries to hide", self.name);
    }

    /// Reveals the tribute to view.
    pub fn reveals(&mut self) {
        self.is_hidden = Some(false);
    }

    /// Tribute is lonely/homesick/etc., loses some sanity.
    pub fn suffers(&mut self) {
        let terror = (self.sanity as f64 / 100.0) * 5.0;
        self.takes_mental_damage(terror.round() as i32);
        println!("😭 {} suffers from loneliness and terror.", self.name);
    }

    pub fn attacks(&mut self, target: &mut Tribute) -> AttackOutcome {
        let game = get_game_by_id(self.game_id.unwrap()).unwrap();
        match attack_contest(self.clone(), target.clone()) {
            AttackResult::AttackerWins => {
                if self == target {
                    println!("🤦 {} harms themself!", self.name);
                } else {
                    println!("🔪 {} attacks {}, and wins!", self.name, target.name);
                }
                target.takes_physical_damage(self.strength.unwrap());

                if target.health <= 0 {
                    target.status = TributeStatus::RecentlyDead;
                }

                self.wins = Some(self.wins.unwrap_or(0) + 1);
                apply_violence_stress(self);

                if target.status == TributeStatus::RecentlyDead {
                    self.kills = Some(self.kills.unwrap_or(0) + 1);
                    target.killed_by = Some(self.name.clone());
                    target.day_killed = Some(game.day.unwrap());
                    target.defeats = Some(target.defeats.unwrap_or(0) + 1);
                    println!("☠️ {} kills {}", self.name, target.name);
                    return AttackOutcome::Kill(self.clone(), target.clone());
                }
                target.status = TributeStatus::Wounded;
                println!("🤕 {} wounds {}", self.name, target.name);
                AttackOutcome::Wound(self.clone(), target.clone())
            }
            AttackResult::DefenderWins => {
                if target == self {
                    println!("🤦 {} harms themself!", self.name);
                } else {
                    println!("🤣 {} attacks {}, but loses!", self.name, target.name);
                }
                self.takes_physical_damage(target.strength.unwrap());

                if self.health <= 0 {
                    self.status = TributeStatus::RecentlyDead;
                }
                target.wins = Some(target.wins.unwrap() + 1);
                apply_violence_stress(target);

                if self.status == TributeStatus::RecentlyDead {
                    target.kills = Some(target.kills.unwrap() + 1);
                    self.killed_by = Some(target.name.clone());
                    self.day_killed = Some(game.day.unwrap());
                    self.defeats = Some(self.defeats.unwrap() + 1);
                    println!("☠️ {} kills {}", target.name, self.name);
                    return AttackOutcome::Kill(target.clone(), self.clone());
                }
                self.status = TributeStatus::Wounded;
                println!("🤕 {} wounds {}", target.name, self.name);
                AttackOutcome::Wound(target.clone(), self.clone())
            }
            AttackResult::Miss => {
                println!("👻 {} attacks {}, but misses!", self.name, target.name);
                self.draws = Some(self.draws.unwrap() + 1);
                target.draws = Some(target.draws.unwrap() + 1);

                AttackOutcome::Miss(self.clone(), target.clone())
            }
        }
    }

    pub fn is_visible(&self) -> bool {
        let is_hidden = self.is_hidden.unwrap_or(false);
        if is_hidden {
            let mut rng = thread_rng();
            !rng.gen_bool(self.intelligence.unwrap() as f64 / 100.0)
        } else {
            true
        }
    }

    pub fn travels(&self, closed_areas: Vec<Area>, suggested_area: Option<String>) -> TravelResult {
        let mut rng = thread_rng();
        let area = self.clone().area.unwrap();
        let failure_msg = format!("😴 {} is too tired to move from {}, rests instead", self.name, area);
        let success_msg = "🚶{tribute} moves from {area} to {new_area}";

        let suggested_area = {
            let suggested_area = suggested_area.clone();
            if suggested_area.is_some() {
                let suggested_area = Area::from_str(suggested_area.unwrap().as_str()).unwrap();
                if closed_areas.contains(&suggested_area) {
                    None
                } else {
                    Some(suggested_area)
                }
            } else {
                None
            }
        };

        if suggested_area.is_some() && suggested_area.clone().unwrap() == area {
            println!("🤔 {} is already in the suggested area, stays put", self.name);
            return TravelResult::Failure;
        }

        let handle_suggested_area = || -> TravelResult {
            if suggested_area.is_some() {
                println!("{}", success_msg
                    .replace("{tribute}", self.name.as_str())
                    .replace("{area}", area.as_str())
                    .replace("{new_area}", suggested_area.clone().unwrap().as_str())
                );
                return TravelResult::Success(suggested_area.unwrap());
            }
            TravelResult::Failure
        };

        match self.movement {
            // No movement left, can't move
            0 => {
                println!("{}", failure_msg);
                TravelResult::Failure
            },
            // Low movement, can only move to suggested area
            1..=10 => {
                match handle_suggested_area() {
                    TravelResult::Success(area) => TravelResult::Success(area),
                    TravelResult::Failure => {
                        println!("{}", failure_msg);
                        TravelResult::Failure
                    }
                }
            },
            // High movement, can move to any open neighbor or the suggested area
            _ => {
                match handle_suggested_area() {
                    TravelResult::Success(area) => return TravelResult::Success(area),
                    TravelResult::Failure => ()
                }
                let neighbors = area.neighbors();
                let new_area = loop {
                    let new_area = neighbors.choose(&mut rng).unwrap();
                    if new_area == &area || closed_areas.contains(new_area) {
                        continue;
                    }
                    break new_area.clone();
                };
                println!("{}", success_msg
                    .replace("{tribute}", self.name.as_str())
                    .replace("{area}", area.as_str())
                    .replace("{new_area}", new_area.as_str())
                );
                TravelResult::Success(new_area)
            }
        }
    }

    pub fn process_status(&mut self) {
        let status = self.status.clone();
        match status {
            TributeStatus::Wounded => {
                self.takes_physical_damage(1);
                println!("🩸 {} bleeds from their wounds.", self.name);
            },
            TributeStatus::Sick => {
                self.strength = Some(std::cmp::max(1, self.strength.unwrap() - 1));
                self.speed = Some(std::cmp::max(1, self.speed.unwrap() - 1));
                println!("🤒 {} contracts dysentery, loses strength and speed", self.name);
            },
            TributeStatus::Electrocuted => {
                self.takes_physical_damage(20);
                println!("🌩️ {} is struck by lightning, loses health", self.name);
            },
            TributeStatus::Frozen => {
                self.speed = Some(std::cmp::max(1, self.speed.unwrap() - 1));
                println!("🥶 {} suffers from hypothermia, loses speed.", self.name);
            },
            TributeStatus::Overheated => {
                self.speed = Some(std::cmp::max(1, self.speed.unwrap() - 1));
                println!("🥵 {} suffers from heat stroke, loses speed.", self.name);
            },
            TributeStatus::Dehydrated => {
                self.strength = Some(std::cmp::max(1, self.strength.unwrap() - 1));
                println!("🌵 {} is severely dehydrated, loses strength", self.name);
            },
            TributeStatus::Starving => {
                self.strength = Some(std::cmp::max(1, self.strength.unwrap() - 1));
                println!("🍴 {} is ravenously hungry, loses strength", self.name);
            },
            TributeStatus::Poisoned => {
                self.takes_mental_damage(5);
                println!("🧪 {} eats something poisonous, loses sanity", self.name);
            },
            TributeStatus::Broken => {
                // coin flip for which bone breaks
                let leg_bone = thread_rng().gen_bool(0.5);

                // TODO: Add in other bones? Ribs and skull make sense.

                if leg_bone {
                    self.speed = Some(std::cmp::max(1, self.speed.unwrap() - 5));
                    println!("🦴 {} injures their leg, loses speed.", self.name);
                } else {
                    self.strength = Some(std::cmp::max(1, self.strength.unwrap() - 5));
                    println!("🦴 {} injures their arm, loses strength.", self.name);
                }
            },
            TributeStatus::Infected => {
                self.takes_physical_damage(2);
                self.takes_mental_damage(2);
                println!("🤢 {} gets an infection, loses health and sanity", self.name);
            },
            TributeStatus::Drowned => {
                self.takes_physical_damage(2);
                self.takes_mental_damage(2);
                println!("🏊 {} partially drowns, loses health and sanity", self.name);
            },
            TributeStatus::Mauled(animal) => {
                let number_of_animals = thread_rng().gen_range(2..=5);
                let damage = animal.damage() * number_of_animals;
                self.takes_physical_damage(damage);
                println!("🐾 {} is attacked by {} {}, takes {} damage!", self.name, number_of_animals, animal.plural(), damage);
            },
            TributeStatus::Burned => {
                self.takes_physical_damage(5);
                println!("🔥 {} gets burned, loses health", self.name);
            }
            _ => {}
        }

        if self.health <= 0 {
            println!("💀 {} dies from {}", self.name, self.status);
            self.killed_by = Some(self.status.to_string());
            self.status = TributeStatus::RecentlyDead;
        }
    }

    pub fn handle_event(&mut self, player_event: TributeEvent) {
        match player_event {
            TributeEvent::AnimalAttack(ref animal) => {
                self.status = TributeStatus::Mauled(animal.clone());
            },
            TributeEvent::Dysentery => {
                self.status = TributeStatus::Sick;
            }
            TributeEvent::LightningStrike => {
                self.status = TributeStatus::Electrocuted;
            }
            TributeEvent::Hypothermia => {
                self.status = TributeStatus::Frozen;
            }
            TributeEvent::HeatStroke => {
                self.status = TributeStatus::Overheated;
            },
            TributeEvent::Dehydration => {
                self.status = TributeStatus::Dehydrated;
            },
            TributeEvent::Starvation => {
                self.status = TributeStatus::Starving;
            },
            TributeEvent::Poisoning => {
                self.status = TributeStatus::Poisoned;
            },
            TributeEvent::BrokenBone => {
                self.status = TributeStatus::Broken;
            },
            TributeEvent::Infection => {
                self.status = TributeStatus::Infected;
            },
            TributeEvent::Drowning => {
                self.status = TributeStatus::Drowned;
            },
            TributeEvent::Burn => {
                self.status = TributeStatus::Burned;
            },
        }
        if self.health <= 0 {
            println!("💀 {} dies by {}", self.name, player_event.clone());
            self.killed_by = Some(self.status.to_string());
            self.status = TributeStatus::RecentlyDead;
        }
    }

    pub fn do_day_night(&mut self, suggested_action: Option<TributeAction>, probability: Option<f64>, day: bool) -> Tribute {
        // Update the tribute based on the period's events.
        self.process_status();

        // Nighttime terror
        if !day { self.suffers(); }

        // Tribute died to the period's events.
        if self.status == TributeStatus::RecentlyDead {
            self.status = TributeStatus::Dead;
            println!("😱 {} is dead!", self.name);
            TributeModel::from(self.clone()).dies();
            return self.clone();
        }

        let game = get_game_by_id(self.game_id.unwrap()).unwrap();
        let area = self.area.clone().unwrap();
        let closed_areas = game.closed_areas().clone();

        // Area is closed, tribute must move.
        if closed_areas.contains(&area) {
            self.travels(closed_areas.clone(), None);
            update_tribute(self.id.unwrap(), self.clone().into());
            return self.clone();
        }

        let brain = &mut self.brain.clone();

        if suggested_action.is_some() {
            brain.set_preferred_action(suggested_action.unwrap(), probability.unwrap());
        }

        let nearby_tributes = game.clone().living_tributes().iter()
            .filter(|t| t.area().is_some())
            .map(|t| Tribute::from(t.clone()))
            .filter(|t| t.clone().area.unwrap() == area)
            .collect::<Vec<_>>();

        let action = brain.act(self, nearby_tributes.len(), closed_areas.clone());

        match &action {
            TributeAction::Move(area) => {
                match self.travels(closed_areas.clone(), area.clone()) {
                    TravelResult::Success(area) => {
                        self.changes_area(area.clone());
                        self.take_action(action.clone(), Some(area.clone().to_string()));
                    },
                    TravelResult::Failure => {
                        self.short_rests();
                        self.take_action(action.clone(), None);
                    }
                }
            },
            TributeAction::Hide => {
                self.hides();
                self.take_action(action, None);
            },
            TributeAction::Rest => {
                self.short_rests();
                self.take_action(action, None);
            },
            TributeAction::Attack => {
                let target = pick_target(self.clone().into(), nearby_tributes.clone());
                if let Some(mut target) = target {
                    if target.is_visible() {
                        self.attacks(&mut target);
                        self.take_action(action, Some(target.clone().name));
                    } else {
                        println!("🤔 {} can't attack {}, they're hidden", self.name, target.name);
                        self.take_action(TributeAction::Hide, None);
                    }
                }
            },
            TributeAction::None => {
                self.short_rests();
                self.take_action(action, None);
            }
            _ => {
                println!("⛔ {} does nothing", self.name);
                self.take_action(action, None);
            }
        }

        update_tribute(self.id.unwrap(), TributeModel::from(self.clone()));
        self.clone()
    }

    fn take_action(&self, action: TributeAction, target: Option<String>) {
        use models::tribute_action::take_action;
        use models::action::get_action;

        let tribute = TributeModel::from(self.clone());
        let action = Action::from(get_action(action.as_str()));
        take_action(&tribute, &action, target);
    }
}

#[derive(Debug)]
pub enum TravelResult {
    Success(Area),
    Failure,
}

fn apply_violence_stress(tribute: &mut Tribute) {
    let kills = tribute.kills.unwrap_or(0);
    let wins = tribute.wins.unwrap_or(0);
    let sanity = tribute.sanity;
    let mut terror = 20.0;

    if kills + wins > 0 {
        terror = (100.0 / (kills + wins) as f64) * (sanity as f64 / 100.0) / 2.0;
    }

    if terror.round() > 0.0 {
        tribute.takes_mental_damage(terror.round() as i32);
        println!("😱 {} is horrified by the violence, loses {} sanity.", tribute.name, terror.round() as i32);
    }
}

fn attack_contest(tribute: Tribute, target: Tribute) -> AttackResult {
    let mut tribute1_roll = thread_rng().gen_range(1..=20); // Base roll
    tribute1_roll += tribute.strength.unwrap(); // Add strength

    // Add luck in here?

    let mut tribute2_roll = thread_rng().gen_range(1..=20); // Base roll
    tribute2_roll += target.dexterity.unwrap(); // Add dexterity

    if tribute1_roll > tribute2_roll {
        AttackResult::AttackerWins
    } else if tribute2_roll > tribute1_roll {
        AttackResult::DefenderWins
    } else {
        AttackResult::Miss
    }
}

pub fn pick_target(tribute: TributeModel, targets: Vec<Tribute>) -> Option<Tribute> {
    match targets.len() {
        0 => { // there are no other targets
            match tribute.sanity {
                0..=9 => { // attempt suicide
                    println!("{} attempts suicide.", tribute.name);
                    Some(tribute.into())
                },
                10..=19 => match thread_rng().gen_bool(0.2) {
                    true => { // attemp suicide
                        println!("{} attempts suicide.", tribute.name);
                        Some(tribute.into())
                    },
                    false => None, // Attack no one
                },
                _ => None, // Attack no one
            }
        },
        _ => {
            let mut targets = targets.clone();
            let enemy_targets: Vec<Tribute> = targets.iter().cloned()
                .filter(|t| t.district != tribute.district)
                .filter(|t| t.is_visible())
                .collect();

            match tribute.sanity {
                0..20 => targets = enemy_targets.clone(), // Sanity is low, target everyone
                _ => ()
            }

            match enemy_targets.len() {
                0 => Some(targets.first()?.clone()), // Sorry, buddy, time to die
                1 => Some(enemy_targets.first()?.clone()), // Easy choice
                _ => {
                    let mut rng = thread_rng();
                    Some(enemy_targets.choose(&mut rng)?.clone()) // Get a random enemy
                }
            }
        }
    }
}

pub fn do_combat(tribute1: &mut Tribute, tribute2: &mut Tribute) -> AttackOutcome {
    // TODO: Add in some sort of bravery/option-weighing here?
    tribute1.attacks(tribute2)
}

impl Default for Tribute {
    fn default() -> Self {
        Self::new("Tribute".to_string(), None)
    }
}

use crate::models::{get_area, get_game_by_id, update_tribute, Action, Tribute as TributeModel};
impl From<TributeModel> for Tribute {
    fn from(tribute: models::tribute::Tribute) -> Self {
        use crate::areas::Area;
        use crate::tributes::actions::TributeAction;

        let area = Area::from(tribute.area().unwrap());
        let actions: Vec<TributeAction> = tribute.actions()
            .iter()
            .map(TributeAction::from)
            .collect();

        let brain = TributeBrain {
            previous_actions: actions,
            preferred_action: None,
            preferred_action_percentage: 0.0,
        };

        Self {
            id: Some(tribute.id),
            game_id: tribute.game_id,
            name: tribute.name.clone(),
            health: tribute.health,
            sanity: tribute.sanity,
            movement: tribute.movement,
            district: tribute.district,
            brain,
            area: Some(area),
            day_killed: tribute.day_killed,
            killed_by: tribute.killed_by.clone(),
            kills: tribute.kills,
            wins: tribute.wins,
            defeats: tribute.defeats,
            draws: tribute.draws,
            games: tribute.games,
            bravery: tribute.bravery,
            loyalty: tribute.loyalty,
            speed: tribute.speed,
            intelligence: tribute.intelligence,
            persuasion: tribute.persuasion,
            luck: tribute.luck,
            strength: tribute.strength,
            defense: tribute.defense,
            is_hidden: tribute.is_hidden,
            dexterity: tribute.dexterity,
            status: TributeStatus::from_str(tribute.status.as_str()).unwrap(),
        }
    }
}

impl Into<UpdateTribute> for Tribute {
    fn into(self) -> UpdateTribute {
        let area = self.area.as_ref().unwrap();
        let area: i32 = get_area(&area.as_str()).id;
        let name = self.name.clone();

        UpdateTribute {
            id: self.id.unwrap(),
            game_id: self.game_id.unwrap(),
            name,
            health: self.health,
            sanity: self.sanity,
            movement: self.movement,
            district: self.district,
            area_id: Some(area),
            day_killed: self.day_killed,
            killed_by: self.killed_by.clone(),
            kills: self.kills,
            wins: self.wins,
            defeats: self.defeats,
            draws: self.draws,
            games: self.games,
            is_hidden: self.is_hidden,
            dexterity: self.dexterity,
            status: self.status.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let tribute = Tribute::new("Katniss".to_string(), None);
        assert_eq!(tribute.health, 100);
        assert_eq!(tribute.sanity, 100);
        assert_eq!(tribute.movement, 100);
        assert_eq!(tribute.status, TributeStatus::Healthy);
    }

    #[test]
    fn takes_physical_damage() {
        let mut tribute = Tribute::new("Katniss".to_string(), None);
        tribute.takes_physical_damage(10);
        assert_eq!(tribute.health, 90);
    }

    #[test]
    fn takes_mental_damage() {
        let mut tribute = Tribute::new("Katniss".to_string(), None);
        tribute.takes_mental_damage(10);
        assert_eq!(tribute.sanity, 90);
    }

    #[test]
    fn moves_and_rests() {
        let mut tribute = Tribute::new("Katniss".to_string(), None);
        tribute.speed = Some(50);
        tribute.moves();
        assert_eq!(tribute.movement, 50);
        tribute.short_rests();
        assert_eq!(tribute.movement, 100);
    }

    #[test]
    fn is_hidden_true() {
        let mut tribute = Tribute::new("Katniss".to_string(), None);
        tribute.intelligence = Some(100);
        tribute.is_hidden = Some(true);
        assert!(!tribute.is_visible());
    }
}
