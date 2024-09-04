use crate::schema::game;
use diesel::prelude::*;
use crate::{establish_connection, models};
use crate::models::Tribute;
use rand::seq::SliceRandom;
use fake::faker::name::raw::Name;
use fake::locales::EN;
use fake::Fake;

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = game)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Game {
    pub id: i32,
    pub name: String,
    pub created_at: chrono::NaiveDateTime,
    pub day: Option<i32>,
    pub closed_areas: Option<Vec<Option<i32>>>,
}

impl Game {
    pub fn tributes(&self) -> Vec<Tribute> {
        use crate::schema::tribute;
        let connection = &mut establish_connection();
        tribute::table
            .filter(tribute::game_id.eq(self.id))
            .load::<Tribute>(connection)
            .expect("Error loading tributes")
    }

    pub fn start(&self) {
        let cornucopia = models::get_area("The Cornucopia");

        let tributes = self.tributes();
        for mut tribute in tributes {
            tribute.set_area(&cornucopia);
        }
    }

    pub fn end(&self) {
        let tributes = self.tributes();
        for tribute in tributes {
            tribute.unset_area();
        }
    }

    pub fn set_day(&self, day_number: i32) {
        let connection = &mut establish_connection();
        diesel::update(game::table.find(self.id))
            .set(game::day.eq(Some(day_number)))
            .execute(connection)
            .expect("Error updating game");
    }

    pub fn close_area(&mut self, area: &models::Area) {
        let connection = &mut establish_connection();

        let mut binding: Vec<Option<i32>> = vec![];
        let closed_areas = self.closed_areas.as_mut().unwrap_or(&mut binding);
        closed_areas.push(Some(area.id));
        let closed_areas = closed_areas.clone();

        diesel::update(game::table.find(self.id))
            .set(game::closed_areas.eq(closed_areas))
            .execute(connection)
            .expect("Error updating game");
    }

    pub fn open_area(&mut self, area: &models::Area) {
        let connection = &mut establish_connection();

        let mut closed_areas = vec![];
        let closed_areas = self.closed_areas.as_mut().unwrap_or(&mut closed_areas);
        let closed_areas = closed_areas.iter().filter(|a| a.unwrap() != area.id).collect::<Vec<_>>();

        diesel::update(game::table.find(self.id))
            .set(game::closed_areas.eq(closed_areas))
            .execute(connection)
            .expect("Error updating game");
    }

    pub fn do_day(&mut self) {
        let mut rng = rand::thread_rng();

        // Update the day
        let day = self.day.unwrap_or(0);
        self.set_day(day + 1);
        println!("☀️ Day {} begins.", day + 1);

        // Check for winner
        let mut living_tributes = get_all_living_tributes(&self);
        if living_tributes.len() == 1 {
            println!("🏆 The winner is {}", living_tributes[0].name);
            return;
        }

        println!("{} tributes remain", living_tributes.len());

        // Trigger any daytime events

        // Run the tribute AI
        living_tributes.shuffle(&mut rng);
        for mut tribute in living_tributes {
            tribute.do_day();
        }
    }

    pub fn do_night(&mut self) {
        let living_tributes = get_all_living_tributes(&self);
        // Find the tributes that have no health and kill them
        let dead_tributes = get_dead_tributes(&self).into_iter()
            .filter(|t| t.day_killed.is_none())
            .collect::<Vec<_>>();

        for tribute in &dead_tributes {
            tribute.dies();
        }

        // Announce them
        println!("📯 {} tribute{} died since yesterday", dead_tributes.len(), if dead_tributes.len() == 1 { "" } else { "s" });
        for tribute in dead_tributes {
            println!("💀 {}", tribute.name);
        }

        // Trigger any nighttime events

        println!("🌙 Night {} begins", self.day.unwrap_or(0) + 1);
        // Trigger any nighttime events

        // Run the tribute AI
        for mut tribute in living_tributes {
            tribute.do_night();
        }
    }
}

#[derive(Insertable, Debug)]
#[diesel(table_name = game)]
pub struct NewGame<'a> {
    pub name: &'a str,
    pub day: i32,
}

pub fn create_game() -> Game {
    let connection = &mut establish_connection();
    let name = generate_random_name();
    let new_game = NewGame { name: &name, day: 0 };

    diesel::insert_into(game::table)
        .values(&new_game)
        .returning(Game::as_returning())
        .get_result(connection)
        .expect("Error saving new area")
}

pub fn get_game(name: &str) -> Result<Game, std::io::Error> {
    let connection = &mut establish_connection();
    let got_game = game::table
        .filter(game::name.ilike(name))
        .first(connection)
        .expect("Error loading game");
    Ok(got_game)
}

pub fn get_game_by_id(id: i32) -> Result<Game, std::io::Error> {
    let connection = &mut establish_connection();
    let got_game = game::table
        .filter(game::id.eq(id))
        .first(connection)
        .expect("Error loading game");
    Ok(got_game)
}

pub fn get_games() -> Vec<Game> {
    let connection = &mut establish_connection();
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

pub fn get_all_living_tributes(game: &Game) -> Vec<Tribute> {
    let conn = &mut establish_connection();
    use crate::schema::tribute;
    tribute::table
        .select(tribute::all_columns)
        .filter(tribute::game_id.eq(game.id))
        .filter(tribute::is_alive.eq(true))
        .load::<Tribute>(conn)
        .expect("Error loading tributes")
}

pub fn get_game_tributes(game: &Game) -> Vec<Tribute> {
    use crate::schema::tribute;
    let conn = &mut establish_connection();
    tribute::table
        .select(tribute::all_columns)
        .filter(tribute::game_id.eq(game.id))
        .load::<Tribute>(conn)
        .expect("Error loading tributes")
}

pub fn get_dead_tributes(game: &Game) -> Vec<Tribute> {
    use crate::schema::tribute;
    let conn = &mut establish_connection();
    tribute::table
        .select(tribute::all_columns)
        .filter(tribute::game_id.eq(game.id))
        .filter(tribute::is_alive.eq(false))
        .load::<Tribute>(conn)
        .expect("Error loading dead tributes")
}

/// Fill the tribute table with up to 24 tributes.
/// Return the number of tributes created.
pub fn fill_tributes(game: &Game) -> usize {
    let tributes = get_game_tributes(game);
    let count = tributes.len();
    if count < 24 {
        for _ in count..24 {
            let name: String = Name(EN).fake();
            let mut tribute = models::create_tribute(&name);
            tribute.set_game(&game)
        }
    }
    24 - count
}