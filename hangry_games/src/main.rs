use hangry_games::areas::Area;
use hangry_games::tributes::tribute_actors::Tribute;

fn main() {
    let cornucopia = Area::Cornucopia;
    let mut katniss = Tribute::new();
    katniss.name = "Katniss".to_string();
    katniss.district = 12;

    katniss.changes_area(cornucopia);

    // let nearby_enemies = cornucopia.nearby_enemies(&katniss.clone());
    // dbg!(nearby_enemies);

    for mut tribute in vec![katniss] {
        tribute.brain.act(&tribute.clone());
        tribute.takes_physical_damage(90);
        tribute.brain.act(&tribute.clone());
        dbg!(tribute);
    }
}
