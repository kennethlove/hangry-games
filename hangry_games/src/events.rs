use std::fmt::Display;
use std::str::FromStr;
use rand::Rng;

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
            AreaEvent::Wildfire => write!(f, "Wildfire"),
            AreaEvent::Flood => write!(f, "Flood"),
            AreaEvent::Earthquake => write!(f, "Earthquake"),
            AreaEvent::Avalanche => write!(f, "Avalanche"),
            AreaEvent::Blizzard => write!(f, "Blizzard"),
            AreaEvent::Landslide => write!(f, "Landslide"),
        }
    }
}

impl AreaEvent {
    pub fn as_str(&self) -> &str {
        match self {
            AreaEvent::Wildfire => "Wildfire",
            AreaEvent::Flood => "Flood",
            AreaEvent::Earthquake => "Earthquake",
            AreaEvent::Avalanche => "Avalanche",
            AreaEvent::Blizzard => "Blizzard",
            AreaEvent::Landslide => "Landslide",
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
