use crate::server::*;
use crate::world::*;
use actix_web::*;
use once_cell::sync::Lazy;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::io::prelude::*;
use std::sync::{Arc, Mutex, RwLock};
use std::time::{SystemTime, UNIX_EPOCH};

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
    let mut state = &mut *(q.lock().unwrap_or(default_value));
    state.queue.push(data);
}
pub fn process_entities(q: web::Data<Mutex<ActionQueue>>) {
    let time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
    let mut state = &mut *(q.lock().unwrap());
    let client_ids = open_client_ids_to_struct();
    for (username, ci) in client_ids.ids.iter() {
    let chunk_entities = open_entities_as_struct(ci.chunk_x, ci.chunk_y);
    for (e_id, entity) in chunk_entities.entities.iter() {
        if entity.entity_type == "scarab".to_string() {
            let action = PostData {
                params: HashMap::from([
                    ("command".to_string(), "move".to_string()),
                    ("move_dir".to_string(), "right".to_string()),
                    ("id".to_string(), entity.id.to_string()),
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
            state.queue.push(action);
        }
    }

    }
}

pub fn execute_queue(q: web::Data<Mutex<ActionQueue>>) {
    let mut state = &mut *(q.lock().unwrap());
    if state.queue.len() > 0 {
        println!("in queue: {}", state.queue.len());
        let last = &state.queue[0];
        println!("{:?}", last);
        execute_action(last.clone());
        state.queue.remove(0);
    }
}
pub fn execute_action(action: PostData) {
    let time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
    let mut rng = rand::thread_rng();
    let w_p = open_world_properties_to_struct();
    let action_chunk_x = action.params["chunk_x"].parse::<i32>().unwrap();
    let action_chunk_y = action.params["chunk_y"].parse::<i32>().unwrap();
    let id = action.params["id"].parse::<u64>().unwrap();
    let mut action_entities = open_entities_as_struct(action_chunk_x as i32, action_chunk_y as i32);
    let mut remove_entity = false;
    let mut add_entity = false;
    let mut chunk_x_to_add = action_chunk_x;
    let mut chunk_y_to_add = action_chunk_y;
    match action.params["command"].as_str() {
        "spawn" => {
            let action_x: i32 = action.params["x"].parse::<i32>().unwrap();
            let action_y: i32 = action.params["y"].parse::<i32>().unwrap();
            let mut entity = Entity {
                x: w_p.chunk_size as i32 * action_chunk_x + action_x,
                y: w_p.chunk_size as i32 * action_chunk_y as i32 + action_y,
                relative_x: action_x,
                relative_y: action_y,
                hp: 100,
                energy: 100,
                experience: 0,
                level: 1,
                chunk_x: action_chunk_x as i32,
                chunk_y: action_chunk_y as i32,
                entity_type: "gatherer".to_string(),
                name: action.params["name"].clone(),
                id: id,
                stats: CharacterStats::gatherer(),
            };
            *entity.stats.abilities.get_mut("2").unwrap() = "LEVEL 2".to_string();
            *entity.stats.abilities.get_mut("3").unwrap() = "LEVEL 3".to_string();
            *entity.stats.abilities.get_mut("4").unwrap() = "LEVEL 4".to_string();
            *entity.stats.abilities.get_mut("5").unwrap() = "LEVEL 5".to_string();
            action_entities.entities.insert(id, entity);
        }
        "move" => {
            println!("x: {}, y: {}", action_chunk_x, action_chunk_y);
            if !action_entities.entities.contains_key(&id) {
                return;
            }
            let e = action_entities.entities.get_mut(&id).unwrap();
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
        }
        "attack" => {
            if !action_entities.entities.contains_key(&id) {
                return;
            }
            let e = action_entities.entities.get_mut(&id).unwrap();
            let e_clone = e.clone();

            let target_id = action.params["target_id"].parse::<u64>().unwrap();
            let mut default_entity = Entity::default();
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
                        let dmg = rng.gen_range(1..10);
                        target_entity.hp -= dmg;
                    }
                    "special" => {
                        match action.params["ability"].as_str() {
                            "double kick" => {
                                let dmg = rng.gen_range(5..20);
                                target_entity.hp -= dmg;
                            }
                            _ => {}
                        };
                        let dmg = rng.gen_range(0..10);
                        target_entity.hp -= dmg;
                    }
                    _ => {}
                };
                if action_entities.entities.get_mut(&target_id).unwrap().hp < 0 {
                    action_entities.entities.remove(&target_id);
                    println!("removed entity by id{}", target_id);
                }
            }
        }
        _ => {}
    };
    if add_entity
        && chunk_x_to_add >= 0
        && chunk_x_to_add <= w_p.world_width as i32
        && chunk_y_to_add >= 0
        && chunk_y_to_add <= w_p.world_height as i32
    {
        let mut add_entities =
            open_entities_as_struct(chunk_x_to_add as i32, chunk_y_to_add as i32);
        let e = action_entities.entities.get(&id).unwrap();
        add_entities.entities.insert(id, e.clone());
        write_entities_to_file(chunk_x_to_add, chunk_y_to_add, add_entities);
    }
    if remove_entity {
        action_entities.entities.remove(&id);
    }
    write_entities_to_file(action_chunk_x, action_chunk_y, action_entities);
}
pub fn update_entity_list(id: u64, entity: Entity) {
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
