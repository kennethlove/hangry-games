use crate::establish_connection;
use crate::models::{get_game_by_id, get_game_tributes, Action, Tribute};
use crate::schema::action;
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
    pub target: Option<String>,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = tribute_action)]
pub struct NewTributeAction {
    pub tribute_id: i32,
    pub action_id: i32,
    pub target: Option<String>,
}

impl TributeAction {
    pub fn create(tribute_id: i32, action_id: i32, target: Option<String>) -> TributeAction {
        let connection = &mut establish_connection();
        let new_tribute_action = NewTributeAction { tribute_id, action_id, target };

        diesel::insert_into(tribute_action::table)
            .values(&new_tribute_action)
            .returning(TributeAction::as_returning())
            .get_result(connection)
            .expect("Error saving new tribute action")
    }

    pub fn get_all_for_tribute(tribute_id: i32) -> Vec<TributeAction> {
        let connection = &mut establish_connection();
        tribute_action::table
            .filter(tribute_action::tribute_id.eq(tribute_id))
            .select(tribute_action::all_columns)
            .load(connection)
            .expect("Error loading tribute actions")
    }

    pub fn action(&self) -> Action {
        let connection = &mut establish_connection();
        action::table.filter(action::id.eq(self.action_id))
            .select(action::all_columns)
            .first(connection)
            .expect("Error loading action")
    }
}

pub fn take_action(tribute: &Tribute, action: &Action, target: Option<String>) -> TributeAction {
    TributeAction::create(tribute.id, action.id, target.clone())
}

pub fn delete_game_tribute_actions(game_id: i32) {
    let connection = &mut establish_connection();
    let game = get_game_by_id(game_id).expect("Game not found");
    let tributes = get_game_tributes(&game);
    let tributes = tributes.iter().map(|t| t.id);

    let count = diesel::delete(tribute_action::table.filter(tribute_action::tribute_id.eq_any(tributes)))
        .execute(connection)
        .expect("Error deleting tribute actions");
    println!("Deleted {} tribute actions", count);
}