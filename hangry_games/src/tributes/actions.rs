use std::str::FromStr;

use diesel::deserialize::FromSql;
use crate::items::Item;
use crate::models::Action as ActionModel;
use crate::tributes::actors::Tribute;


#[derive(Clone, Debug, Default, PartialEq)]
pub enum TributeAction {
    #[default]
    None,
    Move(Option<String>),
    Rest,
    UseItem(Option<String>),
    Attack,
    Hide,
    TakeItem,
}

impl TributeAction {
    pub fn as_str(&self) -> &str {
        match self {
            TributeAction::None => "None",
            TributeAction::Move(_) => "Move",
            TributeAction::Rest => "Rest",
            TributeAction::UseItem(_) => "Use Item",
            TributeAction::Attack => "Attack",
            TributeAction::Hide => "Hide",
            TributeAction::TakeItem => "Take Item",
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
            "none" => Ok(TributeAction::None),
            "move" => Ok(TributeAction::Move(None)),
            "rest" => Ok(TributeAction::Rest),
            "use item" => Ok(TributeAction::UseItem(None)),
            "attack" => Ok(TributeAction::Attack),
            "hide" => Ok(TributeAction::Hide),
            "take item" => Ok(TributeAction::TakeItem),
            _ => Err(()),
        }
    }
}

impl From<&ActionModel> for TributeAction {
    fn from(value: &ActionModel) -> Self {
        let name = value.name.as_str();
        let action = Self::from_str(name);
        action.expect("Couldn't match that action")
    }
}

#[derive(Debug)]
pub enum AttackResult {
    AttackerWins,
    DefenderWins,
    Miss,
}

#[derive(Debug)]
pub enum AttackOutcome {
    Kill(Tribute, Tribute),
    Wound(Tribute, Tribute),
    Miss(Tribute, Tribute),
}