use crate::schema::game;
use diesel::prelude::*;

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = game)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Game {
    pub id: i32,
    pub name: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = game)]
pub struct NewGame<'a> {
    pub name: &'a str,
}

impl NewGame<'_> {
    pub fn create(connection: &mut PgConnection, name: &str) -> Game {
        diesel::insert_into(game::table)
            .values(&NewGame { name })
            .get_result(connection)
            .expect("Error saving new game")
    }
}

pub fn get_game(connection: &mut PgConnection, name: &str) -> Game {
    game::table
        .filter(game::name.ilike(name))
        .first(connection)
        .expect("Error loading game")
}

fn generate_random_name() -> String {
    let wp_gen = witty_phrase_generator::WPGen::new();
    let name = wp_gen.generic(3, 1, Some(5), Some(25), None).expect("Couldn't generate name");
    let name = name[0].join("-");
    name.to_string()
}

pub fn create_game(connection: &mut PgConnection) -> Game {
    NewGame::create(connection, &generate_random_name())
}
