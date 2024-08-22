pub mod tributes;
pub mod area;

pub mod db;
pub mod cli;
pub mod models;
pub mod schema;

use diesel::sqlite::SqliteConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
use self::models::{NewArea, Area};

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url).unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

// pub fn create_area(conn: &mut SqliteConnection, name: &str) -> Area {
//     use crate::schema::areas;
//
//     let new_area = NewArea { name };
//
//     diesel::insert_into(areas::table)
//         .values(&new_area)
//         .returning(Area::as_returning())
//         .get_result(conn)
//         .expect("Error saving new area")
// }