use super::actions::{AttackOutcome, AttackResult, TributeAction};
use super::brains::TributeBrain;
use super::statuses::TributeStatus;
use crate::areas::Area;
use crate::events::TributeEvent;
use crate::models;
use crate::models::tribute::UpdateTribute;
use rand::prelude::*;
use std::str::FromStr;
use crate::items::{Attribute, Item};
use crate::messages::GameMessage;

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

    pub fn delete(id: i32) {
        models::tribute::Tribute::delete(id);
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
        println!("{}", GameMessage::TributeRest(self.clone()));
        self.movement = 100;
    }

    pub fn long_rests(&mut self) {
        println!("{}", GameMessage::TributeLongRest(self.clone()));
        self.short_rests();
        self.heals(5);
        self.heals_mental_damage(5);
    }

    /// Marks the tribute as recently dead and reveals them.
    pub fn dies(&mut self) {
        self.status = TributeStatus::RecentlyDead;
        self.is_hidden = Some(false);
    }

    pub fn is_alive(&self) -> bool {
        match (self.status.clone(), self.health) {
            (_, 0) => false,
            (TributeStatus::RecentlyDead | TributeStatus::Dead, _) => false,
            _ => true,
        }
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
        println!("{}", GameMessage::TributeHide(self.clone()));
        self.is_hidden = Some(true);
    }

    /// Reveals the tribute to view.
    pub fn reveals(&mut self) {
        self.is_hidden = Some(false);
    }

    /// Tribute is lonely/homesick/etc., loses some sanity.
    pub fn suffers(&mut self) {
        let game = get_game_by_id(self.game_id.unwrap()).unwrap();
        let district_mates = get_all_living_tributes(&game).iter()
            .filter(|t| t.district == self.district)
            .filter(|t| self.area == Some(Area::from(get_area_by_id(t.area_id).unwrap())))
            .count() as f64;

        let loneliness = self.bravery.unwrap_or(0) as f64 / 100.0;  // how lonely is the tribute?
        let terror = (self.sanity as f64 / 100.0) * game.day.unwrap() as f64; // how scared are they?
        let connectedness = district_mates * loneliness;
        let terror = terror - connectedness;

        if terror.round() > 1.0 {
            println!("{}", GameMessage::TributeSuffer(self.clone()));
            self.takes_mental_damage(terror.round() as i32);
        }
    }

    pub fn attacks(&mut self, target: &mut Tribute) -> AttackOutcome {
        if self == target {
            println!("{}", GameMessage::TributeSelfHarm(self.clone()));
        }

        match attack_contest(self.clone(), target.clone()) {
            AttackResult::AttackerWins => {
                println!("{}", GameMessage::TributeAttackWin(self.clone(), target.clone()));
                target.takes_physical_damage(self.strength.unwrap());
                target.defeats = Some(target.defeats.unwrap_or(0) + 1);
                self.wins = Some(self.wins.unwrap_or(0) + 1);

                if target.health > 0 {
                    println!("{}", GameMessage::TributeAttackWound(self.clone(), target.clone()));
                    return AttackOutcome::Wound(self.clone(), target.clone())
                }
            }
            AttackResult::AttackerWinsDecisively => {
                println!("{}", GameMessage::TributeAttackWinExtra(self.clone(), target.clone()));
                target.takes_physical_damage(self.strength.unwrap() * 2);
                target.defeats = Some(target.defeats.unwrap_or(0) + 1);
                self.wins = Some(self.wins.unwrap_or(0) + 1);

                if target.health > 0 {
                    println!("{}", GameMessage::TributeAttackWound(self.clone(), target.clone()));
                    return AttackOutcome::Wound(self.clone(), target.clone())
                }
            }
            AttackResult::DefenderWins => {
                println!("{}", GameMessage::TributeAttackLose(self.clone(), target.clone()));
                self.takes_physical_damage(target.strength.unwrap());
                self.defeats = Some(self.defeats.unwrap() + 1);
                target.wins = Some(target.wins.unwrap() + 1);

                if self.health > 0 {
                    println!("{}", GameMessage::TributeAttackWound(target.clone(), self.clone()));
                    return AttackOutcome::Wound(target.clone(), self.clone())
                }
            }
            AttackResult::DefenderWinsDecisively => {
                println!("{}", GameMessage::TributeAttackLoseExtra(target.clone(), self.clone()));
                self.takes_physical_damage(target.strength.unwrap() * 2);
                self.defeats = Some(self.defeats.unwrap() + 1);
                target.wins = Some(target.wins.unwrap() + 1);

                if self.health > 0 {
                    println!("{}", GameMessage::TributeAttackWound(target.clone(), self.clone()));
                    return AttackOutcome::Wound(target.clone(), self.clone())
                }
            }
            AttackResult::Miss => {
                println!("{}", GameMessage::TributeAttackMiss(self.clone(), target.clone()));
                self.draws = Some(self.draws.unwrap() + 1);
                target.draws = Some(target.draws.unwrap() + 1);

                return AttackOutcome::Miss(self.clone(), target.clone())
            }
        };

        if self.health <= 0 {
            println!("{}", GameMessage::TributeAttackKill(self.clone(), target.clone()));
            self.killed_by = Some(target.name.clone());
            self.status = TributeStatus::RecentlyDead;
            self.dies();
            AttackOutcome::Kill(target.clone(), self.clone())
        } else if target.health <= 0 {
            println!("{}", GameMessage::TributeAttackSuccessKill(self.clone(), target.clone()));
            target.killed_by = Some(self.name.clone());
            target.status = TributeStatus::RecentlyDead;
            target.dies();
            AttackOutcome::Kill(self.clone(), target.clone())
        } else {
            return AttackOutcome::Miss(self.clone(), target.clone())
        }

        // apply_violence_stress(self);
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
            println!("{}", GameMessage::TributeTravelAlreadyThere(self.clone(), suggested_area.unwrap()));
            return TravelResult::Failure;
        }

        let handle_suggested_area = || -> TravelResult {
            if suggested_area.is_some() {
                println!("{}", GameMessage::TributeTravel(self.clone(), area.clone(), suggested_area.clone().unwrap()));
                return TravelResult::Success(suggested_area.unwrap());
            }
            TravelResult::Failure
        };

        match self.movement {
            // No movement left, can't move
            0 => {
                println!("{}", GameMessage::TributeTravelTooTired(self.clone(), area.clone()));
                TravelResult::Failure
            },
            // Low movement, can only move to suggested area
            1..=10 => {
                match handle_suggested_area() {
                    TravelResult::Success(area) => TravelResult::Success(area),
                    TravelResult::Failure => {
                        println!("{}", GameMessage::TributeTravelTooTired(self.clone(), area.clone()));
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
                for area in &neighbors {
                    if area.tributes(self.game_id.unwrap()).iter()
                        .filter(|t| t.district == self.district)
                        .count() > 0 {
                            println!("{}", GameMessage::TributeTravelFollow(self.clone(), area.clone()));
                            return TravelResult::Success(area.clone());
                    }
                }
                let mut count = 0;
                let new_area = loop {
                    let new_area = neighbors.choose(&mut rng).unwrap();
                    if new_area == &area || closed_areas.contains(new_area) {
                        count += 1;

                        if count == 10 {
                            println!("{}", GameMessage::TributeTravelStay(self.clone(), area.clone()));
                            return TravelResult::Success(area.clone());
                        }

                        continue;
                    }
                    break new_area.clone();
                };
                println!("{}", GameMessage::TributeTravel(self.clone(), area.clone(), new_area.clone()));
                TravelResult::Success(new_area)
            }
        }
    }

    pub fn process_status(&mut self) {
        let status = self.status.clone();
        match status {
            TributeStatus::Wounded => {
                self.takes_physical_damage(1);
                println!("{}", GameMessage::TributeBleeds(self.clone()));
            },
            TributeStatus::Sick => {
                self.strength = Some(std::cmp::max(1, self.strength.unwrap() - 1));
                self.speed = Some(std::cmp::max(1, self.speed.unwrap() - 1));
                println!("{}", GameMessage::TributeSick(self.clone()));
            },
            TributeStatus::Electrocuted => {
                println!("{}", GameMessage::TributeElectrocuted(self.clone()));
                self.takes_physical_damage(20);
            },
            TributeStatus::Frozen => {
                println!("{}", GameMessage::TributeFrozen(self.clone()));
                self.speed = Some(std::cmp::max(1, self.speed.unwrap() - 1));
            },
            TributeStatus::Overheated => {
                println!("{}", GameMessage::TributeOverheated(self.clone()));
                self.speed = Some(std::cmp::max(1, self.speed.unwrap() - 1));
            },
            TributeStatus::Dehydrated => {
                println!("{}", GameMessage::TributeDehydrated(self.clone()));
                self.strength = Some(std::cmp::max(1, self.strength.unwrap() - 1));
            },
            TributeStatus::Starving => {
                println!("{}", GameMessage::TributeStarving(self.clone()));
                self.strength = Some(std::cmp::max(1, self.strength.unwrap() - 1));
            },
            TributeStatus::Poisoned => {
                println!("{}", GameMessage::TributePoisoned(self.clone()));
                self.takes_mental_damage(5);
            },
            TributeStatus::Broken => {
                // coin flip for which bone breaks
                let leg_bone = thread_rng().gen_bool(0.5);

                // TODO: Add in other bones? Ribs and skull make sense.

                if leg_bone {
                    println!("{}", GameMessage::TributeBrokenLeg(self.clone()));
                    self.speed = Some(std::cmp::max(1, self.speed.unwrap() - 5));
                } else {
                    println!("{}", GameMessage::TributeBrokenArm(self.clone()));
                    self.strength = Some(std::cmp::max(1, self.strength.unwrap() - 5));
                }
            },
            TributeStatus::Infected => {
                println!("{}", GameMessage::TributeInfected(self.clone()));
                self.takes_physical_damage(2);
                self.takes_mental_damage(2);
            },
            TributeStatus::Drowned => {
                println!("{}", GameMessage::TributeDrowned(self.clone()));
                self.takes_physical_damage(2);
                self.takes_mental_damage(2);
            },
            TributeStatus::Mauled(animal) => {
                let number_of_animals = thread_rng().gen_range(2..=5);
                let damage = animal.damage() * number_of_animals;
                println!("{}", GameMessage::TributeMauled(self.clone(), number_of_animals, animal.clone(), damage));
                self.takes_physical_damage(damage);
            },
            TributeStatus::Burned => {
                println!("{}", GameMessage::TributeBurned(self.clone()));
                self.takes_physical_damage(5);
            }
            _ => {}
        }

        if self.health <= 0 {
            println!("{}", GameMessage::TributeDiesFromStatus(self.clone(), self.status.clone()));
            create_full_log(
                self.game_id.unwrap(),
                GameMessage::TributeDiesFromStatus(self.clone(), self.status.clone()).to_string(),
                None,
                Some(self.area.clone().unwrap().id()),
                Some(self.id.unwrap()),
                None,
                None
            );
            self.killed_by = Some(self.status.to_string());
            self.status = TributeStatus::RecentlyDead;
        }
    }

    pub fn handle_event(&mut self, tribute_event: TributeEvent) {
        match tribute_event {
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
            println!("{}", GameMessage::TributeDiesFromTributeEvent(self.clone(), tribute_event.clone()));
            create_full_log(
                self.game_id.unwrap(),
                GameMessage::TributeDiesFromTributeEvent(self.clone(), tribute_event.clone()).to_string(),
                None,
                Some(self.area.clone().unwrap().id()),
                Some(self.id.unwrap()),
                None,
                None
            );
            self.killed_by = Some(self.status.to_string());
            self.status = TributeStatus::RecentlyDead;
        }
    }

    pub fn do_day_night(&mut self, suggested_action: Option<TributeAction>, probability: Option<f64>, day: bool) -> Tribute {
        // Tribute is already dead, do nothing.
        if !self.is_alive() {
            println!("{}", GameMessage::TributeAlreadyDead(self.clone()));
            create_full_log(
                self.game_id.unwrap(),
                GameMessage::TributeAlreadyDead(self.clone()).to_string(),
                None,
                Some(self.area.clone().unwrap().id()),
                Some(self.id.unwrap()),
                None,
                None
            );
            return self.clone();
        }

        // Update the tribute based on the period's events.
        self.process_status();

        // Nighttime terror
        if !day && self.is_alive() {
            self.suffers();

            // Gift from patrons?
            let chance = match self.district {
                1 | 2 => 1.0 / 10.0,
                3 | 4 => 1.0 / 15.0,
                5 | 6 => 1.0 / 20.0,
                7 | 8 => 1.0 / 25.0,
                9 | 10 => 1.0 / 30.0,
                _ => 1.0 / 50.0,
            };

            if thread_rng().gen_bool(chance) {
                let item = Item::new_random("Gift".to_string(), self.game_id, None, self.id);
                println!("{}", GameMessage::SponsorGift(self.clone(), item.clone()));
                create_full_log(
                    self.game_id.unwrap(),
                    GameMessage::SponsorGift(self.clone(), item.clone()).to_string(),
                    None,
                    None,
                    Some(self.id.unwrap()),
                    None,
                    None
                );
            }
        }

        // Tribute died to the period's events.
        if self.status == TributeStatus::RecentlyDead || self.health <= 0 {
            println!("{}", GameMessage::TributeDead(self.clone()));
            create_full_log(
                self.game_id.unwrap(),
                GameMessage::TributeDead(self.clone()).to_string(),
                None,
                Some(self.area.clone().unwrap().id()),
                Some(self.id.unwrap()),
                None,
                None
            );
            return self.clone();
        }

        let game = get_game_by_id(self.game_id.unwrap()).unwrap();
        let area = self.area.clone().unwrap();
        let closed_areas = game.closed_areas().clone();

        // Area is closed, tribute must move.
        if closed_areas.contains(&area) {
            self.travels(closed_areas.clone(), None);
            create_full_log(
                self.game_id.unwrap(),
                GameMessage::TributeTravel(self.clone(), area.clone(), self.area.clone().unwrap()).to_string(),
                Some(get_action("Move").id),
                Some(area.id()),
                Some(self.id.unwrap()),
                None,
                Some(self.area.clone().unwrap().id())
            );
            return self.clone();
        }

        let brain = &mut self.brain.clone();

        if suggested_action.is_some() {
            brain.set_preferred_action(suggested_action.unwrap(), probability.unwrap());
        }

        let nearby_tributes = get_all_living_tributes(&game).iter()
            .filter(|t| t.area().is_some())
            .map(|t| Tribute::from(t.clone()))
            .filter(|t| t.clone().area.unwrap() == area)
            .collect::<Vec<_>>().len();

        let action = brain.act(self, nearby_tributes, closed_areas.clone());

        match &action {
            TributeAction::Move(area) => {
                match self.travels(closed_areas.clone(), area.clone()) {
                    TravelResult::Success(area) => {
                        self.changes_area(area.clone());
                        self.take_action(action.clone(), Some(area.clone().to_string()));
                        create_full_log(
                            self.game_id.unwrap(),
                            GameMessage::TributeTravel(self.clone(), self.area.clone().unwrap(), area.clone()).to_string(),
                            Some(get_action(action.clone().as_str()).id),
                            Some(self.area.clone().unwrap().id()),
                            Some(self.id.unwrap()),
                            Some(action.clone().as_str().to_string()),
                            Some(area.id())
                        );
                    },
                    TravelResult::Failure => {
                        println!("{}", GameMessage::TributeRest(self.clone()));
                        create_full_log(
                            self.game_id.unwrap(),
                            GameMessage::TributeRest(self.clone()).to_string(),
                            Some(get_action(action.clone().as_str()).id),
                            Some(self.area.clone().unwrap().id()),
                            Some(self.id.unwrap()),
                            Some(action.clone().as_str().to_string()),
                            None
                        );
                        self.short_rests();
                        self.take_action(action.clone(), None);
                    }
                }
            },
            TributeAction::Hide => {
                println!("{}", GameMessage::TributeHide(self.clone()));
                create_full_log(
                    self.game_id.unwrap(),
                    GameMessage::TributeHide(self.clone()).to_string(),
                    Some(get_action(action.clone().as_str()).id),
                    Some(self.area.clone().unwrap().id()),
                    Some(self.id.unwrap()),
                    Some(action.clone().as_str().to_string()),
                    None
                );
                self.hides();
                self.take_action(action, None);
            },
            TributeAction::Rest => {
                println!("{}", GameMessage::TributeLongRest(self.clone()));
                create_full_log(
                    self.game_id.unwrap(),
                    GameMessage::TributeLongRest(self.clone()).to_string(),
                    Some(get_action(action.clone().as_str()).id),
                    Some(self.area.clone().unwrap().id()),
                    Some(self.id.unwrap()),
                    Some(action.clone().as_str().to_string()),
                    None
                );
                self.long_rests();
                self.take_action(action, None);
            },
            TributeAction::Attack => {
                let target = pick_target(self.clone().into());
                if let Some(mut target) = target {
                    if target.is_visible() {
                        match self.attacks(&mut target) {
                            AttackOutcome::Kill(mut attacker, mut target) => {
                                if attacker.health <= 0 {
                                    attacker.dies();
                                }
                                if target.health <= 0 {
                                    target.dies();
                                }
                                if attacker.id == target.id {
                                    attacker.health = target.health.clone();
                                    attacker.day_killed = target.day_killed.clone();
                                    attacker.killed_by = target.killed_by.clone();
                                    attacker.status = target.status.clone();
                                    return target;
                                }
                                update_tribute(attacker.id.unwrap(), attacker.clone().into());
                                update_tribute(target.id.unwrap(), target.clone().into());
                                create_full_log(
                                    self.game_id.unwrap(),
                                    GameMessage::TributeAttackKill(attacker.clone(), target.clone()).to_string(),
                                    Some(get_action(action.as_str()).id),
                                    Some(self.area.clone().unwrap().id()),
                                    Some(attacker.id.unwrap()),
                                    Some(action.as_str().to_string()),
                                    Some(target.id.unwrap())
                                );
                            },
                            _ => ()
                        }
                        self.take_action(action, Some(target.clone().name));
                    } else {
                        println!("{}", GameMessage::TributeAttackHidden(self.clone(), target.clone()));
                        create_full_log(
                            self.game_id.unwrap(),
                            GameMessage::TributeAttackHidden(self.clone(), target.clone()).to_string(),
                            Some(get_action(action.clone().as_str()).id),
                            Some(self.area.clone().unwrap().id()),
                            Some(self.id.unwrap()),
                            Some(action.clone().as_str().to_string()),
                            Some(target.id.unwrap())
                        );
                        self.take_action(TributeAction::Attack, None);
                    }
                }
            },
            TributeAction::None => {
                self.long_rests();
                self.take_action(action, None);
            },
            TributeAction::TakeItem => {
                let item = self.take_nearby_item(area);
                println!("{}", GameMessage::TributeTakeItem(self.clone(), item.clone()));
                self.take_action(action, Some(item.name.clone()));
            },
            TributeAction::UseItem(None) => {
                // Get consumable items
                let mut items = self.consumable_items();
                if items.is_empty() {
                    self.long_rests();
                    self.take_action(TributeAction::Rest, None);
                } else {
                    // Use random item
                    let item = items.choose_mut(&mut thread_rng()).unwrap();
                    match self.use_consumable(item.clone()) {
                        true => {
                            println!("{}", GameMessage::TributeUseItem(self.clone(), item.clone()));
                            self.take_action(action, Some(item.name.clone()));
                        },
                        false => {
                            println!("{}", GameMessage::TributeCannotUseItem(self.clone(), item.clone()));
                            self.short_rests();
                            self.take_action(TributeAction::Rest, None);
                        }
                    };
                }
            }
            TributeAction::UseItem(item) => {
                let items = self.consumable_items();
                if let Some(item) = item {
                    let selected_item = items.iter().find(|i| i.name == item.clone());
                    if selected_item.is_some() {
                        match self.use_consumable(selected_item.unwrap().clone()) {
                            true => {
                                println!("{}", GameMessage::TributeUseItem(self.clone(), selected_item.unwrap().clone()));
                                self.take_action(action, Some(selected_item.unwrap().name.clone()));
                            },
                            false => {
                                println!("{}", GameMessage::TributeCannotUseItem(self.clone(), selected_item.unwrap().clone()));
                                self.short_rests();
                                self.take_action(TributeAction::Rest, None);
                            }
                        };
                    }
                }
            }
        }
        self.clone()
    }

    fn take_action(&self, action: TributeAction, target: Option<String>) {
        use models::tribute_action::take_action;
        use models::action::get_action;

        let tribute = TributeModel::from(self.clone());
        let action = Action::from(get_action(action.as_str()));
        take_action(&tribute, &action, target);
    }

    fn take_nearby_item(&self, area: Area) -> Item {
        let mut rng = thread_rng();
        let mut items = area.available_items(self.game_id.unwrap());
        let item = items.choose_mut(&mut rng).unwrap();
        self.take_item(item.clone());
        item.clone()
    }

    fn take_item(&self, item: Item) {
        let tribute = TributeModel::from(self.clone());
        tribute.takes_item(item.id.unwrap());
    }

    fn use_consumable(&mut self, chosen_item: Item) -> bool {
        let items = self.consumable_items();
        #[allow(unused_assignments)]
        let mut item = items.iter().last().unwrap().clone();
        if let Some(selected_item) = items.iter()
            .filter(|i| i.name == chosen_item.name)
            .filter(|i| i.quantity > 0)
            .last()
        {
            item = selected_item.clone();
        } else {
            return false;
        }
        item.quantity -= 1;

        // Apply item effect
        match item.attribute {
            Attribute::Health => {
                self.heals(item.effect);
            },
            Attribute::Sanity => {
                self.heals_mental_damage(item.effect);
            },
            Attribute::Movement => {
                self.movement = std::cmp::min(100, self.movement + item.effect);
            },
            Attribute::Bravery => {
                self.bravery = Some(std::cmp::min(100, self.bravery.unwrap() + item.effect));
            },
            Attribute::Speed => {
                self.speed = Some(std::cmp::min(100, self.speed.unwrap() + item.effect));
            },
            Attribute::Strength => {
                self.strength = Some(std::cmp::min(50, self.strength.unwrap() + item.effect));
            },
            _ => ()
        }

        if item.quantity <= 0 {
            // No uses left
            TributeModel::from(self.clone()).uses_consumable(item.id.unwrap());
        } else {
            // Update item quantity
            update_item(models::UpdateItem::from(item.clone()).into());
        }
        update_tribute(self.id.unwrap(), self.clone().into());
        true
    }

    pub fn items(&self) -> Vec<Item> {
        let items = models::item::Item::get_by_tribute(self.game_id.unwrap(), self.id.unwrap());
        items.iter().filter(|i| i.quantity > 0).cloned().map(Item::from).collect()
    }

    pub fn weapons(&self) -> Vec<Item> {
        self.items().iter().cloned().filter(|i| i.is_weapon()).collect()
    }

    pub fn defensive_items(&self) -> Vec<Item> {
        self.items().iter().cloned().filter(|i| i.is_defensive()).collect()
    }

    pub fn consumable_items(&self) -> Vec<Item> {
        self.items().iter().cloned().filter(|i| i.is_consumable()).collect()
    }
}

#[derive(Debug)]
pub enum TravelResult {
    Success(Area),
    Failure,
}

#[allow(dead_code)]
fn apply_violence_stress(tribute: &mut Tribute) {
    let kills = tribute.kills.unwrap_or(0);
    let wins = tribute.wins.unwrap_or(0);
    let sanity = tribute.sanity;
    let mut terror = 20.0;

    if kills + wins > 0 {
        terror = (100.0 / (kills + wins) as f64) * (sanity as f64 / 100.0) / 2.0;
    }

    if terror.round() > 0.0 {
        println!("{}", GameMessage::TributeHorrified(tribute.clone(), terror.round() as i32));
        tribute.takes_mental_damage(terror.round() as i32);
    }
}

fn attack_contest(attacker: Tribute, target: Tribute) -> AttackResult {
    let mut tribute1_roll = thread_rng().gen_range(1..=20); // Base roll
    tribute1_roll += attacker.strength.unwrap(); // Add strength

    if let Some(weapon) = attacker.weapons().iter_mut().last() {
        tribute1_roll += weapon.effect; // Add weapon damage
        weapon.quantity -= 1;
        if weapon.quantity <= 0 {
            println!("{}", GameMessage::WeaponBreak(attacker.clone(), weapon.clone()));
            weapon.delete();
        }
        update_item(models::UpdateItem::from(weapon.clone()).into());
    }

    // Add luck in here?

    let mut tribute2_roll = thread_rng().gen_range(1..=20); // Base roll
    tribute2_roll += target.defense.unwrap(); // Add defense

    if let Some(shield) = target.items().iter_mut().filter(|i| i.is_defensive()).next() {
        tribute2_roll += shield.effect; // Add weapon defense
        shield.quantity -= 1;
        if shield.quantity <= 0 {
            println!("{}", GameMessage::ShieldBreak(target.clone(), shield.clone()));
            shield.delete();
        }
        update_item(models::UpdateItem::from(shield.clone()).into());
    }

    let response = {
        if tribute1_roll > tribute2_roll {
            if tribute1_roll >= tribute2_roll + 5 { // Attacker wins significantly
                AttackResult::AttackerWinsDecisively
            } else {
                AttackResult::AttackerWins
            }
        } else if tribute2_roll > tribute1_roll {
            if tribute2_roll >= tribute1_roll + 5 { // Defender wins significantly
                AttackResult::DefenderWinsDecisively
            } else {
                AttackResult::DefenderWins
            }
        } else {
            AttackResult::Miss
        }
    };
    response
}

pub fn pick_target(tribute: TributeModel) -> Option<Tribute> {
    let area = get_area_by_id(tribute.area_id).unwrap();
    let tributes = area.tributes(tribute.game_id.unwrap()).iter()
        .map(|t| Tribute::from(t.clone()))
        .filter(|t| t.is_alive())
        .filter(|t| t.id.unwrap() != tribute.id)
        .collect::<Vec<_>>();

    match tributes.len() {
        0 => { // there are no other targets
            match tribute.sanity {
                0..=9 => { // attempt suicide
                    println!("{}", GameMessage::TributeSuicide(Tribute::from(tribute.clone())));
                    Some(tribute.into())
                },
                10..=19 => match thread_rng().gen_bool(0.2) {
                    true => { // attempt suicide
                        println!("{}", GameMessage::TributeSuicide(Tribute::from(tribute.clone())));
                        Some(tribute.into())
                    },
                    false => None, // Attack no one
                },
                _ => None, // Attack no one
            }
        },
        _ => {
            let mut targets = tributes.clone();
            let enemy_targets: Vec<Tribute> = targets.iter().cloned()
                .filter(|t| t.district != tribute.district)
                .filter(|t| t.is_visible())
                .collect();

            match tribute.sanity {
                0..20 => (), // Sanity is low, target everyone
                _ => targets = enemy_targets.clone() // Sane enough not to attack district mate
            }

            match targets.len() {
                0 | 1 => Some(targets.first()?.clone()), // Easy choice
                _ => {
                    let mut rng = thread_rng();
                    Some(targets.choose(&mut rng)?.clone()) // Get a random enemy
                }
            }
        }
    }
}

impl Default for Tribute {
    fn default() -> Self {
        Self::new("Tribute".to_string(), None)
    }
}

use crate::models::{create_full_log, get_action, get_all_living_tributes, get_area, get_area_by_id, get_game_by_id, update_item, update_tribute, Action, Tribute as TributeModel};
impl From<TributeModel> for Tribute {
    fn from(tribute: models::tribute::Tribute) -> Self {
        use crate::areas::Area;
        use crate::tributes::actions::TributeAction;
        use crate::models::Area as AreaModel;

        let area = tribute.area().unwrap_or(AreaModel::from(Area::default()));

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
            area: Some(Area::from(area.clone())),
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
            bravery: self.bravery,
            loyalty: self.loyalty,
            speed: self.speed,
            intelligence: self.intelligence,
            persuasion: self.persuasion,
            luck: self.luck,
            strength: self.strength,
            defense: self.defense,
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
