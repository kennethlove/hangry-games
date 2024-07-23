#[derive(Debug)]
pub struct Tribute {
    pub name: String,
    pub health: u32,
    pub sanity: u32,
    pub hunger: u32,
    pub sleep: u32,
    pub movement: u32,
    pub is_alive: bool,
}

impl Tribute {
    /// Creates a new Tribute with full health, sanity, hunger, sleep, and movement
    pub fn new() -> Self {
        Self {
            name: String::from("Tribute"),
            health: 100,
            sanity: 100,
            hunger: 100,
            sleep: 100,
            movement: 100,
            is_alive: true,
        }
    }

    /// Reduces health
    pub fn takes_damage(&mut self, damage: u32) {
        self.health = self.health.saturating_sub(damage);

        if self.health == 0 {
            self.dies();
        }
    }

    /// Reduces mental health
    pub fn takes_mental_damage(&mut self, damage: u32) {
        self.sanity = self.sanity.saturating_sub(damage);
    }

    /// Restores health
    pub fn heals(&mut self, health: u32) {
        self.health = self.health.saturating_add(health);
    }

    /// Restores mental health
    pub fn heals_mental_damage(&mut self, health: u32) {
        self.sanity = self.sanity.saturating_add(health);
    }

    /// Reduces hunger and deals 10 damage, health and mental, if hunger is 0
    pub fn hungers(&mut self, hunger: u32) {
        self.hunger = self.hunger.saturating_sub(hunger);

        if self.hunger == 0 {
            self.takes_damage(10);
            self.takes_mental_damage(10);
        }
    }

    /// Restores hunger to 100 and heals 10 health
    pub fn eats(&mut self) {
        self.hunger = 100;
        self.heals(10)
    }

    /// Reduces sleep and deals 10 damage, health and mental, if sleep is 0
    pub fn tires(&mut self, sleep: u32) {
        self.sleep = self.sleep.saturating_sub(sleep);

        if self.sleep == 0 {
            self.takes_damage(10);
            self.takes_mental_damage(10);
        }
    }

    /// Restores sleep to 100 and heals 10 health and mental damage
    pub fn sleeps(&mut self) {
        self.sleep = 100;
        self.heals(10);
        self.heals_mental_damage(10);
    }

    pub fn moves(&mut self, distance: u32) {
        self.movement = self.movement.saturating_sub(distance);
        self.tires(distance);
    }

    pub fn rests(&mut self) {
        self.movement = 100;
    }

    pub fn dies(&mut self) {
        self.is_alive = false;
    }
}

impl Default for Tribute {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let tribute = Tribute::new();
        assert_eq!(tribute.health, 100);
        assert_eq!(tribute.sanity, 100);
        assert_eq!(tribute.hunger, 100);
        assert_eq!(tribute.sleep, 100);
        assert_eq!(tribute.movement, 100);
        assert!(tribute.is_alive);
    }

    #[test]
    fn takes_damage() {
        let mut tribute = Tribute::new();
        tribute.takes_damage(10);
        assert_eq!(tribute.health, 90);
        tribute.takes_damage(100);
        assert_eq!(tribute.health, 0);
    }

    #[test]
    fn takes_mental_damage() {
        let mut tribute = Tribute::new();
        tribute.takes_mental_damage(10);
        assert_eq!(tribute.sanity, 90);
    }

    #[test]
    fn hungers_and_eats() {
        let mut tribute = Tribute::new();

        // Hunger to 50
        tribute.hungers(50);
        assert_eq!(tribute.hunger, 50);

        // Hunger to 0
        tribute.hungers(60);
        assert_eq!(tribute.hunger, 0);
        assert_eq!(tribute.health, 90);

        // Hunger to 100
        tribute.eats();
        assert_eq!(tribute.hunger, 100);
        assert_eq!(tribute.health, 100);
    }

    #[test]
    fn tires_and_sleeps() {
        let mut tribute = Tribute::new();

        // Sleep to 90
        tribute.tires(10);
        assert_eq!(tribute.sleep, 90);

        // Sleep to 0
        tribute.tires(100);
        assert_eq!(tribute.sleep, 0);
        assert_eq!(tribute.health, 90);
        assert_eq!(tribute.sanity, 90);

        tribute.sleeps();
        assert_eq!(tribute.sleep, 100);
        assert_eq!(tribute.health, 100);
    }

    #[test]
    fn moves_and_rests() {
        let mut tribute = Tribute::new();
        tribute.moves(10);
        assert_eq!(tribute.movement, 90);
        assert_eq!(tribute.sleep, 90);
        tribute.rests();
        assert_eq!(tribute.movement, 100);
    }

    #[test]
    fn dies() {
        let mut tribute = Tribute::new();
        tribute.takes_damage(100);
        assert!(!tribute.is_alive);
    }
}
