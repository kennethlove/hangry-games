use super::get_area_by_id;
use crate::establish_connection;
use crate::models::{get_area, get_game_by_id, tribute_action, Action, Area, Game};
use crate::schema::tribute;
use crate::tributes::actors::Tribute as TributeActor;
use crate::tributes::statuses::TributeStatus;
use diesel::prelude::*;

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
    pub avatar: Option<String>,
    pub real_name: Option<String>,
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

    pub fn update(&self, tribute: UpdateTribute) {
        let connection = &mut establish_connection();
        diesel::update(tribute::table.find(self.id))
            .set(&tribute)
            .execute(connection)
            .expect("Error updating tribute");
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

    pub fn takes_item(&self, item_id: i32) {
        use crate::schema::item;
        let connection = &mut establish_connection();

        diesel::update(item::table.find(item_id))
            .set((
                item::tribute_id.eq(self.id),
                item::area_id.eq(None::<i32>),
            ))
            .execute(connection)
            .expect("Error giving item to tribute");
    }

    pub fn uses_consumable(&self, item_id: i32) {
        use crate::schema::item;
        let connection = &mut establish_connection();

        diesel::delete(item::table.find(item_id))
            .execute(connection)
            .expect("Error using consumable");
    }

    pub fn delete(id: i32) {
        let connection = &mut establish_connection();
        use crate::schema::log_entry;
        use crate::schema::tribute_action;
        use crate::schema::tribute;

        diesel::delete(log_entry::table.filter(log_entry::tribute_id.eq(id)))
            .execute(connection)
            .expect("Error deleting log entries");

        diesel::delete(tribute_action::table
            .filter(tribute_action::tribute_id.eq(id)))
            .execute(connection)
            .expect("Error deleting tribute actions");

        diesel::delete(tribute::table.find(id))
            .execute(connection)
            .expect("Error deleting tribute");
    }
}

impl From<crate::tributes::actors::Tribute> for Tribute {
    fn from(tribute: crate::tributes::actors::Tribute) -> Self {
        let current_tribute = get_tribute_by_id(tribute.id.expect("Tribute has no ID"));
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
            avatar: tribute.avatar,
            real_name: tribute.real_name,
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
    pub avatar: Option<String>,
    pub real_name: Option<String>,
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
            avatar: tribute.avatar,
            real_name: tribute.real_name,
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
    pub bravery: Option<i32>,
    pub loyalty: Option<i32>,
    pub speed: Option<i32>,
    pub intelligence: Option<i32>,
    pub persuasion: Option<i32>,
    pub luck: Option<i32>,
    pub strength: Option<i32>,
    pub defense: Option<i32>,
    pub killed_by: Option<String>,
    pub is_hidden: Option<bool>,
    pub dexterity: Option<i32>,
    pub status: String,
    pub avatar: Option<String>,
    pub real_name: Option<String>,
}

#[derive(Insertable, Debug, AsChangeset)]
#[diesel(table_name = tribute)]
pub struct EditTribute {
    pub name: String,
    pub district: i32,
    pub avatar: Option<String>,
    pub real_name: Option<String>,
}

pub fn edit_tribute(tribute_id: i32, tribute: EditTribute) {
    let conn = &mut establish_connection();
    diesel::update(tribute::table.find(tribute_id))
        .set(&tribute)
        .execute(conn)
        .expect("Error updating tribute");
}

pub fn create_tribute(name: &str, avatar: Option<String>) -> Tribute {
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

    let tribute = TributeActor::new(name.to_string(), Some(district), avatar);
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

pub fn get_tribute_by_id(tribute_id: i32) -> Tribute {
    use crate::schema::tribute;
    let conn = &mut establish_connection();
    let tribute: Tribute = tribute::table
        .find(tribute_id)
        .first::<Tribute>(conn)
        .expect("Error loading tribute");
    tribute
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
        status: tribute.status,
        avatar: tribute.avatar,
        real_name: tribute.real_name,
    };
    diesel::update(tribute::table.find(tribute_id))
        .set(&update_tribute)
        .execute(conn)
        .expect("Error updating tribute");
}
