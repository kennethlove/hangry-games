use crate::establish_connection;
use crate::models::{Area, Game};
use crate::schema::item;
use diesel::prelude::*;

#[derive(Queryable, Selectable, Debug, Clone, Associations)]
#[diesel(table_name = item)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(Area, foreign_key = area_id))]
#[diesel(belongs_to(Game, foreign_key = game_id))]
pub struct Item {
    pub id: i32,
    pub name: String,
    pub item_type: String,
    pub area_id: Option<i32>,
    pub game_id: Option<i32>,
    pub quantity: i32,
    pub attribute: String,
    pub effect: i32,
}

impl Item {
    pub fn get_all() -> Vec<Item> {
        let connection = &mut establish_connection();
        item::table
            .select(item::all_columns)
            .load::<Item>(connection)
            .expect("Error loading items")
    }

    pub fn get_by_id(id: i32) -> Item {
        let connection = &mut establish_connection();
        item::table.find(id).first(connection).expect("Error loading item")
    }

    pub fn get_by_name(name: String) -> Item {
        let connection = &mut establish_connection();
        item::table.filter(item::name.eq(name)).first(connection).expect("Error loading item")
    }

    pub fn get_by_type(item_type: String) -> Vec<Item> {
        let connection = &mut establish_connection();
        item::table.filter(item::item_type.eq(item_type)).load::<Item>(connection).expect("Error loading items")
    }

    pub fn get_by_area(area_id: i32) -> Vec<Item> {
        let connection = &mut establish_connection();
        item::table.filter(item::area_id.eq(area_id)).load::<Item>(connection).expect("Error loading items")
    }

    pub fn get_by_game(game_id: i32) -> Vec<Item> {
        let connection = &mut establish_connection();
        item::table.filter(item::game_id.eq(game_id)).load::<Item>(connection).expect("Error loading items")
    }
}

impl From<crate::items::Item> for Item {
    fn from(item: crate::items::Item) -> Self {
        Item {
            id: item.id.unwrap(),
            name: item.name,
            item_type: item.item_type.to_string(),
            area_id: item.area_id,
            game_id: item.game_id,
            quantity: item.quantity,
            attribute: item.attribute.to_string(),
            effect: item.effect,
        }
    }
}

#[derive(Insertable, Debug)]
#[diesel(table_name = item)]
pub struct NewItem {
    pub name: String,
    pub item_type: String,
    pub area_id: Option<i32>,
    pub game_id: Option<i32>,
    pub quantity: i32,
    pub attribute: String,
    pub effect: i32,
}

pub fn create_item(new_item: NewItem) -> Item {
    use crate::schema::item;
    let connection = &mut establish_connection();
    diesel::insert_into(item::table)
        .values(&new_item)
        .get_result(connection)
        .unwrap()
}

impl From<Item> for NewItem {
    fn from(item: Item) -> Self {
        NewItem {
            name: item.name,
            item_type: item.item_type,
            area_id: item.area_id,
            game_id: item.game_id,
            quantity: item.quantity,
            attribute: item.attribute,
            effect: item.effect,
        }
    }
}

impl From<crate::items::Item> for NewItem {
    fn from(item: crate::items::Item) -> Self {
        NewItem {
            name: item.name,
            item_type: item.item_type.to_string(),
            area_id: item.area_id,
            game_id: item.game_id,
            quantity: item.quantity,
            attribute: item.attribute.to_string(),
            effect: item.effect,
        }
    }
}