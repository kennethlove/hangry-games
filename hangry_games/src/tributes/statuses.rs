use std::fmt::Display;
use std::str::FromStr;

#[derive(Clone, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub enum TributeStatus {
    #[default]
    Healthy,
    Injured,
    // Dying,
    // Starving,
    // Dehydrated,
    // Sick,
    // Poisoned,
    Dead,
    // Drunk,
    // High,
    // Insane,
    // Hallucinating,
    // Paralyzed,
    // Asleep,
    // Unconscious,
    // Revived,
}

impl FromStr for TributeStatus {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "healthy" => Ok(TributeStatus::Healthy),
            "injured" => Ok(TributeStatus::Injured),
            // "dying" => Ok(TributeStatus::Dying),
            // "starving" => Ok(TributeStatus::Starving),
            // "dehydrated" => Ok(TributeStatus::Dehydrated),
            // "sick" => Ok(TributeStatus::Sick),
            // "poisoned" => Ok(TributeStatus::Poisoned),
            "dead" => Ok(TributeStatus::Dead),
            // "drunk" => Ok(TributeStatus::Drunk),
            // "high" => Ok(TributeStatus::High),
            // "insane" => Ok(TributeStatus::Insane),
            // "hallucinating" => Ok(TributeStatus::Hallucinating),
            // "paralyzed" => Ok(TributeStatus::Paralyzed),
            // "asleep" => Ok(TributeStatus::Asleep),
            // "unconscious" => Ok(TributeStatus::Unconscious),
            // "revived" => Ok(TributeStatus::Revived),
            _ => Err(()),
        }
    }
}

impl Display for TributeStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TributeStatus::Healthy => write!(f, "Healthy"),
            TributeStatus::Injured => write!(f, "Injured"),
            // TributeStatus::Dying => write!(f, "Dying"),
            // TributeStatus::Starving => write!(f, "Starving"),
            // TributeStatus::Dehydrated => write!(f, "Dehydrated"),
            // TributeStatus::Sick => write!(f, "Sick"),
            // TributeStatus::Poisoned => write!(f, "Poisoned"),
            TributeStatus::Dead => write!(f, "Dead"),
            // TributeStatus::Drunk => write!(f, "Drunk"),
            // TributeStatus::High => write!(f, "High"),
            // TributeStatus::Insane => write!(f, "Insane"),
            // TributeStatus::Hallucinating => write!(f, "Hallucinating"),
            // TributeStatus::Paralyzed => write!(f, "Paralyzed"),
            // TributeStatus::Asleep => write!(f, "Asleep"),
            // TributeStatus::Unconscious => write!(f, "Unconscious"),
            // TributeStatus::Revived => write!(f, "Revived"),
        }
    }
}