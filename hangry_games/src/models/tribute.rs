use crate::establish_connection;
use crate::models::{get_area, get_game_by_id, tribute_action, Action, Area, Game};
use crate::schema::tribute;
use crate::tributes::actors::pick_target;
use crate::areas::Area as AreaStruct;
use diesel::prelude::*;
use fake::faker::name::raw::*;
use fake::locales::*;
use fake::Fake;
use rand::seq::SliceRandom;
use super::get_area_by_id;

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
    pub is_alive: bool,
    pub district: i32,
    pub area_id: Option<i32>,
    pub game_id: Option<i32>,
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

    pub fn take_action(&self, action: &Action) {
        use crate::models::TributeAction;
        TributeAction::create(self.id, action.id);
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
        diesel::update(tribute::table.find(self.id))
            .set((
                tribute::is_alive.eq(false),
                tribute::health.eq(0),
            ))
            .execute(connection)
            .expect("Error killing tribute");
    }

    pub fn do_day(&mut self) -> Self {
        use crate::tributes::actors::Tribute as TributeActor;
        use crate::tributes::actions::TributeAction;

        if self.is_alive == false || self.health == 0 {
            println!("Tribute is dead");
            return self.clone();
        }

        let connection = &mut establish_connection();
        let area = self.area().unwrap();

        // Create Tribute struct
        let tribute = TributeActor::from(self.clone());

        // Get Brain struct
        let mut brain = tribute.brain.clone();

        // Get nearby tributes
        let area_tributes = area.tributes(self.game_id.unwrap());
        let living_tributes = area_tributes.iter().filter(|t| t.is_alive && t.health > 0 && t.game_id == self.game_id);
        let nearby_tributes: Vec<TributeActor> = living_tributes.clone().map(|t| TributeActor::from(t.clone())).collect();
        let nearby_targets: Vec<Tribute> = living_tributes.into_iter().cloned().collect();

        // If the tribute is in a closed area, move them.
        let game = get_game_by_id(self.game_id.unwrap());
        if let Ok(game) = game {
            if game.closed_areas.unwrap_or(Vec::<Option<i32>>::new()).contains(&Some(area.id)) {
                self.move_tribute(tribute.clone());
                return self.clone()
            }
        }

        // Decide the next logical action
        brain.act(&tribute, nearby_tributes.clone());

        match brain.last_action() {
            TributeAction::Move => {
                self.move_tribute(tribute);
            }
            TributeAction::Rest | TributeAction::Hide | TributeAction::Idle => {
                self.rest_tribute(connection);
            }
            TributeAction::Attack => {
                let nearby_targets: Vec<TributeActor> = nearby_targets.iter()
                    .filter(|t| t.id != self.id)
                    .map(
                        |t| TributeActor::from(t.clone())
                    ).collect();
                if let Some(target) = pick_target(self.clone(), nearby_targets) {
                    let target = Tribute::from(target);
                    attack_target(self.clone(), target.clone());
                }
            }
            _ => {
                // Do nothing
            }
        }

        // Find the action model instance
        let last_action = crate::models::action::get_action(brain.last_action().as_str());

        // Connect Tribute to Action
        tribute_action::take_action(&self.clone(), &last_action);
        self.clone()
    }

    // TODO: Extract from impl
    fn rest_tribute(&mut self, connection: &mut PgConnection) {
        // Rest the tribute
        self.health = std::cmp::min(self.health + 50, 100);
        self.sanity = std::cmp::min(self.sanity + 50, 100);
        self.movement = std::cmp::min(self.movement + 25, 100);

        diesel::update(tribute::table.find(self.id))
            .set((
                tribute::health.eq(self.health),
                tribute::sanity.eq(self.sanity),
                tribute::movement.eq(self.movement),
            ))
            .execute(connection)
            .expect("Error resting tribute");
        println!("{} rests", self.name);
    }

    // TODO: Extract from impl
    fn move_tribute(&self, mut tribute: crate::tributes::actors::Tribute) {
        if tribute.movement < 25 {
            println!("{} is too tired to move", tribute.name);
            // TODO: Add a rest action
            return;
        }
        let connection = &mut establish_connection();

        // This next chunk feels gross but I don't know a better way
        let game = get_game_by_id(self.game_id.unwrap()).unwrap();
        let tribute_area = tribute.area.unwrap();
        let neighbors = tribute_area.neighbors();

        // Get a random neighbor that isn't the tribute's current area
        let random_neighbor = loop {
            let area = neighbors.choose(&mut rand::thread_rng()).unwrap();
            let area = get_area(area.as_str());

            // Same area check
            if area.name == tribute_area.as_str() {
                continue;
            }
            // Closed area check
            if game.closed_areas.clone().unwrap_or(vec![]).contains(&Some(area.id)) {
                continue;
            }
            break area;
        };

        tribute.area = Some(AreaStruct::from(random_neighbor.clone()));
        tribute.moves(50);

        let tribute_instance = Tribute::from(tribute.clone());
        // save tribute_instance
        diesel::update(tribute::table.find(self.id))
            .set((
                tribute::area_id.eq(tribute_instance.area_id),
                tribute::movement.eq(tribute_instance.movement),
            ))
            .execute(connection)
            .expect("Error moving tribute");
        println!("{} moves from {} to {}", tribute.name, tribute_area.as_str(), &random_neighbor.name.as_str());
    }
}

impl From<crate::tributes::actors::Tribute> for Tribute {
    fn from(tribute: crate::tributes::actors::Tribute) -> Self {

        let current_tribute = get_tribute(&tribute.name);
        let area = crate::models::get_area(tribute.area.unwrap().as_str());
        let game_id = current_tribute.game_id.unwrap();

        let out_tribute = Tribute {
            id: current_tribute.id,
            name: tribute.name,
            health: tribute.health as i32,
            sanity: tribute.sanity as i32,
            movement: tribute.movement as i32,
            is_alive: tribute.is_alive,
            district: tribute.district as i32,
            area_id: Some(area.id),
            game_id: Some(game_id),
        };
        out_tribute
    }
}

#[derive(Insertable, Debug)]
#[diesel(table_name = tribute)]
pub struct NewTribute<'a> {
    pub name: &'a str,
    pub district: i32,
}

#[derive(Insertable, Debug, AsChangeset)]
#[diesel(table_name = tribute)]
pub struct UpdateTribute {
    pub name: String,
    pub district: i32,
    pub health: i32,
    pub sanity: i32,
    pub movement: i32,
    pub is_alive: bool,
    pub area_id: Option<i32>,
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

    let new_tribute = NewTribute { name, district };

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

pub fn get_all_living_tributes(game: &Game) -> Vec<Tribute> {
    let conn = &mut establish_connection();
    use crate::schema::tribute;
    tribute::table
        .select(tribute::all_columns)
        .filter(tribute::game_id.eq(game.id))
        .filter(tribute::is_alive.eq(true))
        .load::<Tribute>(conn)
        .expect("Error loading tributes")
}

pub fn get_game_tributes(game: &Game) -> Vec<Tribute> {
    use crate::schema::tribute;
    let conn = &mut establish_connection();
    tribute::table
        .select(tribute::all_columns)
        .filter(tribute::game_id.eq(game.id))
        .load::<Tribute>(conn)
        .expect("Error loading tributes")
}

/// Fill the tribute table with up to 24 tributes.
/// Return the number of tributes created.
pub fn fill_tributes(game: Game) -> usize {
    let tributes = get_game_tributes(&game);
    let count = tributes.len();
    if count < 24 {
        for _ in count..24 {
            let name: String = Name(EN).fake();
            let mut tribute = create_tribute(&name);
            tribute.set_game(&game)
        }
    }
    24 - count
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
    use crate::tributes::actors::Tribute as TributeActor;
    use crate::tributes::actors::do_combat;

    let mut tribute = TributeActor::from(attacker.clone());
    let mut target = TributeActor::from(victim.clone());

    // Mutates tribute and target
    do_combat(&mut tribute, &mut target);

    let tribute = Tribute::from(tribute);
    let target = Tribute::from(target);
    update_tribute(attacker.id, tribute);
    update_tribute(victim.id, target);
}

fn update_tribute(tribute_id: i32, tribute: Tribute) {
    let conn = &mut establish_connection();
    let update_tribute = UpdateTribute {
        name: tribute.name,
        district: tribute.district,
        health: tribute.health,
        sanity: tribute.sanity,
        movement: tribute.movement,
        is_alive: tribute.is_alive,
        area_id: tribute.area_id,
    };
    diesel::update(tribute::table.find(tribute_id))
        .set(&update_tribute)
        .execute(conn)
        .expect("Error updating tribute");
}
