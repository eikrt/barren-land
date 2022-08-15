use crate::classes::*;
use crate::entities::*;
use crate::server::*;
use crate::tiles::*;
use crate::world::*;
use actix_web::*;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::io::prelude::*;
use std::sync::Mutex;
pub struct MoveSet {
    directions: Vec<String>,
}
impl Default for MoveSet {
    fn default() -> MoveSet {
        MoveSet {
            directions: Vec::from([
                "up".to_string(),
                "down".to_string(),
                "left".to_string(),
                "right".to_string(),
            ]),
        }
    }
}
impl MoveSet {
    fn get_direction(&self) -> String {
        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..self.directions.len());
        return self.directions[index].clone();
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ActionQueue {
    pub queue: Vec<PostData>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PostData {
    pub params: HashMap<String, String>,
}
pub fn queue_to_object(data: web::Json<PostData>) -> PostData {
    PostData {
        params: data.params.clone(),
    }
}
pub fn add_to_queue(q: web::Data<Mutex<ActionQueue>>, data: PostData) {
    let default_queue = ActionQueue { queue: Vec::new() };
    let default_mutex = Mutex::new(default_queue);
    let default_data = web::Data::new(default_mutex);
    let default_value = default_data.lock().unwrap();
    let state = &mut *(q.lock().unwrap_or(default_value));
    state.queue.push(data);
}
pub fn process_entities(q: web::Data<Mutex<ActionQueue>>) {
    let state = &mut *(q.lock().unwrap());
    let client_ids = open_client_ids_to_struct();
    let w_p = open_world_properties_to_struct();
    for (_username, ci) in client_ids.ids.iter() {
        let mut lx = ci.chunk_x - 1;
        let mut ly = ci.chunk_y - 1;
        let mut rx = ci.chunk_x + 2;
        let mut ry = ci.chunk_y + 2;
        if lx < 0 {
            lx = 0;
        }
        if ly < 0 {
            ly = 0;
        }
        if rx > w_p.world_width as i32 - 1 {
            rx = w_p.world_width as i32 - 1;
        }
        if ry > w_p.world_height as i32 - 1 {
            ry = w_p.world_height as i32 - 1;
        }
        for cy in ly..ry {
            for cx in lx..rx {
                let mut chunk_entities = open_entities_as_struct(cx, cy).unwrap();
                let chunk_entities_clone = chunk_entities.clone();

                for (_e_id, entity) in chunk_entities.entities.iter_mut() {
                    let moveset = MoveSet::default();
                    match entity.entity_type.as_str() {
                        "coyote" => {
                            let mut action = PostData {
                                params: HashMap::from([
                                    ("command".to_string(), "move".to_string()),
                                    ("move_dir".to_string(), moveset.get_direction()),
                                    ("id".to_string(), entity.id.to_string()),
                                    ("chunk_x".to_string(), format!("{}", cx).to_string()),
                                    ("chunk_y".to_string(), format!("{}", cy).to_string()),
                                ]),
                            };
                            for (_, other_entity) in chunk_entities_clone.entities.iter() {
                                let dist = ((other_entity.y as f32 - entity.y as f32).powf(2.0)
                                    + (other_entity.x as f32 - entity.x as f32).powf(2.0))
                                .sqrt();
                                if dist < 2.0 && entity.id != other_entity.id {
                                    entity.target_entity_id = other_entity.id;
                                    action = PostData {
                                        params: HashMap::from([
                                            ("command".to_string(), "attack".to_string()),
                                            ("type".to_string(), "auto".to_string()),
                                            ("id".to_string(), format!("{}", entity.id)),
                                            ("target_id".to_string(), format!("{}", entity.target_entity_id)),
                                            (
                                                "chunk_x".to_string(),
                                                format!("{}", entity.chunk_x).to_string(),
                                            ),
                                            (
                                                "chunk_y".to_string(),
                                                format!("{}", entity.chunk_y).to_string(),
                                            ),
                                        ]),
                                    };
                                    break;
                                }
                            }
                            state.queue.push(action);
                        }
                        _ => {}
                    };
                }
            }
        }
    }
}
pub fn execute_queue(q: web::Data<Mutex<ActionQueue>>) {
    let state = &mut *(q.lock().unwrap());
    if state.queue.len() > 0 {
        println!("in queue: {}", state.queue.len());
        let last = &state.queue[0];
        println!("{:?}", last);
        execute_action(last.clone());
        state.queue.remove(0);
    }
}
pub fn execute_action(action: PostData) {
    let mut rng = rand::thread_rng();
    let w_p = open_world_properties_to_struct();
    let action_chunk_x = action.params["chunk_x"].parse::<i32>().unwrap();
    let action_chunk_y = action.params["chunk_y"].parse::<i32>().unwrap();
    let id = action.params["id"].parse::<u64>().unwrap();
    let mut action_entities =
        match open_entities_as_struct(action_chunk_x as i32, action_chunk_y as i32) {
            Ok(o) => o,
            Err(e) => return,
        };
    let mut action_entities_clone = action_entities.clone();
    let mut action_tiles = match open_tiles_as_struct(action_chunk_x as i32, action_chunk_y as i32)
    {
        Ok(o) => o,
        Err(e) => return,
    };
    let mut remove_entity = false;
    let mut add_entity = false;
    let mut chunk_x_to_add = action_chunk_x;
    let mut chunk_y_to_add = action_chunk_y;
    let mut remove_id = id;
    match action.params["command"].as_str() {
        "spawn" => {
            let action_x: i32 = action.params["x"].parse::<i32>().unwrap();
            let action_y: i32 = action.params["y"].parse::<i32>().unwrap();

            let units = Entity::generate_default_units();
            let mut entity = Entity {
                x: w_p.chunk_size as i32 * action_chunk_x + action_x,
                y: w_p.chunk_size as i32 * action_chunk_y as i32 + action_y,
                alive: true,
                relative_x: action_x,
                relative_y: action_y,
                experience: 0,
                level: 1,
                chunk_x: action_chunk_x as i32,
                chunk_y: action_chunk_y as i32,
                entity_type: "ogre".to_string(),
                name: action.params["name"].clone(),
                id: id,
                target_entity_id: 1,
                stats: CharacterStats::ogre(),
                units: units,
                resources: HashMap::from([("wood".to_string(), 0), ("food".to_string(), 10)]),
                standing_tile: Tile::default(),
            };
            *entity.stats.abilities.get_mut("1").unwrap() = "LEVEL 2".to_string();
            *entity.stats.abilities.get_mut("2").unwrap() = "LEVEL 3".to_string();
            *entity.stats.abilities.get_mut("3").unwrap() = "LEVEL 4".to_string();
            *entity.stats.abilities.get_mut("4").unwrap() = "LEVEL 5".to_string();
            *entity.stats.abilities.get_mut("5").unwrap() = "LEVEL 6".to_string();
            update_entity_list(id, entity.clone());
            action_entities.entities.insert(id, entity);
        }
        "move" => {
            println!("x: {}, y: {}", action_chunk_x, action_chunk_y);
            if !action_entities.entities.contains_key(&id) {
                return;
            }
            let e = action_entities.entities.get_mut(&id).unwrap();
            match action.params["move_dir"].as_str() {
                "up" => {
                    if e.chunk_y == 0 && e.relative_y == 0 {
                        return;
                    }
                }
                "down" => {
                    if e.chunk_y == w_p.world_height as i32 - 1
                        && e.relative_y == w_p.chunk_size as i32 - 1
                    {
                        return;
                    }
                }
                "left" => {
                    if e.chunk_x == 0 && e.relative_x == 0 {
                        return;
                    }
                }
                "right" => {
                    if e.chunk_x == w_p.world_width as i32 - 1
                        && e.relative_y == w_p.chunk_size as i32 - 1
                    {
                        return;
                    }
                }
                _ => {}
            };
            e.move_dir(action.params["move_dir"].to_string());
            if e.relative_x < 0 {
                remove_entity = true;
                add_entity = true;
                chunk_x_to_add = action_chunk_x - 1;
                e.relative_x = w_p.chunk_size as i32 - 1;
                e.chunk_x = chunk_x_to_add;
                update_entity_list(id, e.clone());
            }
            if e.relative_y < 0 {
                remove_entity = true;
                add_entity = true;
                chunk_y_to_add = action_chunk_y - 1;
                e.relative_y = w_p.chunk_size as i32 - 1;
                e.chunk_y = chunk_y_to_add;
                update_entity_list(id, e.clone());
            }
            if e.relative_x > w_p.chunk_size as i32 - 1 {
                remove_entity = true;
                add_entity = true;
                chunk_x_to_add = action_chunk_x + 1;
                e.relative_x = 0;
                e.chunk_x = chunk_x_to_add;
                update_entity_list(id, e.clone());
            }
            if e.relative_y > w_p.chunk_size as i32 - 1 {
                remove_entity = true;
                add_entity = true;
                chunk_y_to_add = action_chunk_y + 1;
                e.relative_y = 0;
                e.chunk_y = chunk_y_to_add;
                update_entity_list(id, e.clone());
            }
            if e.resources.get_mut("food").unwrap_or(&mut 0) > &mut 0 {
                *e.resources.get_mut("food").unwrap_or(&mut 0) -= 1;
            } else {
                e.damage("starve".to_string());
            }
            let mut standing_tile = Tile::default();
            for row in action_tiles.tiles.iter() {
                for tile in row.iter() {
                    if tile.x == e.x && tile.y == e.y {
                        standing_tile = tile.clone();
                    }
                }
            }
            if standing_tile.tile_type == "water".to_string() {
                if e.resources.get_mut("wood").unwrap_or(&mut 0) > &mut 0 {
                    *e.resources.get_mut("wood").unwrap_or(&mut 0) -= 1;
                } else {
                    e.damage("water".to_string());
                }
            }
            for (e_id, a_e) in action_entities_clone.entities.iter() {
                if !a_e.alive && a_e.x == e.x && a_e.y == e.y {
                    remove_entity = true;
                    remove_id = *e_id;
                    e.experience += 10;
                } 
            }
        }
        "attack" => {
            if !action_entities.entities.contains_key(&id) {
                return;
            }
            let e = action_entities.entities.get_mut(&id).unwrap();
            let e_clone = e.clone();

            let target_id = action.params["target_id"].parse::<u64>().unwrap();
            if action_entities.entities.contains_key(&target_id) {
                let target_entity = action_entities.entities.get_mut(&target_id).unwrap();
                let dist = ((e_clone.y as f32 - target_entity.y as f32).powf(2.0)
                    + (e_clone.x as f32 - target_entity.x as f32).powf(2.0))
                .sqrt();
                if dist > 2.0 {
                    return;
                }
                match action.params["type"].as_str() {
                    "auto" => {
                        target_entity.damage_by_entity("auto".to_string(), target_entity.clone());
                    }
                    "special" => {
                        match action.params["ability"].as_str() {
                            "" => {
                                //let dmg = rng.gen_range(5..20);
                                //target_entity.hp -= dmg;
                            }
                            _ => {}
                        };
                        //let dmg = rng.gen_range(0..10);
                        //target_entity.hp -= dmg;
                    }
                    _ => {}
                };
            }
        }
        "gather_resource" => {
            let e = action_entities.entities.get_mut(&id).unwrap();
            for row in action_tiles.tiles.iter_mut() {
                for tile in row.iter_mut() {
                    if tile.x == e.x && tile.y == e.y {
                        if tile.tile_type == "grass".to_string() && tile.has_trees {
                            *e.resources.get_mut("wood").unwrap_or(&mut 0) += 1;
                            if e.resources.get_mut("food").unwrap_or(&mut 0) > &mut 0 {
                                *e.resources.get_mut("food").unwrap_or(&mut 0) -= 1;
                            }
                            tile.tile_type = "sand".to_string();
                            tile.has_trees = false;
                        }
                    }
                }
            }
        }
        "gather_food" => {
            let e = action_entities.entities.get_mut(&id).unwrap();
            for row in action_tiles.tiles.iter_mut() {
                for tile in row.iter_mut() {
                    if tile.x == e.x && tile.y == e.y {
                        if !tile.gathered {
                            let f = rng.gen_range(1..4);
                            *e.resources.get_mut("food").unwrap_or(&mut 0) += f;
                        }
                        tile.gathered = true;
                    }
                }
            }
        }
        _ => {}
    };
    let e = action_entities.entities.get_mut(&id).unwrap();
    if e.units.values().len() <= 0 {
        e.alive = false;
    }
    if add_entity
        && chunk_x_to_add >= 0
        && chunk_x_to_add <= w_p.world_width as i32
        && chunk_y_to_add >= 0
        && chunk_y_to_add <= w_p.world_height as i32
    {
        let mut add_entities =
            open_entities_as_struct(chunk_x_to_add as i32, chunk_y_to_add as i32).unwrap();
        let e = action_entities.entities.get(&id).unwrap();
        add_entities.entities.insert(id, e.clone());
        write_entities_to_file(chunk_x_to_add, chunk_y_to_add, add_entities);
    }
    if remove_entity {
        action_entities.entities.remove(&remove_id);
    }
    write_entities_to_file(action_chunk_x, action_chunk_y, action_entities);
    write_tiles_to_file(action_chunk_x, action_chunk_y, action_tiles);
}
pub fn update_entity_list(_id: u64, entity: Entity) {
    let mut u_e = open_client_ids_to_struct();
    match u_e.ids.get_mut(&entity.name) {
        Some(mut e) => {
            e.chunk_x = entity.chunk_x;
            e.chunk_y = entity.chunk_y;
            write_client_ids_to_file(u_e.clone());
        }
        None => {}
    };
}
pub fn write_entities_to_file(x: i32, y: i32, write_entities: Entities) {
    let mut entities_file =
        fs::File::create(format!("world/chunks/chunk_{}_{}/entities.dat", x, y)).unwrap();
    let encoded: Vec<u8> = bincode::serialize(&write_entities).unwrap();

    entities_file.write_all(&encoded);
}
pub fn write_tiles_to_file(x: i32, y: i32, write_tiles: Tiles) {
    let mut tiles_file =
        fs::File::create(format!("world/chunks/chunk_{}_{}/tiles.dat", x, y)).unwrap();
    let encoded: Vec<u8> = bincode::serialize(&write_tiles).unwrap();

    tiles_file.write_all(&encoded);
}
