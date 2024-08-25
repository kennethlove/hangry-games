use crate::models::{Action, Tribute};
use crate::schema::tribute_action;
use diesel::prelude::*;

#[derive(Queryable, Selectable, Debug, Associations)]
#[diesel(table_name = tribute_action)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(Action, foreign_key = action_id))]
#[diesel(belongs_to(Tribute, foreign_key = tribute_id))]
pub struct TributeAction {
    pub id: i32,
    pub tribute_id: i32,
    pub action_id: i32,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = tribute_action)]
pub struct NewTributeAction {
    pub tribute_id: i32,
    pub action_id: i32,
}
