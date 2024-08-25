use crate::establish_connection;
use crate::models::{Action, Area};
use crate::schema::tribute;
use diesel::prelude::*;
use fake::faker::name::raw::*;
use fake::locales::*;
use fake::Fake;

use super::get_area_by_id;

#[derive(Queryable, Selectable, Debug, Associations)]
#[diesel(table_name = tribute)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(Area, foreign_key = area_id))]
pub struct Tribute {
    pub id: i32,
    pub name: String,
    pub health: i32,
    pub sanity: i32,
    pub movement: i32,
    pub is_alive: bool,
    pub district: i32,
    pub area_id: Option<i32>,
}

impl Tribute {
    pub fn set_area(&mut self, area: Area) {
        self.area_id = Some(area.id);
    }

    pub fn unset_area(&mut self) {
        self.area_id = None;
    }

    pub fn area(&mut self) -> Option<Area> {
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

pub fn get_tributes(conn: &mut PgConnection) -> Vec<Tribute> {
    use crate::schema::tribute;
    tribute::table
        .select(tribute::all_columns)
        .load::<Tribute>(conn)
        .expect("Error loading tributes")
}

/// Fill the tribute table with up to 24 tributes.
pub fn fill_tributes(conn: &mut PgConnection) {
    let tributes = get_tributes(conn);
    if tributes.len() < 24 {
        for _ in tributes.len()..24 {
            let name: String = Name(EN).fake();
            create_tribute(conn, &name);
        }
    }
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
