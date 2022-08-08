use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::classes::*;
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Entity {
    pub x: i32,
    pub y: i32,
    pub hp: i32,
    pub energy: i32,
    pub experience: i32,
    pub level: i32,
    pub id: u64,
    pub name: String,
    pub relative_x: i32,
    pub relative_y: i32,
    pub chunk_x: i32,
    pub chunk_y: i32,
    pub entity_type: String,
    pub stats: CharacterStats,
}
impl Entity {
    pub fn move_dir(&mut self, dir: String) {
        match dir.as_str() {
            "up" => {
                self.relative_y -= 1;
                self.y -= 1;
            }
            "down" => {
                self.relative_y += 1;
                self.y += 1;
            }
            "left" => {
                self.relative_x -= 1;
                self.x -= 1;
            }
            "right" => {
                self.relative_x += 1;
                self.x += 1;
            }
            _ => {}
        }
    }
}
impl Default for Entity {
    fn default() -> Entity {
        Entity {
            x: 1,
            y: 1,
            hp: 100,
            energy: 100,
            experience: 0,
            level: 1,
            relative_x: 1,
            relative_y: 1,
            chunk_x: 1,
            chunk_y: 1,
            entity_type: "no entity".to_string(),
            id: 0,
            name: "no name".to_string(),
            stats: CharacterStats::default(),
        }
    }
}
pub trait Scarab {
    fn scarab(x: i32, y: i32, relative_x: i32, relative_y: i32, chunk_x: i32, chunk_y: i32, id: u64, entity_type: String, name: String) -> Entity;
}
impl Scarab for Entity {
    fn scarab(x: i32, y: i32, relative_x: i32, relative_y: i32, chunk_x: i32, chunk_y: i32, id: u64, entity_type: String, name: String) -> Entity {
        Entity {
            x: x,
            y: y,
            hp: 100,
            energy: 100,
            experience: 0,
            level: 1,
            relative_x: relative_x,
            relative_y: relative_y,
            chunk_x: chunk_x,
            chunk_y: chunk_y,
            entity_type: entity_type,
            id: id,
            name: name,
            stats: CharacterStats::default(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Entities {
    pub entities: HashMap<u64, Entity>,
    pub x: i32,
    pub y: i32,
}
impl Default for Entities {
    fn default() -> Entities {
        Entities {
            entities: HashMap::new(),
            x: 0,
            y: 0,
        }
    }
}
