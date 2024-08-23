use diesel::prelude::*;
use crate::schema::areas;

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = areas)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Area {
    pub id: i32,
    pub name: String,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = areas)]
pub struct NewArea<'a> {
    pub name: &'a str,
}