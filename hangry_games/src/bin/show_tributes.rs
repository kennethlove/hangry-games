use self::models::tributes::*;
use diesel::prelude::*;
use hangry_games::*;

fn main() {
    use self::schema::tributes::dsl::*;

    let connection = &mut establish_connection();
    let results = tributes
        .select(Tribute::as_select())
        .load(connection)
        .expect("Error loading areas");

    println!("Displaying {} tributes", results.len());
    for tribute in results {
        println!("{}, District {}", tribute.name, tribute.district);
    }
}
