use crate::schema::area;
use diesel::prelude::*;
use crate::establish_connection;

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = area)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Area {
    pub id: i32,
    pub name: String,
}

impl Area {
    pub fn random() -> Area {
        let connection = &mut establish_connection();
        let areas = get_areas(connection);
        let random_index = rand::random::<usize>() % areas.len();
        Area { id: areas[random_index].id, name: areas[random_index].name.clone() }
    }

    /// Get all the tributes in an area.
    pub fn tributes(&self) -> Vec<crate::models::Tribute> {
        let connection = &mut establish_connection();
        let tributes = crate::models::get_all_tributes(connection);
        tributes.into_iter().filter(|t| t.area_id == Some(self.id)).collect()
    }
}

impl From<crate::areas::Area> for Area {
    fn from(area: crate::areas::Area) -> Self {
        let aa = get_area(&mut establish_connection(), &area.as_str());
        Area { id: aa.id, name: aa.name }
    }
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
