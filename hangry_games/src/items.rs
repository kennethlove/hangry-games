use crate::areas::Area;
use crate::games::Game;
use crate::models::item::{create_item, Item as ItemModel, NewItem};
use crate::models::{get_area_by_id, get_game_by_id};
use rand::Rng;
use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Item {
    pub id: Option<i32>,
    pub name: String,
    pub item_type: ItemType,
    pub area_id: Option<i32>,
    pub game_id: Option<i32>,
    pub quantity: i32,
    pub attribute: Attribute,
    pub effect: i32,
}

impl Item {
    pub fn area(&self) -> Area {
        Area::from(get_area_by_id(self.area_id).unwrap())
    }

    pub fn game(&self) -> Game {
        Game::from(get_game_by_id(self.game_id.unwrap()).unwrap())
    }

    pub fn get_item_by_name(name: &str) -> Option<Item> {
        Some(Item::from(ItemModel::get_by_name(name.to_string())))
    }

    pub fn new(name: &str, item_type: i32, quantity: i32, attribute: &str, effect: i32) -> Item {
        let new_item = NewItem {
            name: name.to_string(),
            item_type: ItemType::from_int(item_type).to_string(),
            area_id: None,
            game_id: None,
            quantity,
            attribute: Attribute::from_str(attribute).unwrap().to_string(),
            effect,
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
            item_type: ItemType::from_str(item.item_type.as_str()).unwrap(),
            area_id: item.area_id,
            game_id: item.game_id,
            quantity: item.quantity,
            attribute: Attribute::from_str(item.attribute.as_str()).unwrap(),
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
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let item = ItemModel::get_by_name(s.to_string());
        Ok(Item::from(item))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
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

    pub fn from_int(i: i32) -> ItemType {
        match i {
            0 => ItemType::Consumable,
            1 => ItemType::Weapon,
            _ => panic!("Invalid item type"),
        }
    }

    pub fn to_int(&self) -> i32 {
        match self {
            ItemType::Consumable => 0,
            ItemType::Weapon => 1,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Attribute {
    Health, // Heals health
    Sanity, // Heals sanity
    Movement, // Increases movement
    Bravery, // Increases bravery
    Speed, // Increases speed
    Strength, // Increases damage done, i.e. weapon
    Defense, // Reduces damage taken
}

impl Display for Attribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Attribute::Health => write!(f, "Health"),
            Attribute::Sanity => write!(f, "Sanity"),
            Attribute::Movement => write!(f, "Movement"),
            Attribute::Bravery => write!(f, "Bravery"),
            Attribute::Speed => write!(f, "Speed"),
            Attribute::Strength => write!(f, "Strength"),
            Attribute::Defense => write!(f, "Defense"),
        }
    }
}

impl FromStr for Attribute {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "health" => Ok(Attribute::Health),
            "sanity" => Ok(Attribute::Sanity),
            "movement" => Ok(Attribute::Movement),
            "bravery" => Ok(Attribute::Bravery),
            "speed" => Ok(Attribute::Speed),
            "strength" => Ok(Attribute::Strength),
            "defense" => Ok(Attribute::Defense),
            _ => Err("Invalid attribute"),
        }
    }
}