
use crate::entities::{Player};
use crate::world::{World, Chunk, get_generated_world};
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
use std::collections::HashMap;
use serde_json;
use crate::server::*;
const REFRESH_TIME: u64 = 1000;
#[derive (Clone)]
struct ui_tile {
    symbol: String,
    color: u8,
}
pub fn open_chunk(x:i32,y:i32) -> Chunk{
    let path = format!("world/chunks/chunk_{}_{}",x,y);
    let chunk = fs::read(path).unwrap();

    let decoded: Chunk = bincode::deserialize(&chunk).unwrap();
    return decoded; 
}
pub async fn load_chunk(x: i32, y: i32) -> Chunk {
    let resp = reqwest::get(format!("http://localhost:8080/chunks/{}/{}", x, y))
        .await.unwrap();
    let body = resp.text().await.unwrap();
    //println!("{:?}", json);
    let decoded = serde_json::from_str(&body).unwrap(); 
    //let decoded = bincode::deserialize(&body).unwrap();
    return decoded; 
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
    //let chunk1 = open_chunk(1,1); 
    let chunk = load_chunk(1,1).await; 
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
    while running {

    let attributes = ColorPair(3);
    window.attron(attributes);
    window.printw("Barren Land\n");
        let delta = SystemTime::now().duration_since(compare_time).unwrap();
        let time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
        compare_time = SystemTime::now();
        for row in chunk.tiles.iter() {
            for tile in row.iter() {
                let attributes = ColorPair(ui_tiles[&tile.tile_type].color);
                window.attron(attributes);
                window.addstr(ui_tiles[&tile.tile_type].symbol.clone()); 
            }
            window.addch('\n');
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
