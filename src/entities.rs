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
    pub items: HashMap<String, Item>,
}
impl Default for Unit {
    fn default() -> Unit {
        Unit {
            name: "default unit".to_string(),
            hp: 1,
            energy: 1,
            profession: "none".to_string(),
            stats: UnitStats::default(),
            items: HashMap::new(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Item {
    pub name: String,
    pub quantity: u8,
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
            items: HashMap::from([
                (
                    "flax shirt".to_string(),
                    Item {
                        name: "flax shirt".to_string(),
                        quantity: 1,
                    },
                ),
                (
                    "flax trousers".to_string(),
                    Item {
                        name: "flax trousers".to_string(),
                        quantity: 1,
                    },
                ),
                (
                    "wooden spear".to_string(),
                    Item {
                        name: "wooden spear".to_string(),
                        quantity: 1,
                    },
                ),
            ]),
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
            items: HashMap::from([
                (
                    "flax shirt".to_string(),
                    Item {
                        name: "flax shirt".to_string(),
                        quantity: 1,
                    },
                ),
                (
                    "flax trousers".to_string(),
                    Item {
                        name: "flax trousers".to_string(),
                        quantity: 1,
                    },
                ),
            ]),
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
            items: HashMap::from([
                (
                    "flax shirt".to_string(),
                    Item {
                        name: "flax shirt".to_string(),
                        quantity: 1,
                    },
                ),
                (
                    "flax trousers".to_string(),
                    Item {
                        name: "flax trousers".to_string(),
                        quantity: 1,
                    },
                ),
                (
                    "shovel".to_string(),
                    Item {
                        name: "shovel".to_string(),
                        quantity: 1,
                    },
                ),
            ]),
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
            items: HashMap::from([
                (
                    "flax shirt".to_string(),
                    Item {
                        name: "flax shirt".to_string(),
                        quantity: 1,
                    },
                ),
                (
                    "flax trousers".to_string(),
                    Item {
                        name: "flax trousers".to_string(),
                        quantity: 1,
                    },
                ),
                (
                    "needle".to_string(),
                    Item {
                        name: "needle".to_string(),
                        quantity: 1,
                    },
                ),
                (
                    "knife".to_string(),
                    Item {
                        name: "knife".to_string(),
                        quantity: 1,
                    },
                ),
            ]),
        }
    }
}
pub trait Donkey {
    fn donkey() -> Unit;
}
impl Donkey for Unit {
    fn donkey() -> Unit {
        let mut rng = rand::thread_rng();
        let id: u64 = rng.gen::<u64>();
        Unit {
            name: get_name(),
            profession: "carrier".to_string(),
            hp: 100,
            energy: 100,
            stats: UnitStats::default(),
            items: HashMap::new(),
        }
    }
}
pub trait CoyoteUnit {
    fn coyote() -> Unit;
}
impl CoyoteUnit for Unit {
    fn coyote() -> Unit {
        let mut rng = rand::thread_rng();
        let id: u64 = rng.gen::<u64>();
        Unit {
            name: "coyote".to_string(),
            profession: "coyote".to_string(),
            hp: 25,
            energy: 100,
            stats: UnitStats::default(),
            items: HashMap::new(),
        }
    }
}
pub trait GenerateDefault {
    fn generate_default_units() -> HashMap<u64, Unit>;
    fn generate_units(unit_type: String) -> HashMap<u64, Unit>;
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
        for i in 0..4 {
            let id: u64 = rng.gen::<u64>();
            units.insert(id, Unit::donkey());
        }
        return units;
    }
    fn generate_units(unit_type: String) -> HashMap<u64, Unit> {
        let mut units = HashMap::new();
        let mut rng = rand::thread_rng();
        for i in 0..12 {
            let id: u64 = rng.gen::<u64>();
            units.insert(id, Unit::coyote());
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
    pub target_entity_id: u64,
}
impl Entity {
    pub fn move_dir(&mut self, dir: String) {
        if !self.alive {
            return;
        }
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
        if damage_type == "starve".to_string() || damage_type == "water".to_string() {
            if self.entity_type == "coyote" {
                return;
            }
        }
        let mut min_dmg = 0;
        let mut max_dmg = 10;
        match damage_type.as_str() {
            "Shout" => {
                min_dmg = 5;
                max_dmg = 10;
            }
            "Roll Attack" => {
                min_dmg = 10;
                max_dmg = 15;

            }
            "Charge" => {
                min_dmg = 15;
                max_dmg = 20;

            }
            "Earth Spike" => {
                min_dmg = 20;
                max_dmg = 25;

            }
            "Earthquake" => {
                min_dmg = 25;
                max_dmg = 30;

            }
            _ => {}
        }
        
        let mut rng = rand::thread_rng();
        for (_k, u) in self.units.iter_mut() {
            let dmg = rng.gen_range(min_dmg..max_dmg);
            u.hp -= dmg;
        }
        for (k, u) in self.units.clone().iter_mut() {
            if u.hp < 0 {
                self.units.remove(&k);
            }
        }
    }
    pub fn damage_by_entity(&mut self, damage_type: String, entity: Entity) {
        let mut weapon_power = 0;
        for u in entity.units.values() {
            for i in u.items.values() {
                if i.name == "wooden spear".to_string() {
                    weapon_power += 1;
                }
            }
            if u.name == "coyote" {
                weapon_power += 1;
            }
        }
        let mut rng = rand::thread_rng();
        for (_k, u) in self.units.iter_mut() {
            let dmg = rng.gen_range(weapon_power..(weapon_power + 5));
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
            target_entity_id: 1,
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
            target_entity_id: 1,
            name: name,
            stats: CharacterStats::default(),
            units: Entity::generate_default_units(),
            resources: HashMap::new(),
            standing_tile: Tile::default(),
            alive: true,
        }
    }
}
pub trait Coyote {
    fn coyote(
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
impl Coyote for Entity {
    fn coyote(
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
            target_entity_id: 1,
            id: id,
            name: name,
            stats: CharacterStats::default(),
            units: Entity::generate_units("coyote".to_string()),
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
