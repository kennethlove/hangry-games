use diesel::prelude::*;
use crate::models::Area;

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