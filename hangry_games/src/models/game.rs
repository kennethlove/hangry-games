use crate::schema::game;
use diesel::prelude::*;
use crate::establish_connection;
use crate::models::get_all_living_tributes;
use rand::seq::SliceRandom;

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
    pub fn tributes(&self) -> Vec<crate::models::Tribute> {
        use crate::schema::tribute;
        let connection = &mut establish_connection();
        tribute::table
            .filter(tribute::game_id.eq(self.id))
            .load::<crate::models::Tribute>(connection)
            .expect("Error loading tributes")
    }

    pub fn start(&self) {
        let cornucopia = crate::models::get_area("The Cornucopia");

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

    pub fn close_area(&mut self, area: &crate::models::Area) {
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

    pub fn open_area(&mut self, area: &crate::models::Area) {
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
        println!("📅 Day {} begins.", day + 1);

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
        // Find the tributes that have no health and kill them
        let dead_tributes = self.tributes().into_iter()
            .filter(|t| t.is_alive == false && t.day_killed.is_none())
            .collect::<Vec<_>>();

        for tribute in &dead_tributes {
            tribute.dies();
        }

        // Announce them
        println!("💀 {} tribute{} died today", dead_tributes.len(), if dead_tributes.len() == 1 { "" } else { "s" });
        for tribute in dead_tributes {
            println!("💀 {}", tribute.name);
        }
        // Activate any nighttime events
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
