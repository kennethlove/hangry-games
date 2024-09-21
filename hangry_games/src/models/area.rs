use crate::schema::area;
use diesel::prelude::*;
use crate::establish_connection;

#[derive(Queryable, Selectable, Debug, Clone, Eq, PartialEq)]
#[diesel(table_name = area)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Area {
    pub id: i32,
    pub name: String,
}

impl Area {
    pub fn random() -> Area {
        let chosen_area = crate::areas::Area::random();
        let area = get_area(&chosen_area.as_str());
        Area { id: area.id, name: area.name.clone() }
    }

    /// Get all the tributes in an area.
    pub fn tributes(&self, game_id: i32) -> Vec<crate::models::Tribute> {
        let tributes = crate::models::get_all_tributes();
        tributes.into_iter()
            .filter(|t| t.game_id == Some(game_id))
            .filter(|t| t.area_id == Some(self.id))
            .collect()
    }
}

impl From<crate::areas::Area> for Area {
    fn from(area: crate::areas::Area) -> Self {
        let aa = get_area(&area.as_str());
        Area { id: aa.id, name: aa.name }
    }
}

#[derive(Insertable, Debug)]
#[diesel(table_name = area)]
pub struct NewArea<'a> {
    pub name: &'a str,
}

pub fn create_area(name: &str) -> Area {
    let conn = &mut establish_connection();
    let new_area = NewArea { name };

    diesel::insert_into(area::table)
        .values(&new_area)
        .returning(Area::as_returning())
        .get_result(conn)
        .expect("Error saving new area")
}

pub fn get_areas() -> Vec<Area> {
    let conn = &mut establish_connection();
    area::table.load::<Area>(conn).expect("Error loading areas")
}

pub fn get_area(name: &str) -> Area {
    let conn = &mut establish_connection();
    let area: Area = area::table
        .filter(area::name.ilike(name))
        .first::<Area>(conn)
        .expect("Error loading area")
        .into();
    area
}

pub fn get_area_by_id(id: Option<i32>) -> Option<Area> {
    if id.is_none() {
        return None;
    }
    let conn = &mut establish_connection();
    let area: Area = area::table
        .find(id?)
        .first::<Area>(conn)
        .expect("Error loading area")
        .into();
    Some(area)
}
