use std::str::FromStr;

use diesel::deserialize::FromSql;

#[derive(Clone, Debug, Default, PartialEq)]
pub enum TributeAction {
    #[default]
    Idle,
    Move,
    Rest,
    UseItem,
    Attack,
    Hide,
}

impl TributeAction {
    pub fn as_str(&self) -> &str {
        match self {
            TributeAction::Idle => "Idle",
            TributeAction::Move => "Move",
            TributeAction::Rest => "Rest",
            TributeAction::UseItem => "Use Item",
            TributeAction::Attack => "Attack",
            TributeAction::Hide => "Hide",
        }
    }
}

impl FromSql<diesel::sql_types::Text, diesel::pg::Pg> for TributeAction {
    fn from_sql(bytes: diesel::pg::PgValue<'_>) -> diesel::deserialize::Result<Self> {
        let s = <String as FromSql<diesel::sql_types::Text, diesel::pg::Pg>>::from_sql(bytes)?;
        TributeAction::from_str(&s).map_err(|_| "Invalid TributeActions".into())
    }
}

impl FromStr for TributeAction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "idle" => Ok(TributeAction::Idle),
            "move" => Ok(TributeAction::Move),
            "rest" => Ok(TributeAction::Rest),
            "use item" => Ok(TributeAction::UseItem),
            "attack" => Ok(TributeAction::Attack),
            "hide" => Ok(TributeAction::Hide),
            _ => Err(()),
        }
    }
}

use crate::models::Action as ActionModel;
impl From<&ActionModel> for TributeAction {
    fn from(value: &ActionModel) -> Self {
        let name = value.name.as_str();
        let action = Self::from_str(name);
        action.expect("Couldn't match that action")
    }
}

