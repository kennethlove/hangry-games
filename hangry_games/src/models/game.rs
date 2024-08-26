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
    pub day: Option<i32>,
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

    pub fn start(&self) {
        let connection = &mut establish_connection();
        let cornucopia = crate::models::get_area(connection, "The Cornucopia");
        let tributes = self.tributes();
        for mut tribute in tributes {
            tribute.set_area(&cornucopia);
        }
    }
}

#[derive(Insertable, Debug)]
#[diesel(table_name = game)]
pub struct NewGame<'a> {
    pub name: &'a str,
    pub day: i32,
}

pub fn create_game(connection: &mut PgConnection) -> Game {
    let name = generate_random_name();
    let new_game = NewGame { name: &name, day: 0 };

    diesel::insert_into(game::table)
        .values(&new_game)
        .returning(Game::as_returning())
        .get_result(connection)
        .expect("Error saving new area")
}

pub fn get_game(connection: &mut PgConnection, name: &str) -> Result<Game, std::io::Error> {
    let got_game = game::table
        .filter(game::name.ilike(name))
        .first(connection)
        .expect("Error loading game");
    Ok(got_game)
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
