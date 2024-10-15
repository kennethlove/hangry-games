use crate::schema::log_entry;
use crate::{establish_connection, models};
use diesel::prelude::*;
use models::get_game_by_id;
use crate::models::get_action;

#[derive(Queryable, Selectable, Debug, Associations)]
#[diesel(table_name = log_entry)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(models::TributeAction, foreign_key = tribute_action_id))]
#[diesel(belongs_to(models::Area, foreign_key = area_id))]
#[diesel(belongs_to(models::Game, foreign_key = game_id))]
#[diesel(belongs_to(models::Tribute, foreign_key = tribute_id))]
pub struct LogEntry {
    pub id: i32,
    pub created_at: chrono::NaiveDateTime,
    pub game_id: i32,
    pub day: i32,
    pub message: String,

    // optional fields for further linking
    pub tribute_action_id: Option<i32>, // Action: Move, Rest, etc
    pub area_id: Option<i32>, // Area the action originates
    pub tribute_id: Option<i32>, // Tribute performing the action
    pub action_target_type: Option<String>, // Type of target of the action
    pub action_target_id: Option<i32>, // Target of the action
}

#[derive(Insertable, Debug)]
#[diesel(table_name = log_entry)]
pub struct NewLogEntry {
    pub game_id: i32,
    pub day: i32,
    pub message: String,
    pub tribute_action_id: Option<i32>,
    pub area_id: Option<i32>,
    pub tribute_id: Option<i32>,
    pub action_target_type: Option<String>,
    pub action_target_id: Option<i32>,
}

impl Default for NewLogEntry {
    fn default() -> Self {
        NewLogEntry {
            game_id: 0,
            day: 0,
            message: "".to_string(),
            tribute_action_id: None,
            area_id: None,
            tribute_id: None,
            action_target_type: None,
            action_target_id: None,
        }
    }
}

impl LogEntry {
    pub fn create(game_id: i32, message: String) -> LogEntry {
        let connection = &mut establish_connection();
        let selected_game = get_game_by_id(game_id).expect("Game not found");
        let new_log_entry = NewLogEntry {
            game_id: selected_game.id,
            day: selected_game.day.unwrap_or(0),
            message,
            tribute_action_id: None,
            area_id: None,
            tribute_id: None,
            action_target_type: None,
            action_target_id: None,
        };

        diesel::insert_into(log_entry::table)
            .values(&new_log_entry)
            .returning(LogEntry::as_returning())
            .get_result(connection)
            .expect("Error saving new log entry")
    }
}

pub fn create_full_log(
    game_id: i32,
    message: String,
    tribute_action_id: Option<i32>,
    area_id: Option<i32>,
    tribute_id: Option<i32>,
    action_target_type: Option<String>,
    action_target_id: Option<i32>,
) -> LogEntry {
    if action_target_type.is_none() && action_target_id.is_some() {
        panic!("Action target type must be provided if action target id is provided");
    } else if action_target_type.is_some() && action_target_id.is_none() {
        panic!("Action target id must be provided if action target type is provided");
    }

    let connection = &mut establish_connection();
    let selected_game = get_game_by_id(game_id).expect("Game not found");
    let new_log_entry = NewLogEntry {
        game_id: selected_game.id,
        day: selected_game.day.unwrap_or(0),
        message,
        tribute_action_id,
        area_id,
        tribute_id,
        action_target_type,
        action_target_id,
    };

    diesel::insert_into(log_entry::table)
        .values(&new_log_entry)
        .returning(LogEntry::as_returning())
        .get_result(connection)
        .expect("Error saving new log entry")
}

pub fn get_log_entry_by_id(id: i32) -> Option<LogEntry> {
    let connection = &mut establish_connection();
    log_entry::table.find(id)
        .select(log_entry::all_columns)
        .first(connection)
        .optional()
        .expect("Error loading log entry")
}

pub fn get_logs_for_game(id: i32) -> Vec<LogEntry> {
    let connection = &mut establish_connection();
    log_entry::table
        .select(log_entry::all_columns)
        .filter(log_entry::game_id.eq(id))
        .load(connection)
        .expect("Error loading log entries")
}

pub fn get_logs_for_game_day(id: i32, day: i32) -> Vec<LogEntry> {
    let connection = &mut establish_connection();
    log_entry::table
        .select(log_entry::all_columns)
        .filter(log_entry::game_id.eq(id))
        .filter(log_entry::day.eq(day))
        .load(connection)
        .expect("Error loading log entries")
}

pub fn get_logs_for_tribute(id: i32) -> Vec<LogEntry> {
    let connection = &mut establish_connection();
    log_entry::table
        .filter(log_entry::tribute_id.eq(id))
        .select(log_entry::all_columns)
        .load(connection)
        .expect("Error loading log entries")
}