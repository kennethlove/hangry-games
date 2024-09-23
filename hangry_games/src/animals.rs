use rand::Rng;
use std::fmt::Display;
use std::str::FromStr;

#[derive(Clone, Debug)]
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

    pub fn random() -> Animal {
        let mut rng = rand::thread_rng();
        let animals = [
            Animal::Squirrel,
            Animal::Bear,
            Animal::Wolf,
            Animal::Cougar,
            Animal::Boar,
            Animal::Snake,
            Animal::Monkey,
            Animal::Baboon,
            Animal::Hyena,
            Animal::Lion,
            Animal::Tiger,
            Animal::Elephant,
            Animal::Rhino,
            Animal::Hippo,
            Animal::TrackerJacker,
        ];
        let index = rng.gen_range(0..animals.len());
        animals[index].clone()
    }

    pub fn damage(&self) -> i32 {
        match self {
            Animal::Squirrel => 1,
            Animal::Bear => 10,
            Animal::Wolf => 5,
            Animal::Cougar => 5,
            Animal::Boar => 3,
            Animal::Snake => 2,
            Animal::Monkey => 3,
            Animal::Baboon => 5,
            Animal::Hyena => 5,
            Animal::Lion => 10,
            Animal::Tiger => 10,
            Animal::Elephant => 10,
            Animal::Rhino => 10,
            Animal::Hippo => 20,
            Animal::TrackerJacker => 5,
        }
    }
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

