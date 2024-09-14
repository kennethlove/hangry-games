use std::str::FromStr;
use crate::areas::Area;
use rand::prelude::*;

use super::actions::{TributeAction, AttackResult, AttackOutcome, PreferredAction};
use super::statuses::TributeStatus;


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

#[derive(Clone, Debug, PartialEq)]
pub struct TributeBrain {
    previous_actions: Vec<TributeAction>,
    preferred_action: Option<PreferredAction>,
    preferred_action_percentage: f64,
}

impl TributeBrain {
    fn new() -> Self {
        Self {
            previous_actions: Vec::new(),
            preferred_action: None,
            preferred_action_percentage: 0.0,
        }
    }

    pub fn set_preferred_action(&mut self, action: PreferredAction, percentage: f64) {
        self.preferred_action = Some(action);
        self.preferred_action_percentage = percentage;
    }

    pub fn clear_preferred_action(&mut self) {
        self.preferred_action = None;
        self.preferred_action_percentage = 0.0;
    }

    /// Decide on an action for the tribute to take
    /// First weighs any preferred actions, then decides based on current state
    pub fn act(&mut self, tribute: &mut Tribute, nearby_tributes: Vec<Tribute>) -> TributeAction {
        if tribute.health == 0 { return TributeAction::None; }

        if let Some(preferred_action) = self.clone().preferred_action {
            if thread_rng().gen_bool(self.preferred_action_percentage) {
                match preferred_action {
                    PreferredAction::Move(area) => {
                        self.previous_actions.push(TributeAction::Move);
                        if tribute.area.is_some() {
                            tribute.changes_area(area.unwrap());
                        }
                        return TributeAction::Move;
                    },
                    _ => (),
                }
            }
        }

        let action = TributeBrain::decide_on_action(tribute, nearby_tributes.clone());

        // Try to get a different action?

        self.previous_actions.push(action.clone());
        action
    }

    /// Get the last action taken by the tribute
    pub fn last_action(&self) -> TributeAction {
        if let Some(previous_action) = self.previous_actions.last() {
            previous_action.clone()
        } else {
            TributeAction::None
        }
    }

    /// The AI for a tribute. Automatic decisions based on current state.
    fn decide_on_action(tribute: &Tribute, nearby_tributes: Vec<Tribute>) -> TributeAction {
        // If the tribute isn't in the area, they do nothing
        if tribute.area.is_none() {
            return TributeAction::None;
        }
        if tribute.movement <= 0 {
            return TributeAction::Rest;
        }

        let _area = tribute.area.as_ref().unwrap();

        match &nearby_tributes.len() {
            0 => {
                match tribute.health {
                    // health is low, rest
                    1..=10 => TributeAction::Rest,
                    // health isn't great, hide
                    11..=15 => TributeAction::Hide,
                    // health is good, move
                    _ => {
                        // If the tribute has movement, move
                        match tribute.movement {
                            0 => TributeAction::Rest,
                            _ => TributeAction::Move,
                        }
                    }
                }
            }
            1..6 => {
                // Enemies are nearby, attack depending on health
                match tribute.health {
                    // health is low, hide
                    1..=5 => TributeAction::Hide,
                    // health isn't great, run away
                    6..=10 => TributeAction::Move,
                    // health is good, attack
                    _ => TributeAction::Attack,
                }
            },
            _ => {
                // More than 5 enemies? Intelligence decides next move
                match tribute.intelligence {
                    // Too dumb to know better, attacks
                    Some(0..36) => TributeAction::Attack,
                    // Smart enough to know better, hides
                    Some(85..101) => TributeAction::Hide,
                    // Average intelligence, moves
                    _ => TributeAction::Move,
                }
            }
        }
    }
}


impl Tribute {
    /// Creates a new Tribute with full health, sanity, and movement
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

    /// Reduces health
    pub fn takes_physical_damage(&mut self, damage: i32) {
        self.health = std::cmp::max(0, self.health - damage);

        if self.health == 0 {
            self.dies();
        }
    }

    /// Reduces mental health
    pub fn takes_mental_damage(&mut self, damage: i32) {
        self.sanity = std::cmp::max(0, self.sanity - damage);
    }

    /// Restores health
    pub fn heals(&mut self, health: i32) {
        self.health = std::cmp::min(100, self.health + health);
    }

    /// Restores mental health
    pub fn heals_mental_damage(&mut self, health: i32) {
        self.sanity = std::cmp::min(100, self.sanity + health);
    }

    pub fn moves(&mut self) {
        self.movement = std::cmp::max(0, self.movement - 50);
    }

    pub fn rests(&mut self) {
        self.movement = 100;
    }

    pub fn dies(&mut self) {
        self.status = TributeStatus::RecentlyDead;
    }

    pub fn changes_area(&mut self, area: Area) {
        self.area = Some(area);
    }

    pub fn leaves_area(&mut self) {
        self.area = None;
    }

    pub fn hides(&mut self) {
        self.is_hidden = Some(true);
    }

    pub fn reveals(&mut self) {
        self.is_hidden = Some(false);
    }

    pub fn bleeds(&mut self) {
        if self.status == TributeStatus::Wounded {
            self.takes_physical_damage(2);
            println!("{} bleeds.", self.name);
        }
    }

    pub fn attacks(&mut self, target: &mut Tribute) -> AttackOutcome {
        let game = get_game_by_id(self.game_id.unwrap()).unwrap();
        match attack_contest(self.clone(), target.clone()) {
            AttackResult::AttackerWins => {
                println!("{} attacks {}, and wins!", self.name, target.name);
                target.takes_physical_damage(self.strength.unwrap());
                apply_violence_stress(self);

                if target.status == TributeStatus::RecentlyDead {
                    self.kills = Some(self.kills.unwrap() + 1);
                    self.wins = Some(self.wins.unwrap() + 1);
                    target.killed_by = Some(self.name.clone());
                    target.day_killed = Some(game.day.unwrap());
                    target.defeats = Some(target.defeats.unwrap() + 1);
                    return AttackOutcome::Kill(self.clone(), target.clone());
                }
                target.status = TributeStatus::Wounded;
                AttackOutcome::Wound(self.clone(), target.clone())
            }
            AttackResult::DefenderWins => {
                println!("{} attacks {}, but loses!", self.name, target.name);
                self.takes_physical_damage(target.strength.unwrap());
                apply_violence_stress(target);

                if self.status == TributeStatus::RecentlyDead {
                    target.kills = Some(target.kills.unwrap() + 1);
                    target.wins = Some(target.wins.unwrap() + 1);
                    self.killed_by = Some(target.name.clone());
                    self.day_killed = Some(game.day.unwrap());
                    self.defeats = Some(self.defeats.unwrap() + 1);
                    return AttackOutcome::Kill(target.clone(), self.clone());
                }
                self.status = TributeStatus::Wounded;
                AttackOutcome::Wound(target.clone(), self.clone())
            }
            AttackResult::Miss => {
                println!("{} attacks {}, but misses!", self.name, target.name);
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

    pub fn travels(&self, closed_areas: Vec<Area>) -> TravelResult {
        let mut rng = thread_rng();
        let area = self.clone().area.unwrap();

        if self.movement > 0 {
            let neighbors = area.neighbors();
            let new_area = loop {
                let new_area = neighbors.choose(&mut rng).unwrap();
                if new_area == &area || closed_areas.contains(new_area) {
                    continue;
                }
                break new_area.clone();
            };
            TravelResult::Success(new_area)
        } else {
            TravelResult::Failure
        }
    }
}

#[derive(Debug)]
pub enum TravelResult {
    Success(Area),
    Failure,
}

fn apply_violence_stress(tribute: &mut Tribute) {
    tribute.takes_mental_damage(10);
}

fn attack_contest(tribute: Tribute, target: Tribute) -> AttackResult {
    let mut tribute1_roll = thread_rng().gen_range(1..=20); // Base roll
    tribute1_roll += tribute.strength.unwrap(); // Add strength

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
                0..=9 => Some(targets.first()?.clone()), // attempt suicide
                10..=19 => match thread_rng().gen_bool(0.2) {
                    true => Some(targets.first()?.clone()), // attempt suicide
                    false => None, // Attack no one
                },
                _ => None, // Attack no one
            }
        },
        _ => {
            let enemy_targets: Vec<Tribute> = targets.iter().cloned()
                .filter(|t| t.district != tribute.district)
                .filter(|t| t.is_visible())
                .collect();
            match enemy_targets.len() {
                0 => Some(targets.first()?.clone()), // Sorry, buddy, time to die
                1 => Some(enemy_targets.first()?.clone()), // Easy choice
                _ => {
                    let mut rng = thread_rng();
                    Some(enemy_targets.choose(&mut rng)?.clone()) // Get a random person
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

use crate::models::{get_area, get_game_by_id, Tribute as TributeModel};
impl From<TributeModel> for Tribute {
    fn from(tribute: crate::models::tribute::Tribute) -> Self {
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

use crate::models::tribute::UpdateTribute;
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
        assert_eq!(tribute.status, TributeStatus::Wounded);
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
        tribute.speed = Some(10);
        tribute.moves();
        assert_eq!(tribute.movement, 90);
        tribute.rests();
        assert_eq!(tribute.movement, 100);
    }

    #[test]
    fn takes_damage_and_dies() {
        let mut tribute = Tribute::new("Katniss".to_string(), None);
        tribute.takes_physical_damage(100);
        assert_eq!(tribute.status, TributeStatus::Dead);
    }

    #[test]
    fn decide_on_action_default() {
        // If there are no enemies nearby, the tribute should move
        let mut tribute = Tribute::new("Katniss".to_string(), None);
        let action = tribute.brain.act(&tribute.clone(), vec![]);
        assert_eq!(action, TributeAction::Move);
    }

    #[test]
    fn decide_on_action_low_health() {
        // If the tribute has low health, they should rest
        let mut tribute = Tribute::new("Katniss".to_string(), None);
        tribute.takes_physical_damage(90);
        let action = tribute.brain.act(&tribute.clone(), vec![]);
        assert_eq!(action, TributeAction::Rest);
    }

    #[test]
    fn decide_on_action_no_movement() {
        // If the tribute has no movement, they should rest
        let mut tribute = Tribute::new("Katniss".to_string(), None);
        tribute.speed = Some(100);
        tribute.moves();
        let action = tribute.brain.act(&tribute.clone(), vec![]);
        assert_eq!(action, TributeAction::Rest);
    }

    #[test]
    fn decide_on_action_enemies() {
        // If there are enemies nearby, the tribute should attack
        let mut tribute = Tribute::new("Katniss".to_string(), None);
        let tribute2 = Tribute::new("Peeta".to_string(), None);
        let action = tribute.brain.act(&tribute.clone(), vec![tribute.clone(), tribute2]);
        assert_eq!(action, TributeAction::Attack);
    }

    #[test]
    fn decide_on_action_enemies_low_health() {
        // If there are enemies nearby, but the tribute is low on health
        // the tribute should hide
        let mut tribute = Tribute::new("Katniss".to_string(), None);
        tribute.takes_physical_damage(90);
        let tribute2 = Tribute::new("Peeta".to_string(), None);
        let action = tribute.brain.act(&tribute.clone(),vec![tribute.clone(), tribute2]);
        assert_eq!(action, TributeAction::Hide);
    }

    #[test]
    fn is_hidden_true() {
        let mut tribute = Tribute::new("Katniss".to_string(), None);
        tribute.intelligence = Some(100);
        tribute.is_hidden = Some(true);
        assert!(!tribute.is_visible());
    }
}
