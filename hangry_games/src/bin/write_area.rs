use hangry_games::*;
use std::io::{stdin, Read};

fn main() {
    let connection = &mut establish_connection();

    let mut name = String::new();

    println!("What's the name of the area?");
    stdin().read_line(&mut name).unwrap();
    let name = name.trim_end();

    let area = create_area(connection, name);
    println!("Saved area {:?}", area);
}