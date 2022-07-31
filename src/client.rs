
use crate::entities::{Player};
use crate::world::{World, Tiles, Entities, WorldProperties, Entity, Tile};
use crate::queue::{PostData};
use rand::Rng;
use std::{thread, time};
use std::io;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::TryRecvError;
use std::time::{SystemTime, UNIX_EPOCH};
use std::fs::File;
use std::fs;
use std::io::prelude::*;
use bincode;
use pancurses::*;
use pancurses::colorpair::ColorPair;
use std::{sync::Mutex, collections::HashMap};
use once_cell::sync::Lazy;

use serde_json;
use crate::server::*;
const REFRESH_TIME: u64 = 10;

#[derive (Clone)]
struct ui_tile {
    symbol: String,
    color: u8,
}
struct ui_entity {
    symbol: String,
    color: u8,
}
#[derive (Clone)]
struct ClientPlayer {
    x: i32,
    y: i32,
    relative_x: i32,
    relative_y: i32,
    chunk_x: i32,
    chunk_y: i32,
}
pub fn open_tiles(x:i32,y:i32) -> Tiles {
    let path = format!("world/chunks/chunk_{}_{}/tiles.dat",x,y);
    let t = fs::read(path).unwrap();

    let decoded: Tiles = bincode::deserialize(&t).unwrap();
    return decoded; 
}
pub async fn load_tiles(x: i32, y: i32) -> Tiles {
    let resp = reqwest::get(format!("http://localhost:8080/tiles/{}/{}", x, y))
        .await.unwrap();
    let body = resp.text().await.unwrap();
    let decoded = serde_json::from_str(&body).unwrap(); 
    return decoded;
}
pub async fn load_entities(x: i32, y: i32) -> Entities {

    let resp = reqwest::get(format!("http://localhost:8080/entities/{}/{}", x, y))
        .await.unwrap();
    let body = resp.text().await.unwrap();
    let decoded = serde_json::from_str(&body).unwrap(); 
    return decoded;
}
pub async fn load_player(x: i32, y: i32, id: u32) -> Entity {

    let resp = reqwest::get(format!("http://localhost:8080/entities/{}/{}", x, y))
        .await.unwrap();
    let body = resp.text().await.unwrap();
    let decoded: Entities = serde_json::from_str(&body).unwrap(); 
    return decoded.entities.get(&id).unwrap().clone();
}
pub async fn load_properties() -> WorldProperties {
    let resp = reqwest::get(format!("http://localhost:8080/world_properties"))
        .await.unwrap();
    let body = resp.text().await.unwrap();
    let decoded = serde_json::from_str(&body).unwrap(); 
    return decoded;
}
pub fn open_world_properties() -> String {
    let path = "world/world_properties.dat";
    let body = fs::read(path).unwrap();
    let decoded: WorldProperties = bincode::deserialize(&body).unwrap();
    let encoded = serde_json::to_string(&decoded).unwrap();
    return encoded; 
}
pub async fn post_to_queue(client: reqwest::Client, action: PostData) {
    let res = client.post("http://localhost:8080/queue")
        .json(&action)
        .send()
        .await;
}
pub async fn run() {
    let mut running = true;
    let mut w = false;
    let mut a = false;
    let mut s = false;
    let mut d = false;
    let mut up = false;
    let mut down = false;
    let mut left = false;
    let mut right = false;
    let mut compare_time = SystemTime::now();
    let current_world_properties = open_world_properties_to_struct();
    let mut current_chunk_tiles = Vec::new();
    for i in 0..2 {
        current_chunk_tiles.push(Vec::new());
        for j in 0..3 {
            current_chunk_tiles[i].push(load_tiles(j as i32,i as i32).await); 
        }
    }
    let client = reqwest::Client::new();
    let mut rng = rand::thread_rng();
    let id: u32 = rng.gen::<u32>(); 
    let mut client_player = ClientPlayer {
        x: 8,
        y: 8,
        relative_x: 8,
        relative_y: 8,
        chunk_x: 1,
        chunk_y: 1,
    };
    post_to_queue(
        client.clone(),
        PostData {
            params: HashMap::from([
                ("command".to_string(), "spawn".to_string()),
                ("id".to_string(), id.to_string()),
                ("x".to_string(), format!("{}", client_player.x).to_string()),
                ("y".to_string(), format!("{}", client_player.y).to_string()),
                ("chunk_x".to_string(), format!("{}", client_player.chunk_x).to_string()),
                ("chunk_y".to_string(), format!("{}", client_player.chunk_y).to_string()),
        ])
        }
    ).await;
    let window = initscr();
    window.refresh();
    window.keypad(true);
    window.timeout(REFRESH_TIME as i32);
    curs_set(0);
    noecho();
    start_color();
    use_default_colors();
    init_pair(1,COLOR_WHITE, COLOR_YELLOW);
    init_pair(2,COLOR_WHITE, COLOR_BLUE);
    init_pair(3,COLOR_WHITE, COLOR_BLACK);
    init_pair(4,COLOR_WHITE, COLOR_GREEN);
    let mut ui_tiles = HashMap::new();
    let mut ui_entities = HashMap::new();
    ui_tiles.insert(
        "sand".to_string(),
        ui_tile {
            symbol: ".".to_string(),
            color: 1,
        },
    );
    ui_tiles.insert(
        "rock".to_string(),
        ui_tile {
            symbol: "^".to_string(),
            color: 1,
        },
    );
    ui_tiles.insert(
        "water".to_string(),
        ui_tile {
            symbol: "~".to_string(),
            color: 2,
        }
    );
    ui_tiles.insert(
        "grass".to_string(),
        ui_tile {
            symbol: ".".to_string(),
            color: 4,
        },
    );
    ui_entities.insert(
        "ogre".to_string(),
        ui_tile {
            symbol: "O".to_string(),
            color: 3,
        },
    );
    ui_entities.insert(
        "hero".to_string(),
        ui_tile {
            symbol: "@".to_string(),
            color: 3,
        },
    );
    while running {

    let mut current_chunk_entities = Vec::new();
    for i in 0..2 {
        current_chunk_entities.push(Vec::new());
        for j in 0..3 {
            let c_x = client_player.chunk_x + j as i32;
            let c_y = client_player.chunk_y + i as i32;

            current_chunk_entities[i].push(load_entities(c_x as i32,c_y as i32).await); 
        }
    }

    let attributes = ColorPair(3);
    window.attron(attributes);
    window.printw("Barren Land\n");
        let delta = SystemTime::now().duration_since(compare_time).unwrap();
        let time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
        compare_time = SystemTime::now();
        for tiles_row in current_chunk_tiles.iter() {
            for chunk_tiles in tiles_row.iter() {
                for row in chunk_tiles.tiles.iter() {
                    for tile in row.iter() {
                        window.mv(chunk_tiles.y * current_world_properties.chunk_size as i32 + tile.relative_y, chunk_tiles.x * current_world_properties.chunk_size as i32 + tile.relative_x);
                        let attributes = ColorPair(ui_tiles[&tile.tile_type].color);
                        window.attron(attributes);
                        window.addstr(ui_tiles[&tile.tile_type].symbol.clone()); 
                    }
                }
            }
        }
        for entities_row in current_chunk_entities.iter() {
            for chunk_entities in entities_row.iter() {

                for entity in chunk_entities.entities.values() {
                    window.mv(chunk_entities.y * current_world_properties.chunk_size as i32 + entity.relative_y, chunk_entities.x * current_world_properties.chunk_size as i32 + entity.relative_x);
                    let attributes = ColorPair(ui_entities[&entity.entity_type].color);
                    window.attron(attributes);
                    window.addstr(ui_entities[&entity.entity_type].symbol.clone()); 

                }
            }
        }

        match window.getch() {
            Some(Input::Character(c)) => { 

                //    window.addch(c); 
                match c {
                    'w' => {
                        move_player(client.clone(), id, "up".to_string(), client_player.clone()).await;
                        client_player.y -= 1;
                        client_player.relative_y -= 1;
                        
                         
                    },
                    'a' => {
                        move_player(client.clone(), id, "left".to_string(),client_player.clone()).await;
                        client_player.x -= 1;
                        client_player.relative_x -= 1;
                        
                         
                    },
                    's' => {
                        move_player(client.clone(), id, "down".to_string(),client_player.clone()).await;
                        client_player.y += 1;
                        client_player.relative_y += 1;
                        
                         
                    },
                    'd' => {
                        move_player(client.clone(), id, "right".to_string(),client_player.clone()).await;
                        client_player.x += 1;
                        client_player.relative_x += 1;
                        
                         
                    },
                    _ => {}

                }
                    if client_player.relative_x < 0{
                        client_player.chunk_x -= 1;
                        client_player.relative_x = current_world_properties.chunk_size as i32 - 1;
                    }
                    else if client_player.relative_y < 0{
                        client_player.chunk_y -= 1;
                        client_player.relative_y = current_world_properties.chunk_size as i32 - 1;
                    }
                    else if client_player.relative_x > current_world_properties.chunk_size as i32 - 1{
                        client_player.chunk_x += 1;
                        client_player.relative_x = 0;
                    }
                    else if client_player.relative_y > current_world_properties.chunk_size as i32 - 1{
                        client_player.chunk_y += 1;
                        client_player.relative_y = 0;

                    }
            },
            Some(Input::KeyDC) => running = false,
            Some(input) => {
                //window.addstr(&format!("{:?}", input)); 
            },
            None => ()
        }
        let delta_as_millis = delta.as_millis();

        window.refresh();
        window.erase();
        thread::sleep(time::Duration::from_millis(REFRESH_TIME));
        }

    endwin();
}
async fn move_player(client: reqwest::Client, id: u32, dir: String, client_player: ClientPlayer) {
    match dir.as_str() { 
        "up" => {
        post_to_queue(
            client.clone(),
            PostData {
                params: HashMap::from([
                    ("command".to_string(), "move".to_string()),
                    ("move_dir".to_string(), "up".to_string()),
                    ("id".to_string(), id.to_string()),
                    ("chunk_x".to_string(), format!("{}", client_player.chunk_x).to_string()),
                    ("chunk_y".to_string(), format!("{}", client_player.chunk_y).to_string()),
            ])
            }
        ).await;

        },
    "down" => {
        post_to_queue(
            client.clone(),
            PostData {
                params: HashMap::from([
                    ("command".to_string(), "move".to_string()),
                    ("move_dir".to_string(), "down".to_string()),
                    ("id".to_string(), id.to_string()),
                    ("chunk_x".to_string(), format!("{}", client_player.chunk_x).to_string()),
                    ("chunk_y".to_string(), format!("{}", client_player.chunk_y).to_string()),
            ])
            }
        ).await;

        },
    "left" => {
        post_to_queue(
            client.clone(),
            PostData {
                params: HashMap::from([
                    ("command".to_string(), "move".to_string()),
                    ("move_dir".to_string(), "left".to_string()),
                    ("id".to_string(), id.to_string()),
                    ("chunk_x".to_string(), format!("{}", client_player.chunk_x).to_string()),
                    ("chunk_y".to_string(), format!("{}", client_player.chunk_y).to_string()),
            ])
            }
        ).await;

        },
    "right" => {
        post_to_queue(
            client.clone(),
            PostData {
                params: HashMap::from([
                    ("command".to_string(), "move".to_string()),
                    ("move_dir".to_string(), "right".to_string()),
                    ("id".to_string(), id.to_string()),
                    ("chunk_x".to_string(), format!("{}", client_player.chunk_x).to_string()),
                    ("chunk_y".to_string(), format!("{}", client_player.chunk_y).to_string()),
            ])
            }
        ).await;

        }

    _ => {}
    }
}
