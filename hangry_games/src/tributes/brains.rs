use rand::{thread_rng, Rng};
use crate::tributes::actions::TributeAction;
use crate::tributes::actors::Tribute;

#[derive(Clone, Debug, PartialEq)]
pub struct TributeBrain {
    pub(crate) previous_actions: Vec<TributeAction>,
    pub(crate) preferred_action: Option<TributeAction>,
    pub(crate) preferred_action_percentage: f64,
}

impl TributeBrain {
    pub(crate) fn new() -> Self {
        Self {
            previous_actions: Vec::new(),
            preferred_action: None,
            preferred_action_percentage: 0.0,
        }
    }

    pub fn set_preferred_action(&mut self, action: TributeAction, percentage: f64) {
        self.preferred_action = Some(action);
        self.preferred_action_percentage = percentage;
    }

    pub fn clear_preferred_action(&mut self) {
        self.preferred_action = None;
        self.preferred_action_percentage = 0.0;
    }

    /// Decide on an action for the tribute to take
    /// First weighs any preferred actions, then decides based on current state
    pub fn act(&mut self, tribute: &Tribute, nearby_tributes: Vec<Tribute>) -> TributeAction {
        if tribute.health == 0 { return TributeAction::None; }

        if let Some(preferred_action) = self.clone().preferred_action {
            if thread_rng().gen_bool(self.preferred_action_percentage) {
                self.previous_actions.push(preferred_action.clone());
                return preferred_action;
            }
        }

        let action = TributeBrain::decide_on_action(tribute, nearby_tributes.clone());

        // Try to get a different action?

        self.previous_actions.push(action.clone());
        action
    }

    /// Get the last action taken by the tribute
    pub fn last_action(&self) -> TributeAction {
        if let Some(previous_action) = self.previous_actions.last() {
            previous_action.clone()
        } else {
            TributeAction::None
        }
    }

    /// The AI for a tribute. Automatic decisions based on current state.
    fn decide_on_action(tribute: &Tribute, nearby_tributes: Vec<Tribute>) -> TributeAction {
        // If the tribute isn't in the area, they do nothing
        if tribute.area.is_none() {
            return TributeAction::None;
        }
        if tribute.movement <= 0 {
            return TributeAction::Rest;
        }

        let _area = tribute.area.as_ref().unwrap();

        match &nearby_tributes.len() {
            0 => {
                match tribute.health {
                    // health is low, rest
                    1..=20 => TributeAction::Rest,
                    // health isn't great, hide
                    // unless sanity is also low, then move
                    21..=30 => {
                        if tribute.sanity > 20 && tribute.is_visible() {
                            TributeAction::Hide
                        } else {
                            TributeAction::Move
                        }
                    },
                    // health is good, move
                    _ => {
                        // If the tribute has movement, move
                        match tribute.movement {
                            0 => TributeAction::Rest,
                            _ => TributeAction::Move,
                        }
                    }
                }
            }
            1..6 => {
                // Enemies are nearby, attack depending on health
                match tribute.health {
                    // health is low, hide
                    1..=5 => {
                        if tribute.sanity > 20 && tribute.is_visible() {
                            TributeAction::Hide
                        } else {
                            TributeAction::Attack
                        }
                    },
                    // health isn't great, run away
                    6..=10 => {
                        if tribute.sanity > 20 {
                            TributeAction::Move
                        } else {
                            TributeAction::Attack
                        }
                    },
                    // health is good, attack
                    _ => TributeAction::Attack,
                }
            },
            _ => {
                // More than 5 enemies? Intelligence decides next move
                let sense = 100 - tribute.intelligence.unwrap() - tribute.sanity;
                match sense {
                    // Too dumb to know better, attacks
                    0..36 => TributeAction::Attack,
                    // Smart enough to know better, hides
                    85..101 => TributeAction::Hide,
                    // Average intelligence, moves
                    _ => TributeAction::Move,
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::tributes::actors::Tribute;
    use crate::tributes::actions::TributeAction;
    #[test]
    fn decide_on_action_default() {
        // If there are no enemies nearby, the tribute should move
        let mut tribute = Tribute::new("Katniss".to_string(), None);
        let action = tribute.brain.act(&tribute.clone(), vec![]);
        assert_eq!(action, TributeAction::Move);
    }

    #[test]
    fn decide_on_action_low_health() {
        // If the tribute has low health, they should rest
        let mut tribute = Tribute::new("Katniss".to_string(), None);
        tribute.takes_physical_damage(90);
        let action = tribute.brain.act(&tribute.clone(), vec![]);
        assert_eq!(action, TributeAction::Rest);
    }

    #[test]
    fn decide_on_action_no_movement() {
        // If the tribute has no movement, they should rest
        let mut tribute = Tribute::new("Katniss".to_string(), None);
        tribute.moves();
        tribute.moves();
        let action = tribute.brain.act(&tribute.clone(), vec![]);
        assert_eq!(action, TributeAction::Rest);
    }

    #[test]
    fn decide_on_action_enemies() {
        // If there are enemies nearby, the tribute should attack
        let mut tribute = Tribute::new("Katniss".to_string(), None);
        let tribute2 = Tribute::new("Peeta".to_string(), None);
        let action = tribute.brain.act(&tribute.clone(), vec![tribute.clone(), tribute2]);
        assert_eq!(action, TributeAction::Attack);
    }

    #[test]
    fn decide_on_action_enemies_low_health() {
        // If there are enemies nearby, but the tribute is low on health
        // the tribute should hide
        let mut tribute = Tribute::new("Katniss".to_string(), None);
        tribute.takes_physical_damage(90);
        let tribute2 = Tribute::new("Peeta".to_string(), None);
        let action = tribute.brain.act(&tribute.clone(),vec![tribute.clone(), tribute2]);
        assert_eq!(action, TributeAction::Move);
    }
}