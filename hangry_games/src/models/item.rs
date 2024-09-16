use diesel::prelude::*;
use crate::schema::items;

#[derive((Queryable, Selectable, Debug, Clone, Associations))]
#[diesel(table_name = items)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(Area, foreign_key = area_id))]
#[diesel(belongs_to(Game, foreign_key = game_id))]
pub struct Item {
    pub id: i32,
    pub name: String,
    pub itemtype: String,
    pub area_id: Option<i32>,
    pub game_id: Option<i32>,
    pub weight: Option<i32>,
    pub strength_mod: Option<i32>,
    pub defense_mod: Option<i32>,
    pub health_mod: Option<i32>,
    pub speed_mod: Option<i32>,
    pub attack_mod: Option<i32>,
    pub durability: Option<i32>,
}