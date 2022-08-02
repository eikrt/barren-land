use serde::{Deserialize, Serialize};
use actix_web::*;
use crate::server::*;
use std::collections::HashSet;
use std::{collections::HashMap};
use std::sync::{Mutex, Arc, RwLock};
use crate::world::*;
use once_cell::sync::Lazy;
use rand::Rng;
use std::fs;
use std::io::prelude::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ActionQueue {
    pub queue: Vec<PostData>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PostData {
    pub params: HashMap<String, String>, 
}
pub fn queue_to_object(data: web::Json<PostData>) -> PostData {
    PostData{
        params: data.params.clone(),
    }
}
pub fn add_to_queue(q: web::Data<Mutex<ActionQueue>>,data: PostData) {
    let default_queue = ActionQueue {
        queue: Vec::new(),
    };
    let default_mutex = Mutex::new(
        default_queue,
    );
    let default_data = web::Data::new(default_mutex);
    let default_value = default_data.lock().unwrap();
    let mut state = &mut *(q.lock().unwrap_or(
        default_value
    ));
    state.queue.push(data);
}
pub fn execute_queue(q: web::Data<Mutex<ActionQueue>>) {
    let mut state = &mut *(q.lock().unwrap());
    if state.queue.len() > 0 {
        println!("in queue: {}", state.queue.len());
        let latest = &state.queue[state.queue.len()-1];
        println!("{:?}", latest);
        execute_action(latest.clone());
        state.queue.remove(0);
    }
}
pub fn execute_action(action: PostData) {
    let mut rng = rand::thread_rng();
    let w_p = open_world_properties_to_struct();
    let action_chunk_x = action.params["chunk_x"].parse::<i32>().unwrap();
    let action_chunk_y = action.params["chunk_y"].parse::<i32>().unwrap();
    let id = action.params["id"].parse::<u64>().unwrap();
    let mut action_entities = open_entities_as_struct(action_chunk_x as i32,action_chunk_y as i32);
    let mut remove_entity = false;
    let mut add_entity = false;
    let mut chunk_x_to_add = action_chunk_x;
    let mut chunk_y_to_add = action_chunk_y;
    if action.params["command"] == "spawn" {
        let action_x: i32 = action.params["x"].parse::<i32>().unwrap()
;
        let action_y: i32 = action.params["y"].parse::<i32>().unwrap();
        let mut entity = Entity {
            x: w_p.chunk_size as i32 * action_chunk_x + action_x,
            y: w_p.chunk_size as i32 * action_chunk_y as i32 + action_y,
            relative_x: action_x,
            relative_y: action_y,
            chunk_x: action_chunk_x as i32,
            chunk_y: action_chunk_y as i32,
            entity_type: "hero".to_string(),
        };
        action_entities.entities.insert(id, entity);
    }
    else if action.params["command"] == "move" {
        
        println!("x: {}, y: {}",action_chunk_x, action_chunk_y);
        if !action_entities.entities.contains_key(&id) {
            return;
        }
        let e = action_entities.entities.get_mut(&id).unwrap();
        e.move_dir(action.params["move_dir"].to_string());
        if e.relative_x < 0{
            remove_entity = true;
            add_entity = true;
            chunk_x_to_add = action_chunk_x - 1;
            e.relative_x = w_p.chunk_size as i32 - 1;
        }
        if e.relative_y < 0{
            remove_entity = true;
            add_entity = true;
            chunk_y_to_add = action_chunk_y - 1;
            e.relative_y = w_p.chunk_size as i32 - 1;
        }
        if e.relative_x > w_p.chunk_size as i32 - 1{
            remove_entity = true;
            add_entity = true;
            chunk_x_to_add = action_chunk_x + 1;
            e.relative_x = 0;
        }
        if e.relative_y > w_p.chunk_size as i32 - 1{
            remove_entity = true;
            add_entity = true;
            chunk_y_to_add = action_chunk_y + 1;
            e.relative_y = 0;

        }
    }
    if add_entity && chunk_x_to_add >= 0 && chunk_x_to_add <= w_p.world_width as i32 && chunk_y_to_add >= 0 && chunk_y_to_add <= w_p.world_height as i32 {

        println!("x: {}, y: {}",chunk_x_to_add, chunk_y_to_add);
        println!("entity added");
        let mut add_entities = open_entities_as_struct(chunk_x_to_add as i32,chunk_y_to_add as i32);
        let e = action_entities.entities.get(&id).unwrap();
        add_entities.entities.insert(id, e.clone());
        write_entities_to_file(chunk_x_to_add, chunk_y_to_add, add_entities);


    }
    if remove_entity {
        action_entities.entities.remove(&id);
    }
    write_entities_to_file(action_chunk_x, action_chunk_y, action_entities);
    
}
pub fn write_entities_to_file(x: i32, y: i32, write_entities: Entities) {
    let mut entities_file = fs::File::create(format!("world/chunks/chunk_{}_{}/entities.dat",x,y)).unwrap();
    let encoded: Vec<u8> = bincode::serialize(&write_entities).unwrap();

    entities_file.write_all(&encoded);

}
