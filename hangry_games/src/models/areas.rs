use diesel::prelude::*;
use crate::schema::areas;

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = areas)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Area {
    pub id: i32,
    pub name: String,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = areas)]
pub struct NewArea<'a> {
    pub name: &'a str,
}

pub fn create_area(conn: &mut PgConnection, name: &str) -> Area {
    let new_area = NewArea { name };

    diesel::insert_into(areas::table)
        .values(&new_area)
        .returning(Area::as_returning())
        .get_result(conn)
        .expect("Error saving new area")
}

pub fn get_areas(conn: &mut PgConnection) -> Vec<Area> {
    areas::table
        .load::<Area>(conn)
        .expect("Error loading areas")
}