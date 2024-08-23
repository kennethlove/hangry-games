use diesel::prelude::*;
use crate::schema::area;

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
    area::table
        .load::<Area>(conn)
        .expect("Error loading areas")
}

// I'm not sure if I need this function to convert the Area model to the Area enum
// or not, but I'll leave it in case I need a refresher elsewhere.
pub fn get_area(conn: &mut PgConnection, name: &str) -> crate::areas::Area {
    let area: Area = area::table
        .filter(area::name.ilike(name))
        .first::<Area>(conn)
        .expect("Error loading area").into();
    let area = crate::areas::Area::from(area);
    area
}