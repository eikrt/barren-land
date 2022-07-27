
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
use pancurses::{initscr, endwin, Input, Window};
use std::collections::HashMap;

const REFRESH_TIME: u64 = 1000;
pub fn open_chunk() -> Chunk{
    let path = format!("world/chunks/chunk_{}_{}",1,1);
    let chunk = fs::read(path).unwrap();
    let decoded: Chunk = bincode::deserialize(&chunk).unwrap();
    return decoded; 
}

pub fn run() {
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
    let window = initscr();
    window.refresh();
    window.keypad(true);
    window.timeout(REFRESH_TIME as i32);
    pancurses::noecho();
    let chunk = open_chunk(); 
    let mut tile_symbols = HashMap::new();
    tile_symbols.insert(
        "sand".to_string(),
        ".".to_string(),
    );
    tile_symbols.insert(
        "rock".to_string(),
        "^".to_string(),
    );
    tile_symbols.insert(
        "water".to_string(),
        "~".to_string(),
    );
    while running {

    window.printw("Barren Land\n");
        let delta = SystemTime::now().duration_since(compare_time).unwrap();
        let time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
        compare_time = SystemTime::now();
        for row in chunk.tiles.iter() {
            for tile in row.iter() {
                window.addstr(tile_symbols[&tile.tile_type].clone()); 
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
