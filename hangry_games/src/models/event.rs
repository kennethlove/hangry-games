use crate::establish_connection;
use crate::models::{Area, Game};
use crate::schema::area_event;
use diesel::prelude::*;

#[derive(Queryable, Selectable, Debug, Associations, Clone)]
#[diesel(table_name = area_event)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(Area, foreign_key = area_id))]
#[diesel(belongs_to(Game, foreign_key = game_id))]
pub struct AreaEvent {
    pub id: i32,
    pub name: String,
    pub area_id: i32,
    pub game_id: i32,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = area_event)]
pub struct NewAreaEvent {
    pub name: String,
    pub area_id: i32,
    pub game_id: i32,
}

impl AreaEvent {
    pub fn create(name: String, area_id: i32, game_id: i32) -> AreaEvent {
        let connection = &mut establish_connection();
        let new_area_event = NewAreaEvent { name, area_id, game_id };

        diesel::insert_into(area_event::table)
            .values(&new_area_event)
            .returning(AreaEvent::as_returning())
            .get_result(connection)
            .expect("Error saving new area event")
    }

    pub fn get_all_for_area(area_id: i32) -> Vec<AreaEvent> {
        let connection = &mut establish_connection();
        area_event::table
            .filter(area_event::area_id.eq(area_id))
            .select(area_event::all_columns)
            .load(connection)
            .expect("Error loading area events")
    }

    pub fn get_all_for_game(game_id: i32) -> Vec<AreaEvent> {
        let connection = &mut establish_connection();
        area_event::table
            .filter(area_event::game_id.eq(game_id))
            .select(area_event::all_columns)
            .load(connection)
            .expect("Error loading area events")
    }
}

pub fn delete_game_area_events(game_id: i32) {
    let connection = &mut establish_connection();

    // Delete game
    diesel::delete(area_event::table)
        .filter(area_event::game_id.eq(game_id))
        .execute(connection)
        .expect("Error deleting area events");
}