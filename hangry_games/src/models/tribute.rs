use crate::establish_connection;
use crate::models::{get_game, tribute_action, Action, Area, Game};
use crate::schema::tribute;
use diesel::prelude::*;
use fake::faker::name::raw::*;
use fake::locales::*;
use fake::Fake;

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
        let connection = &mut establish_connection();
        get_area_by_id(connection, self.area_id)
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
        use crate::tributes::actors::Tribute as ATribute;
        use crate::tributes::actions::TributeAction;

        if self.is_alive == false || self.health == 0 {
            dbg!("Tribute is dead");
            return self.clone();
        }

        let connection = &mut establish_connection();

        // Create Tribute struct
        let mut tribute = ATribute::from(self.clone());

        // Get Brain struct
        let mut brain = tribute.brain.clone();

        // Get nearby tributes
        let area_tributes = self.area().unwrap().tributes();
        let living_tributes = area_tributes.iter().filter(|t| t.is_alive && t.health > 0 && t.game_id == self.game_id);
        let nearby_tributes: Vec<_> = living_tributes.clone().map(|t| ATribute::from(t.clone())).collect();

        // Decide the next logical action
        brain.act(&tribute, nearby_tributes.clone());

        use rand::seq::SliceRandom;

        match brain.last_action() {
            TributeAction::Move => {
                dbg!("Moving");
                let current_area = tribute.area.unwrap();
                let random_neighbor = current_area.neighbors().choose(&mut rand::thread_rng()).unwrap().clone();

                tribute.area = Some(random_neighbor.clone());
                tribute.movement = tribute.movement.saturating_sub(50);
                println!("{} moves from {} to {}", tribute.name, current_area.as_str(), &random_neighbor.as_str());

                let tribute_instance = Tribute::from(tribute);
                // save tribute_instance
                diesel::update(tribute::table.find(self.id))
                    .set((
                        tribute::area_id.eq(tribute_instance.area_id),
                        tribute::movement.eq(tribute_instance.movement),
                    ))
                    .execute(connection)
                    .expect("Error moving tribute");
            }
            TributeAction::Rest | TributeAction::Hide | TributeAction::Idle => {
                dbg!("Resting");
                // Rest the tribute
                self.health = std::cmp::min(self.health + 50, 100);
                self.sanity = std::cmp::min(self.sanity + 50, 100);
                self.movement = std::cmp::min(self.movement + 50, 100);

                diesel::update(tribute::table.find(self.id))
                    .set((
                        tribute::health.eq(self.health),
                        tribute::sanity.eq(self.sanity),
                        tribute::movement.eq(self.movement),
                    ))
                    .execute(connection)
                    .expect("Error resting tribute");
            }
            TributeAction::Attack => {
                dbg!("Attacking");
                let mut victim;
                let success = rand::random::<f32>() < 0.5;

                // Am I alone?
                if nearby_tributes.len() == 1 {
                    // Am I sane?
                    println!("{} is alone", self.name);
                    if self.sanity > 10 {
                        println!("Decides not to live another day");
                        return self.clone();
                    }

                    // Suicide/self-harm
                    println!("Can't take it, unalives themself");
                    victim = self.clone();
                } else {
                    // I am NOT alone
                    let victims = living_tributes.clone().filter(|t| {
                            t.is_alive == true && t.health > 0 && t.id != self.id
                        })
                        .collect::<Vec<&Tribute>>();
                    victim = victims[0].clone();
                    if let Some(chosen_victim) = victims.choose(&mut rand::thread_rng()) {
                        victim = chosen_victim.clone().clone();
                    }
                }

                println!("{} attacks {}", self.name, victim.name);

                // Attack another tribute
                if success {
                    let victim_health = victim.health.saturating_sub(50);
                    let victim_sanity = victim.sanity.saturating_sub(30);
                    let victim_movement = victim.movement.saturating_sub(10);

                    // Injure the victim
                    diesel::update(tribute::table.find(victim.id))
                        .set((
                            tribute::health.eq(victim_health),
                            tribute::sanity.eq(victim_sanity),
                            tribute::movement.eq(victim_movement),
                        ))
                        .execute(connection)
                        .expect("Error attacking tribute");

                    // Stress the attacker
                    self.sanity = std::cmp::max(self.sanity - 20, 0);

                    diesel::update(tribute::table.find(self.id))
                        .set(tribute::sanity.eq(self.sanity))
                        .execute(connection)
                        .expect("Error stressing tribute");

                    println!("Attack succeeds");
                    println!("{} health {}", self.health, victim_health);
                    println!("{} sanity {}", self.sanity, victim_sanity);
                } else { println!("Attack fails"); }
            }
            _ => {
                // Do nothing
            }
        }

        // Find the action model instance
        let last_action = crate::models::action::get_action(connection, brain.last_action().as_str());

        // Connect Tribute to Action
        tribute_action::take_action(&self.clone(), &last_action);
        self.clone()
    }
}

impl From<crate::tributes::actors::Tribute> for Tribute {
    fn from(tribute: crate::tributes::actors::Tribute) -> Self {
        let connection = &mut establish_connection();

        let current_tribute = get_tribute(connection, &tribute.name);
        let area = crate::models::get_area(connection, tribute.area.unwrap().as_str());
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

pub fn create_tribute(conn: &mut PgConnection, name: &str) -> Tribute {
    use crate::schema::tribute;

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

pub fn get_all_tributes(conn: &mut PgConnection) -> Vec<Tribute> {
    use crate::schema::tribute;
    tribute::table
        .select(tribute::all_columns)
        .load::<Tribute>(conn)
        .expect("Error loading tributes")
}

pub fn get_all_living_tributes(conn: &mut PgConnection, game: &Game) -> Vec<Tribute> {
    use crate::schema::tribute;
    tribute::table
        .select(tribute::all_columns)
        .filter(tribute::game_id.eq(game.id))
        .filter(tribute::is_alive.eq(true))
        .load::<Tribute>(conn)
        .expect("Error loading tributes")
}

pub fn get_game_tributes(conn: &mut PgConnection, game: &Game) -> Vec<Tribute> {
    use crate::schema::tribute;
    tribute::table
        .select(tribute::all_columns)
        .filter(tribute::game_id.eq(game.id))
        .load::<Tribute>(conn)
        .expect("Error loading tributes")
}

/// Fill the tribute table with up to 24 tributes.
/// Return the number of tributes created.
pub fn fill_tributes(conn: &mut PgConnection, game: Game) -> usize {
    let tributes = get_game_tributes(conn, &game);
    let count = tributes.len();
    if count < 24 {
        for _ in count..24 {
            let name: String = Name(EN).fake();
            let mut tribute = create_tribute(conn, &name);
            tribute.set_game(&game)
        }
    }
    24 - count
}

pub fn place_tribute_in_area(conn: &mut PgConnection, tribute: &Tribute, area: &Area) {
    diesel::update(tribute::table.find(tribute.id))
        .set(tribute::area_id.eq(Some(area.id)))
        .execute(conn)
        .expect("Error updating tribute");
}

pub fn get_tribute(conn: &mut PgConnection, name: &str) -> Tribute {
    use crate::schema::tribute;
    let tribute: Tribute = tribute::table
        .filter(tribute::name.ilike(name))
        .first::<Tribute>(conn)
        .expect("Error loading tribute");
    tribute
}
