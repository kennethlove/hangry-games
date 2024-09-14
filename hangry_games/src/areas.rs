use std::fmt::Display;
use rand::Rng;

#[derive(Clone, Default, Debug, Eq, PartialEq)]
pub enum Area {
    #[default]
    Cornucopia,
    Northeast,
    Northwest,
    Southeast,
    Southwest,
}

impl Display for Area {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Area::Cornucopia => write!(f, "The Cornucopia"),
            Area::Northeast => write!(f, "North East"),
            Area::Northwest => write!(f, "North West"),
            Area::Southeast => write!(f, "South East"),
            Area::Southwest => write!(f, "South West"),
        }
    }
}

impl Area {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "the cornucopia" => Some(Area::Cornucopia),
            "cornucopia" => Some(Area::Cornucopia),
            "north east" => Some(Area::Northeast),
            "northeast" => Some(Area::Northeast),
            "ne" => Some(Area::Northeast),
            "north west" => Some(Area::Northwest),
            "northwest" => Some(Area::Northwest),
            "nw" => Some(Area::Northwest),
            "south east" => Some(Area::Southeast),
            "southeast" => Some(Area::Southeast),
            "se" => Some(Area::Southeast),
            "south west" => Some(Area::Southwest),
            "southwest" => Some(Area::Southwest),
            "sw" => Some(Area::Southwest),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            Area::Cornucopia => "The Cornucopia",
            Area::Northeast => "Northeast",
            Area::Northwest => "Northwest",
            Area::Southeast => "Southeast",
            Area::Southwest => "Southwest",
        }
    }

    pub fn neighbors(&self) -> Vec<Area> {
        match self {
            Area::Cornucopia => vec![Area::Northeast, Area::Northwest, Area::Southeast, Area::Southwest],
            Area::Northeast => vec![Area::Cornucopia, Area::Northwest],
            Area::Northwest => vec![Area::Cornucopia, Area::Northeast],
            Area::Southeast => vec![Area::Cornucopia, Area::Southwest],
            Area::Southwest => vec![Area::Cornucopia, Area::Southeast],
        }
    }

    pub fn random() -> Area {
        let mut rng = rand::thread_rng();
        match rng.gen_range(0..5) {
            1 => Area::Northeast,
            2 => Area::Northwest,
            3 => Area::Southeast,
            4 => Area::Southwest,
            _ => Area::Cornucopia,
        }
    }
}

use super::models::area::Area as AreaModel;
impl From<AreaModel> for Area {
    fn from(area: AreaModel) -> Self {
        Self::from_str(area.name.as_str()).unwrap_or(Area::Cornucopia)
    }
}

#[cfg(test)]
mod tests {
    use super::Area;

    #[test]
    fn area_from_str() {
        assert_eq!(Area::from_str("The Cornucopia"), Some(Area::Cornucopia));
        assert_eq!(Area::from_str("Cornucopia"), Some(Area::Cornucopia));
        assert_eq!(Area::from_str("North East"), Some(Area::Northeast));
        assert_eq!(Area::from_str("Northeast"), Some(Area::Northeast));
        assert_eq!(Area::from_str("NE"), Some(Area::Northeast));
        assert_eq!(Area::from_str("North West"), Some(Area::Northwest));
        assert_eq!(Area::from_str("Northwest"), Some(Area::Northwest));
        assert_eq!(Area::from_str("NW"), Some(Area::Northwest));
        assert_eq!(Area::from_str("South East"), Some(Area::Southeast));
        assert_eq!(Area::from_str("Southeast"), Some(Area::Southeast));
        assert_eq!(Area::from_str("SE"), Some(Area::Southeast));
        assert_eq!(Area::from_str("South West"), Some(Area::Southwest));
        assert_eq!(Area::from_str("Southwest"), Some(Area::Southwest));
        assert_eq!(Area::from_str("SW"), Some(Area::Southwest));
    }

    #[test]
    fn area_as_str() {
        assert_eq!(Area::Cornucopia.as_str(), "The Cornucopia");
        assert_eq!(Area::Northeast.as_str(), "Northeast");
        assert_eq!(Area::Northwest.as_str(), "Northwest");
        assert_eq!(Area::Southeast.as_str(), "Southeast");
        assert_eq!(Area::Southwest.as_str(), "Southwest");
    }

    #[test]
    fn random_area() {
        let area = Area::random();
        assert!(
            area == Area::Cornucopia ||
            area == Area::Northeast ||
            area == Area::Northwest ||
            area == Area::Southeast ||
            area == Area::Southwest
        );
    }

    #[test]
    fn area_neighbors() {
        assert_eq!(Area::Cornucopia.neighbors(), vec![Area::Northeast, Area::Northwest, Area::Southeast, Area::Southwest]);
        assert_eq!(Area::Northeast.neighbors(), vec![Area::Cornucopia, Area::Northwest]);
        assert_eq!(Area::Northwest.neighbors(), vec![Area::Cornucopia, Area::Northeast]);
        assert_eq!(Area::Southeast.neighbors(), vec![Area::Cornucopia, Area::Southwest]);
        assert_eq!(Area::Southwest.neighbors(), vec![Area::Cornucopia, Area::Southeast]);
    }
}