
use crate::entities::{Player};
use crate::world::{World, Tiles, Entities, WorldProperties};
use crate::queue::{PostData};
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
const REFRESH_TIME: u64 = 1000;

#[derive (Clone)]
struct ui_tile {
    symbol: String,
    color: u8,
}
struct ui_entity {
    symbol: String,
    color: u8,
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
    let current_tiles = load_tiles(1,1).await; 
    let client = reqwest::Client::new();

    post_to_queue(
        client,
        PostData {
            params: HashMap::from([
                ("command".to_string(), "spawn".to_string()),
                ("id".to_string(), "8".to_string()),
                ("x".to_string(), "8".to_string()),
                ("y".to_string(), "8".to_string()),
                ("chunk_x".to_string(), "1".to_string()),
                ("chunk_y".to_string(), "1".to_string()),
        ])
        }
    ).await;
    let window = initscr();
    window.refresh();
    window.keypad(true);
    window.timeout(REFRESH_TIME as i32);
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

    let current_entities = load_entities(1,1).await; 

    let attributes = ColorPair(3);
    window.attron(attributes);
    window.printw("Barren Land\n");
        let delta = SystemTime::now().duration_since(compare_time).unwrap();
        let time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
        compare_time = SystemTime::now();
        for row in current_tiles.tiles.iter() {
            for tile in row.iter() {
                let attributes = ColorPair(ui_tiles[&tile.tile_type].color);
                window.attron(attributes);
                window.addstr(ui_tiles[&tile.tile_type].symbol.clone()); 
            }
            window.addch('\n');
        }
        for entity in current_entities.entities.values() {
            window.mv(entity.relative_x,entity.relative_y);
            let attributes = ColorPair(ui_entities[&entity.entity_type].color);
            window.attron(attributes);
            window.addstr(ui_entities[&entity.entity_type].symbol.clone()); 

        }

        match window.getch() {
            Some(Input::Character(c)) => { 

                    window.addch(c); 
            },
            Some(Input::KeyDC) => running = false,
            Some(input) => { window.addstr(&format!("{:?}", input)); },
            None => ()
        }
        let delta_as_millis = delta.as_millis();

        window.refresh();
        window.clear();
        thread::sleep(time::Duration::from_millis(REFRESH_TIME));
        }

    endwin();
}
