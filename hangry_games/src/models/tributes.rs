use diesel::prelude::*;
use crate::schema::tributes;

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = tributes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Tribute {
    pub id: i32,
    pub name: String,
    pub health: i32,
    pub sanity: i32,
    pub movement: i32,
    pub is_alive: bool,
    pub district: i32,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = tributes)]
pub struct NewTribute<'a> {
    pub name: &'a str,
    pub district: i32,
}
