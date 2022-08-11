use crate::classes::*;
use crate::tiles::*;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Unit {
    pub name: String,
    pub hp: i32,
    pub energy: i32,
    pub profession: String,
    pub stats: UnitStats,
}
pub trait Soldier {
    fn soldier() -> Unit;
}
impl Soldier for Unit {
    fn soldier() -> Unit {
        let mut rng = rand::thread_rng();
        let id: u64 = rng.gen::<u64>();
        Unit {
            name: get_name(),
            profession: "soldier".to_string(),
            hp: 100,
            energy: 100,
            stats: UnitStats::default(),
        }
    }
}
pub trait Gatherer {
    fn gatherer() -> Unit;
}
impl Gatherer for Unit {
    fn gatherer() -> Unit {
        let mut rng = rand::thread_rng();
        let id: u64 = rng.gen::<u64>();
        Unit {
            name: get_name(),
            profession: "gatherer".to_string(),
            hp: 100,
            energy: 100,
            stats: UnitStats::default(),
        }
    }
}
pub trait Worker {
    fn worker() -> Unit;
}
impl Worker for Unit {
    fn worker() -> Unit {
        let mut rng = rand::thread_rng();
        let id: u64 = rng.gen::<u64>();
        Unit {
            name: get_name(),
            profession: "worker".to_string(),
            hp: 100,
            energy: 100,
            stats: UnitStats::default(),
        }
    }
}
pub trait Crafter {
    fn crafter() -> Unit;
}
impl Crafter for Unit {
    fn crafter() -> Unit {
        let mut rng = rand::thread_rng();
        let id: u64 = rng.gen::<u64>();
        Unit {
            name: get_name(),
            profession: "crafter".to_string(),
            hp: 100,
            energy: 100,
            stats: UnitStats::default(),
        }
    }
}
pub trait GenerateDefault {
    fn generate_default_units() -> HashMap<u64, Unit>;
}
impl GenerateDefault for Entity {
    fn generate_default_units() -> HashMap<u64, Unit> {
        let mut units = HashMap::new();
        let mut rng = rand::thread_rng();
        for i in 0..4 {
            let id: u64 = rng.gen::<u64>();
            units.insert(id, Unit::soldier());
        }
        for i in 0..4 {
            let id: u64 = rng.gen::<u64>();
            units.insert(id, Unit::gatherer());
        }
        for i in 0..4 {
            let id: u64 = rng.gen::<u64>();
            units.insert(id, Unit::worker());
        }
        for i in 0..4 {
            let id: u64 = rng.gen::<u64>();
            units.insert(id, Unit::crafter());
        }
        return units;
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Entity {
    pub x: i32,
    pub y: i32,
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
    pub resources: HashMap<String, i32>,
    pub units: HashMap<u64, Unit>,
    pub standing_tile: Tile,
    pub alive: bool,
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
    pub fn damage(&mut self, damage_type: String) {
        let mut rng = rand::thread_rng();
        for (_k, u) in self.units.iter_mut() {
            let dmg = rng.gen_range(0..10);
            u.hp -= dmg;
        }
        for (k, u) in self.units.clone().iter_mut() {
            if u.hp < 0 {
                self.units.remove(&k);
            }
        }
    }
}
impl Default for Entity {
    fn default() -> Entity {
        Entity {
            x: 1,
            y: 1,
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
            units: HashMap::new(),
            resources: HashMap::new(),
            standing_tile: Tile::default(),
            alive: true,
        }
    }
}
pub trait Scarab {
    fn scarab(
        x: i32,
        y: i32,
        relative_x: i32,
        relative_y: i32,
        chunk_x: i32,
        chunk_y: i32,
        id: u64,
        entity_type: String,
        name: String,
    ) -> Entity;
}
impl Scarab for Entity {
    fn scarab(
        x: i32,
        y: i32,
        relative_x: i32,
        relative_y: i32,
        chunk_x: i32,
        chunk_y: i32,
        id: u64,
        entity_type: String,
        name: String,
    ) -> Entity {
        Entity {
            x: x,
            y: y,
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
            units: Entity::generate_default_units(),
            resources: HashMap::new(),
            standing_tile: Tile::default(),
            alive: true,
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
fn get_name() -> String {
    let mut rng = rand::thread_rng();
    let filename = "text/words.txt";
    let contents = fs::read_to_string(filename).expect("Failed to read file");
    let content_vec: Vec<&str> = contents.split("\n").collect();
    let mut word: String = content_vec[rng.gen_range(0..content_vec.len() - 1)]
        .chars()
        .rev()
        .collect::<String>();
    word.remove(word.len() - 1);
    let letters = vec![
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    ];
    let mut char_1 = letters[rng.gen_range(0..letters.len() - 1)];
    if letters.len() < 2 {
        char_1 = 'a';
    }
    word.push(char_1);
    if word.len() - 1 != 0 {
        word.remove(rng.gen_range(0..word.len() - 1));
    } else {
        word.remove(0);
    }
    word = word.to_lowercase();
    let first_letter = word.chars().nth(0).unwrap();
    word.replace_range(
        0..1,
        &first_letter.to_uppercase().nth(0).unwrap().to_string(),
    );
    return word.to_string();
}
