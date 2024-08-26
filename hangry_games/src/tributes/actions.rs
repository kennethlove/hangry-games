use std::str::FromStr;

use diesel::deserialize::FromSql;

#[derive(Clone, Debug, Default, PartialEq)]
pub enum TributeActions {
    #[default]
    Idle,
    Move,
    Rest,
    UseItem,
    Attack,
    Hide,
}

impl TributeActions {
    pub fn as_str(&self) -> &str {
        match self {
            TributeActions::Idle => "Idle",
            TributeActions::Move => "Move",
            TributeActions::Rest => "Rest",
            TributeActions::UseItem => "Use Item",
            TributeActions::Attack => "Attack",
            TributeActions::Hide => "Hide",
        }
    }
}

impl FromSql<diesel::sql_types::Text, diesel::pg::Pg> for TributeActions {
    fn from_sql(bytes: diesel::pg::PgValue<'_>) -> diesel::deserialize::Result<Self> {
        let s = <String as FromSql<diesel::sql_types::Text, diesel::pg::Pg>>::from_sql(bytes)?;
        TributeActions::from_str(&s).map_err(|_| "Invalid TributeActions".into())
    }
}

impl FromStr for TributeActions {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Idle" => Ok(TributeActions::Idle),
            "Move" => Ok(TributeActions::Move),
            "Rest" => Ok(TributeActions::Rest),
            "Use Item" => Ok(TributeActions::UseItem),
            "Attack" => Ok(TributeActions::Attack),
            "Hide" => Ok(TributeActions::Hide),
            _ => Err(()),
        }
    }
}
