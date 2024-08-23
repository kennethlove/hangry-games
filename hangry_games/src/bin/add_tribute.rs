use hangry_games::db::establish_connection;
use hangry_games::db::create_tribute;
use std::io::stdin;

fn main() {
    let connection = &mut establish_connection();

    let mut name = String::new();

    println!("What's the name of the tribute?");
    stdin().read_line(&mut name).unwrap();
    let name = name.trim_end();

    let tribute = create_tribute(connection, name);
    println!("Saved tribute {:?}", tribute);
}
