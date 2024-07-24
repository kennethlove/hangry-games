use hangry_games::tributes::tribute_actors::Tribute;

fn main() {
    let mut tribute = Tribute::new();
    tribute.hunger = 5;
    let decision = tribute.decide_on_action();
    dbg!(decision);
}
