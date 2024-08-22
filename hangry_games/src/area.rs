use std::fmt::Display;

#[derive(Clone, Default, Debug, PartialEq)]
pub enum Area {
    #[default]
    Cornucopia,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
}

impl Display for Area {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Area::Cornucopia => write!(f, "The Cornucopia"),
            Area::NorthEast => write!(f, "North East"),
            Area::NorthWest => write!(f, "North West"),
            Area::SouthEast => write!(f, "South East"),
            Area::SouthWest => write!(f, "South West"),
        }
    }
}

impl Area {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "the cornucopia" => Some(Area::Cornucopia),
            "cornucopia" => Some(Area::Cornucopia),
            "north east" => Some(Area::NorthEast),
            "northeast" => Some(Area::NorthEast),
            "ne" => Some(Area::NorthEast),
            "north west" => Some(Area::NorthWest),
            "northwest" => Some(Area::NorthWest),
            "nw" => Some(Area::NorthWest),
            "south east" => Some(Area::SouthEast),
            "southeast" => Some(Area::SouthEast),
            "se" => Some(Area::SouthEast),
            "south west" => Some(Area::SouthWest),
            "southwest" => Some(Area::SouthWest),
            "sw" => Some(Area::SouthWest),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            Area::Cornucopia => "The Cornucopia",
            Area::NorthEast => "North East",
            Area::NorthWest => "North West",
            Area::SouthEast => "South East",
            Area::SouthWest => "South West",
        }
    }

    pub fn neighbors(&self) -> Vec<Area> {
        match self {
            Area::Cornucopia => vec![Area::NorthEast, Area::NorthWest, Area::SouthEast, Area::SouthWest],
            Area::NorthEast => vec![Area::Cornucopia, Area::NorthWest],
            Area::NorthWest => vec![Area::Cornucopia, Area::NorthEast],
            Area::SouthEast => vec![Area::Cornucopia, Area::SouthWest],
            Area::SouthWest => vec![Area::Cornucopia, Area::SouthEast],
        }
    }
}