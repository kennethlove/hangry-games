use self::models::*;
use diesel::prelude::*;
use hangry_games::*;

fn main() {
    use self::schema::areas::dsl::*;

    let connection = &mut establish_connection();
    let results = areas
        .limit(5)
        .select(Area::as_select())
        .load(connection)
        .expect("Error loading areas");

    println!("Displaying {} areas", results.len());
    for area in results {
        println!("{}", area.name);
    }
}