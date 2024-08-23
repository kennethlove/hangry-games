use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
use crate::models::{Area, NewArea};

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_area(conn: &mut PgConnection, name: &str) -> Area {
    use crate::schema::areas;

    let new_area = NewArea { name };

    diesel::insert_into(areas::table)
        .values(&new_area)
        .returning(Area::as_returning())
        .get_result(conn)
        .expect("Error saving new area")
}
