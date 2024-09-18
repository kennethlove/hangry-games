use std::fmt::Display;
use std::str::FromStr;

pub enum Animal {
    Squirrel,
    Bear,
    Wolf,
    Cougar,
    Boar,
    Snake,
    Monkey,
    Baboon,
    Hyena,
    Lion,
    Tiger,
    Elephant,
    Rhino,
    Hippo,
    TrackerJacker,
}

impl FromStr for Animal {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "squirrel" => Ok(Animal::Squirrel),
            "bear" => Ok(Animal::Bear),
            "wolf" => Ok(Animal::Wolf),
            "cougar" => Ok(Animal::Cougar),
            "boar" => Ok(Animal::Boar),
            "snake" => Ok(Animal::Snake),
            "monkey" => Ok(Animal::Monkey),
            "baboon" => Ok(Animal::Baboon),
            "hyena" => Ok(Animal::Hyena),
            "lion" => Ok(Animal::Lion),
            "tiger" => Ok(Animal::Tiger),
            "elephant" => Ok(Animal::Elephant),
            "rhino" => Ok(Animal::Rhino),
            "hippo" => Ok(Animal::Hippo),
            "tracker jacker" => Ok(Animal::TrackerJacker),
            _ => Err(()),
        }
    }
}

impl Display for Animal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Animal::Squirrel => write!(f, "Squirrel"),
            Animal::Bear => write!(f, "Bear"),
            Animal::Wolf => write!(f, "Wolf"),
            Animal::Cougar => write!(f, "Cougar"),
            Animal::Boar => write!(f, "Boar"),
            Animal::Snake => write!(f, "Snake"),
            Animal::Monkey => write!(f, "Monkey"),
            Animal::Baboon => write!(f, "Baboon"),
            Animal::Hyena => write!(f, "Hyena"),
            Animal::Lion => write!(f, "Lion"),
            Animal::Tiger => write!(f, "Tiger"),
            Animal::Elephant => write!(f, "Elephant"),
            Animal::Rhino => write!(f, "Rhino"),
            Animal::Hippo => write!(f, "Hippo"),
            Animal::TrackerJacker => write!(f, "Tracker Jacker"),
        }
    }
}

impl Animal {
    pub fn as_str(&self) -> &str {
        match self {
            Animal::Squirrel => "Squirrel",
            Animal::Bear => "Bear",
            Animal::Wolf => "Wolf",
            Animal::Cougar => "Cougar",
            Animal::Boar => "Boar",
            Animal::Snake => "Snake",
            Animal::Monkey => "Monkey",
            Animal::Baboon => "Baboon",
            Animal::Hyena => "Hyena",
            Animal::Lion => "Lion",
            Animal::Tiger => "Tiger",
            Animal::Elephant => "Elephant",
            Animal::Rhino => "Rhino",
            Animal::Hippo => "Hippo",
            Animal::TrackerJacker => "Tracker Jacker",
        }
    }

    pub fn plural(&self) -> String {
        match self {
            Animal::Wolf => "Wolves".to_string(),
            _ => {
                let pluralized = format!("{}s", self.as_str());
                pluralized
            },
        }
    }
}
