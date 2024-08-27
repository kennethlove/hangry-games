use std::fmt::Display;

#[derive(Clone, Default, Debug, PartialEq)]
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
}

use super::models::area::Area as AreaModel;
impl From<AreaModel> for Area {
    fn from(area: AreaModel) -> Self {
        Self::from_str(area.name.as_str()).unwrap_or(Area::Cornucopia)
    }
}