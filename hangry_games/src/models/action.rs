use crate::schema::action;
use crate::establish_connection;
use diesel::prelude::*;

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = action)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Action {
    pub id: i32,
    pub name: String,
    pub description: String,
}

pub struct NewAction<'a> {
    pub name: &'a str,
    pub description: &'a str,
}

pub fn get_action(name: &str) -> Action {
    let connection = &mut establish_connection();
    let action = action::table
        .filter(action::name.ilike(format!("{}%", name)))
        .first(connection)
        .expect("Error loading action");
    action
}
