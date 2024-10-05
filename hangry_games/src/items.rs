use crate::areas::Area;
use crate::games::Game;
use crate::models::{get_area_by_id, get_game_by_id};
use crate::models::item::{create_item, Item as ItemModel, NewItem};
use std::fmt::Display;
use std::str::FromStr;
use rand::Rng;

#[derive(Debug, Clone, PartialEq, Eq)]]
pub struct Item {
    pub id: Option<i32>,
    pub name: String,
    pub item_type: String,
    pub area_id: Option<i32>,
    pub game_id: Option<i32>,
    pub quantity: i32,
    pub attribute: String,
    pub effect: i32,
}

impl Item {
    pub fn area(&self) -> Area {
        Area::from(get_area_by_id(self.area_id))
    }

    pub fn game(&self) -> Game {
        Game::from(get_game_by_id(self.game_id.unwrap()))
    }

    pub fn get_item_by_name(name: &str) -> Option<Item> {
        Some(Item::from(ItemModel::get_by_name(name.to_string())))
    }

    pub fn new(name: &str, item_type: &str, quantity: i32, attribute: &str, effect: i32) -> Item {
        let new_item = NewItem {
            name: name.to_string(),
            item_type: item_type.to_string(),
            quantity,
            attribute: attribute.to_string(),
            effect,
            area_id: None,
            game_id: None,
        };
        let item = create_item(new_item);
        Item::from(item)
    }
}

impl From<ItemModel> for Item {
    fn from(item: ItemModel) -> Self {
        Item {
            id: Some(item.id),
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

impl Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl FromStr for Item {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let item = ItemModel::get_item_by_name(s);
        match item {
            Some(item) => Ok(item),
            None => Err("Item not found"),
        }
    }
}

pub enum ItemType {
    Consumable,
    Weapon,
}

impl Display for ItemType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ItemType::Consumable => write!(f, "Consumable"),
            ItemType::Weapon => write!(f, "Weapon"),
        }
    }
}

impl FromStr for ItemType {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "consumable" => Ok(ItemType::Consumable),
            "weapon" => Ok(ItemType::Weapon),
            _ => Err("Invalid item type"),
        }
    }
}

impl ItemType {
    pub fn random() -> ItemType {
        let mut rng = rand::thread_rng();
        match rng.gen_bool(0.5) {
            true => ItemType::Consumable,
            false => ItemType::Weapon,
        }
    }
}