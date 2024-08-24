use crate::schema::area;
use diesel::prelude::*;

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = area)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Area {
    pub id: i32,
    pub name: String,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = area)]
pub struct NewArea<'a> {
    pub name: &'a str,
}

pub fn create_area(conn: &mut PgConnection, name: &str) -> Area {
    let new_area = NewArea { name };

    diesel::insert_into(area::table)
        .values(&new_area)
        .returning(Area::as_returning())
        .get_result(conn)
        .expect("Error saving new area")
}

pub fn get_areas(conn: &mut PgConnection) -> Vec<Area> {
    area::table.load::<Area>(conn).expect("Error loading areas")
}

pub fn get_area(conn: &mut PgConnection, name: &str) -> Area {
    let area: Area = area::table
        .filter(area::name.ilike(name))
        .first::<Area>(conn)
        .expect("Error loading area")
        .into();
    area
}

pub fn get_area_by_id(conn: &mut PgConnection, id: Option<i32>) -> Option<Area> {
    if id.is_none() {
        return None;
    }
    let area: Area = area::table
        .find(id.unwrap())
        .first::<Area>(conn)
        .expect("Error loading area")
        .into();
    Some(area)
}
