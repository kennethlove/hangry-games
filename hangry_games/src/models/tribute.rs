use super::get_area_by_id;
use crate::establish_connection;
use crate::events::TributeEvent;
use crate::models::{get_area, get_game_by_id, tribute_action, Action, Area, Game};
use crate::schema::tribute;
use crate::tributes::actions::{AttackOutcome, TributeAction};
use crate::tributes::actors::pick_target;
use crate::tributes::actors::{TravelResult, Tribute as TributeActor};
use crate::tributes::statuses::TributeStatus;
use diesel::prelude::*;
use rand::prelude::*;

#[derive(Queryable, Selectable, Debug, Clone, Associations)]
#[diesel(table_name = tribute)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(Area, foreign_key = area_id))]
#[diesel(belongs_to(Game, foreign_key = game_id))]
pub struct Tribute {
    pub id: i32,
    pub name: String,
    pub health: i32,
    pub sanity: i32,
    pub movement: i32,
    pub district: i32,
    pub area_id: Option<i32>,
    pub game_id: Option<i32>,
    pub day_killed: Option<i32>,
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
    pub killed_by: Option<String>,
    pub strength: Option<i32>,
    pub defense: Option<i32>,
    pub is_hidden: Option<bool>,
    pub dexterity: Option<i32>,
    pub status: String,
}

impl Tribute {
    pub fn set_area(&mut self, area: &Area) {
        let connection = &mut establish_connection();
        diesel::update(tribute::table.find(self.id))
            .set(tribute::area_id.eq(Some(area.id)))
            .execute(connection)
            .expect("Error updating tribute");
    }

    pub fn unset_area(&self) {
        let connection = &mut establish_connection();
        diesel::update(tribute::table.find(self.id))
            .set(tribute::area_id.eq(None::<i32>))
            .execute(connection)
            .expect("Error updating tribute");
    }

    pub fn area(&self) -> Option<Area> {
        get_area_by_id(self.area_id)
    }

    pub fn actions(&self) -> Vec<Action> {
        use crate::schema::action;
        use crate::schema::tribute_action;

        let connection = &mut establish_connection();
        tribute_action::table
            .inner_join(action::table)
            .filter(tribute_action::tribute_id.eq(self.id))
            .select(action::all_columns)
            .load::<Action>(connection)
            .expect("Error loading actions")
    }

    /// Get all the TributeActions for a Tribute.
    pub fn tribute_actions(&self) -> Vec<crate::models::TributeAction> {
        tribute_action::TributeAction::get_all_for_tribute(self.id)
    }

    pub fn take_action(&self, action: &Action) {
        use crate::models::TributeAction;
        TributeAction::create(self.id, action.id, None);
    }

    pub fn set_game(&mut self, game: &Game) {
        let connection = &mut establish_connection();
        diesel::update(tribute::table.find(self.id))
            .set(tribute::game_id.eq(Some(game.id)))
            .execute(connection)
            .expect("Error updating tribute");
    }

    pub fn unset_game(&mut self) {
        let connection = &mut establish_connection();
        diesel::update(tribute::table.find(self.id))
            .set(tribute::game_id.eq(None::<i32>))
            .execute(connection)
            .expect("Error updating tribute");
    }

    pub fn try_set_game(&mut self, game: &Game) -> Result<(), String> {
        if self.game_id.is_some() {
            return Err("Tribute already has a game".to_string());
        }
        if game.tributes().len() >= 24 {
            return Err("Game is full".to_string());
        }
        self.set_game(game);
        Ok(())
    }

    pub fn dies(&self) {
        let connection = &mut establish_connection();
        let game = get_game_by_id(self.game_id.unwrap()).unwrap();
        let game_day = game.day.unwrap();

        diesel::update(tribute::table.find(self.id))
            .set((
                tribute::status.eq(TributeStatus::Dead.to_string()),
                tribute::health.eq(0),
                tribute::day_killed.eq(game_day),
            ))
            .execute(connection)
            .expect("Error killing tribute");
    }

    pub fn do_day(&mut self) -> Self {
        if self.health == 0 {
            println!("😱 {} is dead!", self.name);
            self.status = TributeStatus::RecentlyDead.to_string();
            return self.clone();
        }

        let area = self.area().unwrap();

        // Create Tribute struct
        let mut tribute = TributeActor::from(self.clone());

        // Get game
        let game = get_game_by_id(self.game_id.unwrap()).unwrap();

        // Get Brain struct
        let mut brain = tribute.brain.clone();

        // Day 1, prefer to move
        if game.day == Some(1) {
            brain.set_preferred_action(TributeAction::Move(None), 0.5);
        }

        // Day 3 is the Feast, prefer move to the Cornucopia
        if game.day == Some(3) {
            brain.set_preferred_action(
                TributeAction::Move(
                    Some(crate::areas::Area::Cornucopia.to_string())
                ),
               0.75
            );
        }

        // Get nearby targets
        let nearby_targets = Self::get_nearby_targets(area.clone(), self.game_id.unwrap());
        let nearby_targets: Vec<TributeActor> = nearby_targets.iter()
            .filter(|t| t.id != self.id)
            .map(|t| TributeActor::from(t.clone()))
            .collect::<Vec<_>>();

        // Collect the closed areas
        let closed_areas: Vec<crate::areas::Area> = game.closed_areas.unwrap_or(Vec::<Option<i32>>::new()).iter()
            .map(|id| get_area_by_id(*id).unwrap())
            .map(|a| crate::areas::Area::from(a))
            .collect::<Vec<_>>();

        // Decide the next logical action
        brain.act(&mut tribute, nearby_targets.len(), closed_areas.clone());
        let mut target = None;

        match brain.last_action() {
            TributeAction::Move(area) => {
                if area.is_some() {
                    target = Some(area.clone().unwrap().as_str().to_string());
                    move_tribute(tribute.into(), Some(area.unwrap().as_str().to_string()));
                } else {
                    move_tribute(tribute.into(), None);
                }
            }
            TributeAction::Hide => {
                hide_tribute(Tribute::from(tribute));
            }
            TributeAction::Rest | TributeAction::None => {
                rest_tribute(tribute.into());
            }
            TributeAction::Attack => {
                if let Some(victim) = pick_target(self.clone(), nearby_targets) {
                    let victim = Tribute::from(victim);
                    target = Some(victim.name.clone());
                    attack_target(self.clone(), victim.clone());
                }
            }
            _ => {
                println!("{} does nothing", self.name);
            }
        }

        // Find the action model instance
        let last_action = crate::models::action::get_action(brain.last_action().as_str());

        // Connect Tribute to Action
        tribute_action::take_action(&self.clone(), &last_action, target);

        self.clone()
    }

    pub fn do_night(&mut self) -> Self {
        if self.health == 0 {
            println!("😱 {} is dead", self.name);
            self.status = TributeStatus::RecentlyDead.to_string();
            return self.clone();
        }

        // Create Tribute struct
        let mut tribute = TributeActor::from(self.clone());

        let game = get_game_by_id(self.game_id.unwrap()).expect("Couldn't get game");
        let area = get_area_by_id(self.area_id).expect("Couldn't get area");

        // Collect the closed areas
        let closed_areas: Vec<crate::areas::Area> = game.closed_areas.unwrap_or(Vec::<Option<i32>>::new()).iter()
            .map(|id| get_area_by_id(*id).unwrap())
            .map(|a| crate::areas::Area::from(a))
            .collect::<Vec<_>>();

        if closed_areas.contains(&crate::areas::Area::from(area.clone())) {
            tribute.takes_physical_damage(100);
            tribute.killed_by = Some(format!("{} didn't escape the closed area", tribute.name));
            println!("🌋 {} waited too long in a closed area", tribute.name);
            tribute.status = TributeStatus::RecentlyDead;
            let t = Self::from(tribute);
            return t
        }

        // Get nearby tributes and targets
        let nearby_targets = Self::get_nearby_targets(area.clone(), self.game_id.unwrap());
        let nearby_targets: Vec<TributeActor> = nearby_targets.iter()
            .filter(|t| t.id != self.id)
            .map(|t| TributeActor::from(t.clone()))
            .collect::<Vec<_>>();

        // Get Brain struct
        let mut brain = tribute.brain.clone();

        // Decide the next logical action
        brain.act(&mut tribute, nearby_targets.len(), closed_areas.clone());
        let mut target = None;

        match brain.last_action() {
            TributeAction::Move(area) => {
                if area.is_some() {
                    target = Some(area.clone().unwrap().as_str().to_string());
                    move_tribute(tribute.into(), area);
                } else {
                    move_tribute(tribute.into(), None);
                }
            }
            TributeAction::Attack => {
                // How brave does the tribute feel at night?
                let mut rng = thread_rng();
                let bravery = self.bravery.unwrap();
                let bravado: u32 = rng.gen_range(0..=100);
                let brave_enough = bravado + bravery as u32 > 50;

                if let Some(victim) = pick_target(self.clone(), nearby_targets) {
                    let victim = Tribute::from(victim);
                    if brave_enough == true {
                        target = Some(victim.name.clone());
                        attack_target(self.clone(), victim.clone());
                    } else {
                        println!("😨 {} is too scared to attack {}", self.name, victim.name);
                    }
                }
            }
            TributeAction::Hide => {
                hide_tribute(Tribute::from(tribute));
            }
            _ => {
                println!("{} does nothing", self.name);
                rest_tribute(tribute.into());
            }
        }

        // Find the action model instance
        let last_action = crate::models::action::get_action(brain.last_action().as_str());

        // Connect Tribute to Action
        tribute_action::take_action(&self.clone(), &last_action, target);

        self.clone()
    }

    fn get_nearby_tributes(area: Area, game_id: i32) -> Vec<TributeActor> {
        // Get nearby tributes
        let area_tributes = area.tributes(game_id);
        let living_tributes = area_tributes.iter()
            .filter(|t|
                t.status != TributeStatus::RecentlyDead.to_string() &&
                t.status != TributeStatus::Dead.to_string() &&
                t.health > 0
            );
        let nearby_tributes: Vec<TributeActor> = living_tributes.clone()
            .map(|t| TributeActor::from(t.clone()))
            .filter(|t| t.is_visible())
            .collect();
        nearby_tributes
    }

    fn get_nearby_targets(area: Area, game_id: i32) -> Vec<Tribute> {
        let nearby_tributes = Self::get_nearby_tributes(area, game_id);
        let nearby_targets: Vec<Tribute> = nearby_tributes.clone().into_iter().map(|t| Tribute::from(t)).collect();
        nearby_targets
    }

    pub fn is_alive(&self) -> bool {
        match self.status.as_str() {
            "Dead" | "RecentlyDead" => false,
            _ => true
        }
    }
}

fn rest_tribute(tribute: Tribute) {
    let mut tribute = TributeActor::from(tribute);

    // Long rest the tribute
    tribute.long_rests();

    update_tribute(tribute.id.unwrap(), Tribute::from(tribute.clone()));
}


fn move_tribute(tribute: Tribute, area: Option<String>) {
    let mut tribute = TributeActor::from(tribute);
    let game = get_game_by_id(tribute.game_id.unwrap()).unwrap();

    let closed_areas: Vec<crate::areas::Area> = game.closed_areas.clone().unwrap_or(vec![]).iter()
        .map(|id| get_area_by_id(*id))
        .map(|a| a.unwrap())
        .map(crate::areas::Area::from)
        .collect();
    match &tribute.travels(closed_areas.clone(), area) {
        TravelResult::Success(area) => {
            tribute.moves();
            tribute.changes_area(area.clone());
        }
        TravelResult::Failure => {
            tribute.short_rests();
        }
    }

    let tribute_instance = Tribute::from(tribute.clone());
    // save tribute_instance
    update_tribute(tribute.id.unwrap(), tribute_instance.clone());
}

fn hide_tribute(tribute: Tribute) {
    let mut hidden_tribute = TributeActor::from(tribute.clone());
    hidden_tribute.hides();
    hidden_tribute.short_rests();

    update_tribute(tribute.id, Tribute::from(hidden_tribute));
}

pub fn suffer_tribute(tribute: Tribute) -> Tribute {
    let mut tribute = TributeActor::from(tribute);
    tribute.suffers();

    let tribute = Tribute::from(tribute);
    update_tribute(tribute.id, tribute.clone());
    tribute
}

pub fn process_tribute_status(tribute: Tribute) -> Tribute {
    let mut tribute = TributeActor::from(tribute);
    tribute.process_status();

    let tribute = Tribute::from(tribute);
    update_tribute(tribute.id, tribute.clone());
    tribute
}

pub fn handle_tribute_event(tribute: Tribute) -> Tribute {
    let mut tribute = TributeActor::from(tribute);
    let event = TributeEvent::random();
    tribute.handle_event(event.clone());

    let tribute = Tribute::from(tribute);
    update_tribute(tribute.id, tribute.clone());
    tribute
}

impl From<crate::tributes::actors::Tribute> for Tribute {
    fn from(tribute: crate::tributes::actors::Tribute) -> Self {
        let current_tribute = get_tribute(&tribute.name);
        let area = get_area(tribute.area.unwrap().as_str());
        let game_id = current_tribute.game_id.unwrap();

        let out_tribute = Tribute {
            id: current_tribute.id,
            name: tribute.name,
            health: tribute.health,
            sanity: tribute.sanity,
            movement: tribute.movement,
            district: tribute.district,
            area_id: Some(area.id),
            game_id: Some(game_id),
            day_killed: tribute.day_killed,
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
            killed_by: tribute.killed_by,
            strength: tribute.strength,
            defense: tribute.defense,
            is_hidden: tribute.is_hidden,
            dexterity: tribute.dexterity,
            status: tribute.status.to_string(),
        };
        out_tribute
    }
}

#[derive(Insertable, Debug)]
#[diesel(table_name = tribute)]
pub struct NewTribute {
    pub name: String,
    pub district: i32,
    pub bravery: Option<i32>,
    pub loyalty: Option<i32>,
    pub speed: Option<i32>,
    pub intelligence: Option<i32>,
    pub persuasion: Option<i32>,
    pub luck: Option<i32>,
    pub strength: Option<i32>,
    pub defense: Option<i32>,
    pub dexterity: Option<i32>,
    pub status: String,
}

impl From<crate::tributes::actors::Tribute> for NewTribute {
    fn from(tribute: crate::tributes::actors::Tribute) -> Self {
        let out_tribute = NewTribute {
            name: tribute.name.clone(),
            district: tribute.district,
            bravery: tribute.bravery,
            loyalty: tribute.loyalty,
            speed: tribute.speed,
            intelligence: tribute.intelligence,
            persuasion: tribute.persuasion,
            luck: tribute.luck,
            strength: tribute.strength,
            defense: tribute.defense,
            dexterity: tribute.dexterity,
            status: tribute.status.to_string(),
        };
        out_tribute
    }
}

#[derive(Insertable, Debug, AsChangeset)]
#[diesel(table_name = tribute)]
pub struct UpdateTribute {
    pub id: i32,
    pub name: String,
    pub district: i32,
    pub health: i32,
    pub sanity: i32,
    pub movement: i32,
    pub area_id: Option<i32>,
    pub game_id: i32,
    pub day_killed: Option<i32>,
    pub kills: Option<i32>,
    pub wins: Option<i32>,
    pub defeats: Option<i32>,
    pub draws: Option<i32>,
    pub games: Option<i32>,
    pub killed_by: Option<String>,
    pub is_hidden: Option<bool>,
    pub dexterity: Option<i32>,
    pub status: String,
}

pub fn create_tribute(name: &str) -> Tribute {
    use crate::schema::tribute;
    let conn = &mut establish_connection();

    let district = tribute::table
        .select(diesel::dsl::count_star())
        .count()
        .into_boxed()
        .get_result::<i64>(conn)
        .expect("Error counting tributes");
    let district = district as i32;
    let district = district % 12 + 1;

    let tribute = TributeActor::new(name.to_string(), Some(district));
    let new_tribute = NewTribute::from(tribute);

    diesel::insert_into(tribute::table)
        .values(&new_tribute)
        .returning(Tribute::as_returning())
        .get_result(conn)
        .expect("Error saving new tribute")
}

pub fn get_all_tributes() -> Vec<Tribute> {
    let conn = &mut establish_connection();
    use crate::schema::tribute;
    tribute::table
        .select(tribute::all_columns)
        .load::<Tribute>(conn)
        .expect("Error loading tributes")
}

pub fn place_tribute_in_area(tribute: &Tribute, area: &Area) {
    let conn = &mut establish_connection();
    diesel::update(tribute::table.find(tribute.id))
        .set(tribute::area_id.eq(Some(area.id)))
        .execute(conn)
        .expect("Error updating tribute");
}

pub fn get_tribute(name: &str) -> Tribute {
    use crate::schema::tribute;
    let conn = &mut establish_connection();
    let tribute: Tribute = tribute::table
        .filter(tribute::name.ilike(name))
        .first::<Tribute>(conn)
        .expect("Error loading tribute");
    tribute
}

fn attack_target(attacker: Tribute, victim: Tribute) {
    let mut attacker = TributeActor::from(attacker.clone());
    let mut victim = TributeActor::from(victim.clone());

    // Mutates attacker and victim
    TributeActor::attacks(&mut attacker, &mut victim);

    let attacker = Tribute::from(attacker);
    let victim = Tribute::from(victim);
    update_tribute(attacker.id, attacker);
    update_tribute(victim.id, victim);
}

pub fn update_tribute(tribute_id: i32, tribute: Tribute) {
    let conn = &mut establish_connection();
    let update_tribute = UpdateTribute {
        id: tribute_id,
        name: tribute.name,
        district: tribute.district,
        health: tribute.health,
        sanity: tribute.sanity,
        movement: tribute.movement,
        area_id: tribute.area_id,
        game_id: tribute.game_id.unwrap(),
        day_killed: tribute.day_killed,
        killed_by: tribute.killed_by,
        kills: tribute.kills,
        wins: tribute.wins,
        defeats: tribute.defeats,
        draws: tribute.draws,
        games: tribute.games,
        is_hidden: tribute.is_hidden,
        dexterity: tribute.dexterity,
        status: tribute.status,
    };
    diesel::update(tribute::table.find(tribute_id))
        .set(&update_tribute)
        .execute(conn)
        .expect("Error updating tribute");
}
