use crate::schema::game;
use diesel::prelude::*;
use crate::{establish_connection, models};
use crate::models::{bleed_tribute, get_area_by_id, suffer_tribute, update_tribute, Tribute};
use rand::seq::SliceRandom;
use fake::faker::name::raw::Name;
use fake::locales::EN;
use fake::Fake;
use rand::Rng;
use crate::areas::Area;
use crate::events::AreaEvent;
use crate::tributes::statuses::TributeStatus;

#[derive(Queryable, Selectable, Clone, Debug)]
#[diesel(table_name = game)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Game {
    pub id: i32,
    pub name: String,
    pub created_at: chrono::NaiveDateTime,
    pub day: Option<i32>,
    pub closed_areas: Option<Vec<Option<i32>>>,
    pub ended_at: Option<chrono::NaiveDateTime>,
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

    pub fn living_tributes(&self) -> Vec<Tribute> {
        use crate::schema::tribute;
        let connection = &mut establish_connection();
        tribute::table
            .filter(tribute::game_id.eq(self.id))
            .filter(tribute::status.ne(TributeStatus::Dead.to_string()))
            .filter(tribute::status.ne(TributeStatus::RecentlyDead.to_string()))
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
        let connection = &mut establish_connection();

        let tributes = self.tributes();
        for tribute in tributes {
            tribute.unset_area();
        }

        let ended_at = Some(chrono::Utc::now().naive_utc());
        diesel::update(game::table.find(self.id))
            .set(game::ended_at.eq(ended_at))
            .execute(connection)
            .expect("Error updating game");
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
        self.closed_areas = Some(closed_areas.clone());

        diesel::update(game::table.find(self.id))
            .set(game::closed_areas.eq(closed_areas))
            .execute(connection)
            .expect("Error updating game");
    }

    pub fn open_area(&mut self, area: &models::Area) {
        let connection = &mut establish_connection();

        let mut closed_areas = vec![];
        let closed_areas = self.closed_areas.as_mut().unwrap_or(&mut closed_areas);
        let closed_areas = closed_areas.iter().filter(|a| a.unwrap() != area.id).cloned().collect::<Vec<_>>();
        self.closed_areas = Some(closed_areas.clone());

        diesel::update(game::table.find(self.id))
            .set(game::closed_areas.eq(closed_areas.clone()))
            .execute(connection)
            .expect("Error updating game");
    }

    pub fn do_day(&mut self) {
        let mut rng = rand::thread_rng();

        self.do_area_event_cleanup();

        // Trigger any daytime events
        if self.day > Some(2) && rng.gen_bool(1.0 / 4.0) {
            self.do_area_event();
        }

        let mut living_tributes = get_all_living_tributes(&self);

        // Run the tribute AI
        living_tributes.shuffle(&mut rng);
        for tribute in living_tributes {
            let mut tribute = bleed_tribute(tribute);
            tribute.do_day();
        }
    }

    pub fn do_night(&mut self) {
        let mut rng = rand::thread_rng();

        self.do_area_event_cleanup();

        // Trigger any nighttime events
        if rng.gen_bool(1.0 / 8.0) {
            self.do_area_event();
        }

        let mut living_tributes = get_all_living_tributes(&self);

        // Run the tribute AI
        living_tributes.shuffle(&mut rng);
        for mut tribute in living_tributes {
            tribute = suffer_tribute(tribute);
            tribute.do_night();
        }
    }

    fn do_area_event(&mut self) {
        // Event happens
        let event = AreaEvent::random();
        let closed_areas = self.closed_areas.clone().unwrap_or(vec![]);
        let closed_areas = closed_areas.iter()
            .map(|a| get_area_by_id(*a))
            .map(|a| Area::from(a.unwrap()))
            .collect::<Vec<_>>();
        let area = Area::random_open(closed_areas);
        let model_area = models::Area::from(area.clone());
        println!("=== 🚨 A(n) {} has occurred in {} ===", event, area);
        models::AreaEvent::create(event.to_string(), model_area.id, self.id);
        self.close_area(&model_area);
    }

    fn do_area_event_cleanup(&mut self) {
        let mut rng = rand::thread_rng();

        // Handle closed areas
        for area_id in self.closed_areas.clone().unwrap_or(vec![]) {
            let area = get_area_by_id(area_id).unwrap();
            let mut tributes = area.tributes(self.id);
            let tributes = tributes
                .iter_mut()
                .filter(|t| t.day_killed.is_none())
                .collect::<Vec<_>>();

            let area_name = area.name.strip_prefix("The ").unwrap_or(area.name.as_str());
            for tribute in tributes {
                println!("{} is trapped in the {}.", tribute.name, area_name);
                tribute.health = 0;
                tribute.status = TributeStatus::RecentlyDead.to_string();
                tribute.is_hidden = Some(false);
                tribute.killed_by = Some("The Gamemakers".to_string());
                update_tribute(tribute.id, tribute.clone());
            }

            // Re-open area?
            if rng.gen_bool(0.5) {
                println!("=== 🔔 The Gamemakers open the {} ===", area_name);
                self.open_area(&area);
            }
        }
    }

    fn do_deaths(&self) {
        let dead_tributes = get_recently_dead_tributes(&self);

        for tribute in &dead_tributes {
            tribute.dies();
        }

        // Announce them
        println!("=== 📯 {} tribute{} died ===", dead_tributes.len(), if dead_tributes.len() == 1 { "" } else { "s" });
        for tribute in dead_tributes {
            println!("- 💀 {}", tribute.name);
        }
    }

    pub fn run_next_day(&mut self) {
        // Update the day
        let day = self.day.unwrap_or(0) + 1;
        self.set_day(day);

        let living_tributes = get_all_living_tributes(&self);

        // Check for winner
        if living_tributes.len() == 1 {
            println!("=== 🏆 The winner is {} ===", living_tributes[0].name);
            self.end();
            return;
        }

        // Make day announcements
        match self.day {
            Some(1) => {
                println!("=== 🎉 The Hunger Games begin! 🎉 ===");
            }
            Some(3) => {
                println!("=== 😋 Feast Day ===");
            }
            _ => {
                println!("=== ☀️ Day {} begins ===", self.day.unwrap());
            }
        }

        println!("=== {} tribute{} remain{} ===",
                 living_tributes.len(),
                 if living_tributes.len() == 1 { "" } else { "s" },
                 if living_tributes.len() == 1 { "s" } else { "" }
        );

        self.do_day();

        self.do_deaths();

        println!("=== 🌙 Night {} begins ===", day);
        self.do_night();

        self.do_deaths();
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
        .filter(tribute::status.ne(TributeStatus::Dead.to_string()))
        .filter(tribute::status.ne(TributeStatus::RecentlyDead.to_string()))
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
        .filter(tribute::status.eq(TributeStatus::Dead.to_string()))
        .filter(tribute::day_killed.is_not_null())
        .load::<Tribute>(conn)
        .expect("Error loading dead tributes")
}

pub fn get_recently_dead_tributes(game: &Game) -> Vec<Tribute> {
    use crate::schema::tribute;
    let conn = &mut establish_connection();
    tribute::table
        .select(tribute::all_columns)
        .filter(tribute::game_id.eq(game.id))
        .filter(
            tribute::status.eq_any(vec![
                TributeStatus::RecentlyDead.to_string(),
                TributeStatus::Wounded.to_string(),
            ])
        )
        .filter(tribute::health.le(0))
        .load::<Tribute>(conn)
        .expect("Error loading recently dead tributes")
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