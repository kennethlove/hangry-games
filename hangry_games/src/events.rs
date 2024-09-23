use crate::animals::Animal;
use rand::Rng;
use std::fmt::Display;
use std::str::FromStr;

#[derive(Clone, Debug)]
pub enum AreaEvent {
    Wildfire,
    Flood,
    Earthquake,
    Avalanche,
    Blizzard,
    Landslide,
}

impl FromStr for AreaEvent {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "wildfire" => Ok(AreaEvent::Wildfire),
            "flood" => Ok(AreaEvent::Flood),
            "earthquake" => Ok(AreaEvent::Earthquake),
            "avalanche" => Ok(AreaEvent::Avalanche),
            "blizzard" => Ok(AreaEvent::Blizzard),
            "landslide" => Ok(AreaEvent::Landslide),
            _ => Err(()),
        }
    }
}

impl Display for AreaEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AreaEvent::Wildfire => write!(f, "wildfire"),
            AreaEvent::Flood => write!(f, "flood"),
            AreaEvent::Earthquake => write!(f, "earthquake"),
            AreaEvent::Avalanche => write!(f, "avalanche"),
            AreaEvent::Blizzard => write!(f, "blizzard"),
            AreaEvent::Landslide => write!(f, "landslide"),
        }
    }
}

impl AreaEvent {
    pub fn as_str(&self) -> &str {
        match self {
            AreaEvent::Wildfire => "wildfire",
            AreaEvent::Flood => "flood",
            AreaEvent::Earthquake => "earthquake",
            AreaEvent::Avalanche => "avalanche",
            AreaEvent::Blizzard => "blizzard",
            AreaEvent::Landslide => "landslide",
        }
    }

    pub fn random() -> AreaEvent {
        let mut rng = rand::thread_rng();
        let events = vec![
            AreaEvent::Wildfire,
            AreaEvent::Flood,
            AreaEvent::Earthquake,
            AreaEvent::Avalanche,
            AreaEvent::Blizzard,
            AreaEvent::Landslide,
        ];
        let index = rng.gen_range(0..events.len());
        events[index].clone()
    }
}

#[derive(Clone, Debug)]
pub enum PlayerEvent {
    AnimalAttack(Animal),
    Dysentery,
    LightningStrike,
    Hypothermia,
    HeatStroke,
    Dehydration,
    Starvation,
    Poisoning,
    BrokenBone,
    Infection,
    Drowning,
}

impl FromStr for PlayerEvent {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.contains("animal attack") {
            let animal_name = s.split_whitespace().skip(2).map(|s| s.to_string()).collect::<Vec<String>>().join(" ");

            let animal = Animal::from_str(animal_name.as_str());
            if animal.is_ok() {
                return Ok(PlayerEvent::AnimalAttack(animal?))
            };
        }
        match s {
            "dysentery" => Ok(PlayerEvent::Dysentery),
            "lightning strike" => Ok(PlayerEvent::LightningStrike),
            "hypothermia" => Ok(PlayerEvent::Hypothermia),
            "heat stroke" => Ok(PlayerEvent::HeatStroke),
            "dehydration" => Ok(PlayerEvent::Dehydration),
            "starvation" => Ok(PlayerEvent::Starvation),
            "poisoning" => Ok(PlayerEvent::Poisoning),
            "broken bone" => Ok(PlayerEvent::BrokenBone),
            "infection" => Ok(PlayerEvent::Infection),
            "drowning" => Ok(PlayerEvent::Drowning),
            _ => Err(())
        }
    }
}

impl Display for PlayerEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PlayerEvent::AnimalAttack(animal) => write!(f, "animal attack {}", animal),
            PlayerEvent::Dysentery => write!(f, "dysentery"),
            PlayerEvent::LightningStrike => write!(f, "lightning strike"),
            PlayerEvent::Hypothermia => write!(f, "hypothermia"),
            PlayerEvent::HeatStroke => write!(f, "heat stroke"),
            PlayerEvent::Dehydration => write!(f, "dehydration"),
            PlayerEvent::Starvation => write!(f, "starvation"),
            PlayerEvent::Poisoning => write!(f, "poisoning"),
            PlayerEvent::BrokenBone => write!(f, "broken bone"),
            PlayerEvent::Infection => write!(f, "infection"),
            PlayerEvent::Drowning => write!(f, "drowning"),
        }
    }
}

impl PlayerEvent {
    pub fn as_str(&self) -> &str {
        match self {
            PlayerEvent::AnimalAttack(animal) => {
                let s = format!("animal attack {}", animal.as_str());
                Box::leak(s.into_boxed_str())
            },
            PlayerEvent::Dysentery => "dysentery",
            PlayerEvent::LightningStrike =>"lightning strike",
            PlayerEvent::Hypothermia => "hypothermia",
            PlayerEvent::HeatStroke => "heat stroke",
            PlayerEvent::Dehydration => "dehydration",
            PlayerEvent::Starvation => "starvation",
            PlayerEvent::Poisoning => "poisoning",
            PlayerEvent::BrokenBone => "broken bone",
            PlayerEvent::Infection => "infection",
            PlayerEvent::Drowning => "drowning",
        }
    }

    pub fn random() -> PlayerEvent {
        let mut rng = rand::thread_rng();
        let animal = Animal::random();
        let events = vec![
            PlayerEvent::AnimalAttack(animal),
            PlayerEvent::Dysentery,
            PlayerEvent::LightningStrike,
            PlayerEvent::Hypothermia,
            PlayerEvent::HeatStroke,
            PlayerEvent::Dehydration,
            PlayerEvent::Starvation,
            PlayerEvent::Poisoning,
            PlayerEvent::BrokenBone,
            PlayerEvent::Infection,
            PlayerEvent::Drowning,
        ];
        let index = rng.gen_range(0..events.len());
        events[index].clone()
    }
}
