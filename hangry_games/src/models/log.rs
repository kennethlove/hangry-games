use crate::{establish_connection, models};
use crate::schema::log_entry;
use diesel::prelude::*;
use models::get_game_by_id;

#[derive(Queryable, Selectable, Debug, Associations)]
#[diesel(table_name = log_entry)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(models::TributeAction, foreign_key = tribute_action_id))]
#[diesel(belongs_to(models::Area, foreign_key = area_id))]
#[diesel(belongs_to(models::Game, foreign_key = game_id))]
pub struct LogEntry {
    pub id: i32,
    pub created_at: chrono::NaiveDateTime,
    pub game_id: i32,
    pub day: i32,
    pub message: String,

    // optional fields for further linking
    pub tribute_action_id: Option<i32>, // Action: Move, Rest, etc
    pub area_id: Option<i32>, // Area the action originates
}

#[derive(Insertable, Debug)]
#[diesel(table_name = log_entry)]
pub struct NewLogEntry {
    pub game_id: i32,
    pub day: i32,
    pub message: String,
    pub tribute_action_id: Option<i32>,
    pub area_id: Option<i32>,
}

impl LogEntry {
    pub fn create(game_id: i32, message: String) -> LogEntry {
        let connection = &mut establish_connection();
        let game = get_game_by_id(game_id).expect("Game not found");
        let new_log_entry = NewLogEntry {
            game_id: game.id,
            day: game.day.unwrap_or(0),
            message,
            tribute_action_id: None,
            area_id: None,
        };

        diesel::insert_into(log_entry::table)
            .values(&new_log_entry)
            .returning(LogEntry::as_returning())
            .get_result(connection)
            .expect("Error saving new log entry")
    }

    pub fn create_full_log(game_id: i32, message: String, tribute_action_id: Option<i32>, area_id: Option<i32>) -> LogEntry {
        let connection = &mut establish_connection();
        let game = get_game_by_id(game_id).expect("Game not found");
        let new_log_entry = NewLogEntry {
            game_id: game.id,
            day: game.day.unwrap_or(0),
            message,
            tribute_action_id,
            area_id,
        };

        diesel::insert_into(log_entry::table)
            .values(&new_log_entry)
            .returning(LogEntry::as_returning())
            .get_result(connection)
            .expect("Error saving new log entry")
    }
}

pub fn get_log_entry_by_id(id: i32) -> Option<LogEntry> {
    let connection = &mut establish_connection();
    log_entry::table.find(id)
        .first(connection)
        .optional()
        .expect("Error loading log entry")
}

