use crate::schema::action;
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

pub fn get_action(connection: &mut PgConnection, name: &str) -> Action {
    action::table
        .filter(action::name.ilike(name))
        .first(connection)
        .expect("Error loading action")
}
