use std::fmt::Display;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub enum Status {
    #[default]
    Alive,
    FreshlyDead,
    Dead,
}

impl Status {
    pub fn as_str(&self) -> &str {
        match self {
            Status::Alive => "Alive",
            Status::FreshlyDead => "FreshlyDead",
            Status::Dead => "Dead",
        }
    }
}

impl From<&str> for Status {
    fn from(s: &str) -> Self {
        match s {
            "Alive" => Status::Alive,
            "ALIVE" => Status::Alive,
            "alive" => Status::Alive,
            "FreshlyDead" => Status::FreshlyDead,
            "FRESHLY-DEAD" => Status::FreshlyDead,
            "freshly-dead" => Status::FreshlyDead,
            "Dead" => Status::Dead,
            "DEAD" => Status::Dead,
            "dead" => Status::Dead,
            _ => Status::Alive,
        }
    }
}

impl From<String> for Status {
    fn from(s: String) -> Self {
        Status::from(s.as_str())
    }
}

impl Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Status::Alive => "Alive".to_string(),
            Status::FreshlyDead => "FreshlyDead".to_string(),
            Status::Dead => "Dead".to_string(),
        };
        write!(f, "{}", str)
    }
}