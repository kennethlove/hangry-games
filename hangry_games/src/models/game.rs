use crate::schema::game;
use diesel::prelude::*;
use crate::establish_connection;

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = game)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Game {
    pub id: i32,
    pub name: String,
    pub created_at: chrono::NaiveDateTime,
}

impl Game {
    pub fn tributes(&self) -> Vec<crate::models::Tribute> {
        use crate::schema::tribute;
        let connection = &mut establish_connection();
        tribute::table
            .filter(tribute::game_id.eq(self.id))
            .load::<crate::models::Tribute>(connection)
            .expect("Error loading tributes")
    }
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

pub fn get_game(connection: &mut PgConnection, name: &str) -> Result<Game, std::io::Error> {
    let game = game::table
        .filter(game::name.ilike(name))
        .first(connection)
        .expect("Error loading game");
    Ok(game)
}

pub fn get_games(connection: &mut PgConnection) -> Vec<Game> {
    game::table
        .select(game::all_columns)
        .load::<Game>(connection)
        .expect("Error loading games")
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
