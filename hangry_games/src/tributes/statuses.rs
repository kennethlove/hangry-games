use std::fmt::Display;
use std::str::FromStr;

#[derive(Clone, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub enum TributeStatus {
    #[default]
    Healthy,
    Wounded,
    Starving,
    Dehydrated,
    Sick,
    Poisoned,
    RecentlyDead,
    Dead,
    Electrocuted,
    Frozen,
    Overheated,
    Broken,
    Infected,
    Drowned,
}

impl FromStr for TributeStatus {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "healthy" => Ok(TributeStatus::Healthy),
            "wounded" => Ok(TributeStatus::Wounded),
            "injured" => Ok(TributeStatus::Wounded),
            "starving" => Ok(TributeStatus::Starving),
            "dehydrated" => Ok(TributeStatus::Dehydrated),
            "sick" => Ok(TributeStatus::Sick),
            "poisoned" => Ok(TributeStatus::Poisoned),
            "recently-dead" => Ok(TributeStatus::RecentlyDead),
            "recently_dead" => Ok(TributeStatus::RecentlyDead),
            "recently dead" => Ok(TributeStatus::RecentlyDead),
            "recentlydead" => Ok(TributeStatus::RecentlyDead),
            "dead" => Ok(TributeStatus::Dead),
            "electrocuted" => Ok(TributeStatus::Electrocuted),
            "frozen" => Ok(TributeStatus::Frozen),
            "overheated" => Ok(TributeStatus::Overheated),
            "broken" => Ok(TributeStatus::Broken),
            "infected" => Ok(TributeStatus::Infected),
            "drowned" => Ok(TributeStatus::Drowned),
            _ => Err(()),
        }
    }
}

impl Display for TributeStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TributeStatus::Healthy => write!(f, "Healthy"),
            TributeStatus::Wounded => write!(f, "Wounded"),
            TributeStatus::Starving => write!(f, "Starving"),
            TributeStatus::Dehydrated => write!(f, "Dehydrated"),
            TributeStatus::Sick => write!(f, "Sick"),
            TributeStatus::Poisoned => write!(f, "Poisoned"),
            TributeStatus::RecentlyDead => write!(f, "RecentlyDead"),
            TributeStatus::Dead => write!(f, "Dead"),
            TributeStatus::Electrocuted => write!(f, "Electrocuted"),
            TributeStatus::Frozen => write!(f, "Frozen"),
            TributeStatus::Overheated => write!(f, "Overheated"),
            TributeStatus::Broken => write!(f, "Broken"),
            TributeStatus::Infected => write!(f, "Infected"),
            TributeStatus::Drowned => write!(f, "Drowned"),
        }
    }
}