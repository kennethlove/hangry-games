use crate::areas::Area;

use super::actions::TributeAction;

#[derive(Clone, Debug, PartialEq)]
pub struct Tribute {
    pub name: String,
    pub health: u32,
    pub sanity: u32,
    pub movement: u32,
    pub is_alive: bool,
    pub district: u32,
    pub brain: TributeBrain,
    pub area: Option<Area>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TributeBrain {
    previous_actions: Vec<TributeAction>,
}

impl TributeBrain {
    fn new() -> Self {
        Self {
            previous_actions: Vec::new(),
        }
    }

    pub fn act(&mut self, tribute: &Tribute, nearby_tributes: Vec<Tribute>) -> TributeAction {
        if tribute.health == 0 { return TributeAction::Idle; }
        let action = TributeBrain::decide_on_action(tribute, nearby_tributes);
        self.previous_actions.push(action.clone());
        action
    }

    pub fn last_action(&self) -> TributeAction {
        if let Some(last) = self.previous_actions.last() {
            last.clone()
        } else {
            TributeAction::Idle
        }
    }

    /// The AI for a tribute. Automatic decisions based on current state.
    fn decide_on_action(tribute: &Tribute, nearby_tributes: Vec<Tribute>) -> TributeAction {
        // If the tribute isn't in the area, they do nothing
        if tribute.area.is_none() {
            return TributeAction::Idle;
        }

        let _area = tribute.area.as_ref().unwrap();

        if nearby_tributes.len() > 1 {
            // enemies are nearby
            return match tribute.health {
                // health is low, hide
                1..=20 => TributeAction::Hide,
                // health isn't great, run away
                21..=50 => TributeAction::Move,
                // health is good, attack
                _ => TributeAction::Attack,
            };
        }

        if nearby_tributes.len() > 5 {
            // too many enemies nearby, run away
            return TributeAction::Move;
        }

        // no enemies nearby
        match tribute.health {
            // health is low, rest
            0..=10 => TributeAction::Hide,
            11..=20 => TributeAction::Rest,
            // health is good, move
            _ => {
                // If the tribute has movement, move
                match tribute.movement {
                    0 => TributeAction::Idle,
                    _ => TributeAction::Move,
                }
            }
        }
    }
}


impl Tribute {
    /// Creates a new Tribute with full health, sanity, and movement
    pub fn new(name: String) -> Self {
        let brain = TributeBrain::new();
        Self {
            name: name.clone(),
            health: 100,
            sanity: 100,
            movement: 100,
            is_alive: true,
            district: 0,
            area: Some(Area::default()),
            brain,
        }
    }

    /// Reduces health
    pub fn takes_physical_damage(&mut self, damage: u32) {
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

    pub fn moves(&mut self, distance: u32) {
        self.movement = self.movement.saturating_sub(distance);
    }

    pub fn rests(&mut self) {
        self.movement = 100;
    }

    pub fn dies(&mut self) {
        self.is_alive = false;
    }

    pub fn changes_area(&mut self, area: Area) {
        self.area = Some(area);
    }

    pub fn leaves_area(&mut self) {
        self.area = None;
    }
}

impl Default for Tribute {
    fn default() -> Self {
        Self::new("Tribute".to_string())
    }
}

use crate::models::Tribute as TributeModel;
impl From<TributeModel> for Tribute {
    fn from(tribute: crate::models::tribute::Tribute) -> Self {
        use crate::areas::Area;
        use crate::tributes::actions::TributeAction;

        let area = Area::from(tribute.area().unwrap());
        let actions: Vec<TributeAction> = tribute.actions()
            .iter()
            .map(TributeAction::from)
            .collect();

        let brain = TributeBrain {
            previous_actions: actions,
        };

        Self {
            name: tribute.name.clone(),
            health: tribute.health as u32,
            sanity: tribute.sanity as u32,
            movement: tribute.movement as u32,
            is_alive: tribute.is_alive,
            district: tribute.district as u32,
            brain,
            area: Some(area),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let tribute = Tribute::new("Katniss".to_string());
        assert_eq!(tribute.health, 100);
        assert_eq!(tribute.sanity, 100);
        assert_eq!(tribute.movement, 100);
        assert!(tribute.is_alive);
    }

    #[test]
    fn takes_physical_damage() {
        let mut tribute = Tribute::new("Katniss".to_string());
        tribute.takes_physical_damage(10);
        assert_eq!(tribute.health, 90);
    }

    #[test]
    fn takes_mental_damage() {
        let mut tribute = Tribute::new("Katniss".to_string());
        tribute.takes_mental_damage(10);
        assert_eq!(tribute.sanity, 90);
    }

    #[test]
    fn moves_and_rests() {
        let mut tribute = Tribute::new("Katniss".to_string());
        tribute.moves(10);
        assert_eq!(tribute.movement, 90);
        tribute.rests();
        assert_eq!(tribute.movement, 100);
    }

    #[test]
    fn takes_damage_and_dies() {
        let mut tribute = Tribute::new("Katniss".to_string());
        tribute.takes_physical_damage(100);
        assert!(!tribute.is_alive);
    }

    #[test]
    #[ignore = "No way to find nearby enemies yet"]
    fn no_nearby_enemies() {
        let mut tribute = Tribute::new("Katniss".to_string());
        let _area = Area::default();
        assert!(true);
    }

    #[test]
    #[ignore = "No way to find nearby enemies yet"]
    fn nearby_enemies() {
        let mut tribute = Tribute::new("Katniss".to_string());
        let mut tribute = Tribute::new("Peeta".to_string());
        assert!(tribute.area.is_some());
    }

    #[test]
    fn decide_on_action_default() {
        // If there are no enemies nearby, the tribute should move
        let mut tribute = Tribute::new("Katniss".to_string());
        let action = tribute.brain.act(&tribute.clone());
        assert_eq!(action, TributeAction::Move);
    }

    #[test]
    fn decide_on_action_low_health() {
        // If the tribute has low health, they should hide
        let mut tribute = Tribute::new("Katniss".to_string());
        tribute.takes_physical_damage(90);
        let action = tribute.brain.act(&tribute.clone());
        assert_eq!(action, TributeAction::Hide);
    }

    #[test]
    fn decide_on_action_no_movement() {
        // If the tribute has no movement, they should rest
        let mut tribute = Tribute::new("Katniss".to_string());
        tribute.moves(100);
        let action = tribute.brain.act(&tribute.clone());
        assert_eq!(action, TributeAction::Rest);
    }

    #[test]
    #[ignore = "No way to find nearby enemies yet"]
    fn decide_on_action_enemies() {
        // If there are enemies nearby, the tribute should attack
        let mut tribute = Tribute::new("Katniss".to_string());
        let _ = Tribute::new();
        let action = tribute.brain.act(&tribute.clone());
        assert_eq!(action, TributeAction::Attack);
    }

    #[test]
    #[ignore = "nearby_enemies is not implemented"]
    fn decide_on_action_enemies_low_health() {
        // If there are enemies nearby, but the tribute is low on health
        // the tribute should attack
        let mut tribute = Tribute::new("Katniss".to_string());
        tribute.takes_physical_damage(90);
        let _ = Tribute::new();
        let action = tribute.brain.act(&tribute.clone());
        assert_eq!(action, TributeAction::Hide);
    }
}
