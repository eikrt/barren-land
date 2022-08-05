use std::collections::hash_map::DefaultHasher;
use crate::entities::{Player};
use crate::world::{World, Tiles, Entities, WorldProperties, Entity, Tile, WorldMapTile, WorldMap};
use crate::queue::{PostData};
use rand::Rng;
use std::{thread, time};
use std::hash::{Hash, Hasher};
use std::io;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::TryRecvError;
use std::time::{SystemTime, UNIX_EPOCH};
use std::fs::File;
use std::fs;
use std::env;
use std::io::prelude::*;
use bincode;
use pancurses::*;
use pancurses::colorpair::ColorPair;
use std::{sync::Mutex, collections::HashMap};
use once_cell::sync::Lazy;

use serde_json;
use crate::server::*;
const REFRESH_TIME: u64 = 10;
const INPUT_DELAY: u64 = 500;
const SCREEN_WIDTH: u8 = 64;
const SCREEN_HEIGHT: u8 = 32;
const EDGE_X: u8 = 16;
const EDGE_Y: u8 = 8;
const HUD_X: u8 = 0;
const HUD_Y: u8 = 32;
const HUD_WIDTH: u8 = 64;
const HUD_HEIGHT: u8 = 12;
const MARGIN_X: i32 = 0;
const MARGIN_Y: i32 = 1;
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
    render_x: i32,
    render_y: i32,
}
struct Camera {
    x: i32,
    y: i32,
}
fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}
pub fn open_tiles(x:i32,y:i32) -> Tiles {
    let path = format!("world/chunks/chunk_{}_{}/tiles.dat",x,y);
    let t = fs::read(path).unwrap();

    let decoded: Tiles = bincode::deserialize(&t).unwrap();
    return decoded; 
}
pub async fn load_tiles(client: reqwest::Client, x: i32, y: i32) -> Tiles {
    let resp = client.get(format!("http://localhost:8080/tiles/{}/{}", x, y))
        .send()
        .await
        .unwrap();
    let body = resp.text().await.unwrap();

    let decoded = serde_json::from_str(&body).unwrap(); 
    return decoded;
}
pub async fn load_world_map_tile(client: reqwest::Client, x: i32, y: i32) -> WorldMapTile {
    let resp = client.get(format!("http://localhost:8080/world_map/{}/{}", x, y))
        .send()
        .await
        .unwrap();
    let body = resp.text().await.unwrap();

    let decoded = serde_json::from_str(&body).unwrap(); 
    return decoded;
}
pub async fn load_chunk_tile(client: reqwest::Client, x: i32, y: i32) -> WorldMapTile {
    let resp = client.get(format!("http://localhost:8080/world_map/{}/{}", x, y))
        .send()
        .await
        .unwrap();
    let body = resp.text().await.unwrap();

    let decoded = serde_json::from_str(&body).unwrap(); 
    return decoded;
}
pub async fn load_entities(client: reqwest::Client, x: i32, y: i32) -> Entities {

    let resp = client.get(format!("http://localhost:8080/entities/{}/{}",x,y))
        .send()
        .await;
    let resp = match resp {
        Ok(r) => r,
        Err(e) => {
            endwin();
            panic!();
        },

    };
    let body = resp.text().await.unwrap();
    let decoded = serde_json::from_str(&body).unwrap(); 
    return decoded;
}
pub async fn load_player(client: reqwest::Client, x: i32, y: i32, id: u64) -> Entity {

    let resp = client.get("http://localhost:8080/entities/{}/{}")
        .send()
        .await
        .unwrap();
    let body = resp.text().await.unwrap();
    let decoded: Entities = serde_json::from_str(&body).unwrap(); 
    return decoded.entities.get(&id).unwrap().clone();
}
pub async fn load_properties(client: reqwest::Client) -> WorldProperties {
    let resp = client.get("http://localhost:8080/world_properties")
        .send()
        .await
        .unwrap();
    let body = resp.text().await.unwrap();
    let decoded = serde_json::from_str(&body).unwrap(); 
    return decoded;
}
pub async fn load_check_if_client_with_id(client: reqwest::Client, username: String, id: u64, chunk_x: i32, chunk_y: i32) -> bool{
    let resp = client.get(format!("http://localhost:8080/client_exists/{}/{}/{}/{}", username, id, chunk_x, chunk_y))
        .send()
        .await
        .unwrap();
    let body = resp.text().await.unwrap();
    let decoded = body.parse().unwrap(); 
    return decoded;
}
pub async fn load_search_entity_clientid(client: reqwest::Client, username: String, id: u64) -> ClientId{
    let resp = client.get(format!("http://localhost:8080/search_entity/{}", username))
        .send()
        .await
        .unwrap();
    let body = resp.text().await.unwrap();
    let decoded = serde_json::from_str(&body).unwrap(); 
    return decoded;
}
pub fn open_world_properties(client: reqwest::Client) -> WorldProperties {
    let path = "world/world_properties.dat";
    let body = fs::read(path).unwrap();
    let decoded: WorldProperties = bincode::deserialize(&body).unwrap();
    let encoded = serde_json::to_string(&decoded).unwrap();
    return decoded; 
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
    let mut move_dir = '?';
    let mut endless_move_mode = false;
    let mut input_change = 0;
    let args: Vec<String> = env::args().collect();
    let render_x = 2;
    let render_y = 1;
    let target: Entity = Entity::default();
    let has_target = false;
    let mut compare_time = SystemTime::now();
    let client = reqwest::Client::new();
    let current_world_properties = open_world_properties(client.clone());
    let mut camera = Camera {
        x: 0,
        y: 0,
    };

    let mut rng = rand::thread_rng();
    
    let mut id: u64 = rng.gen::<u64>(); 
    let mut username = "".to_string();
    if args.len() == 3 {
        username = args[1].clone();
        let to_hashed: String = args[2].parse::<String>().unwrap() + &username;
        id = calculate_hash(&to_hashed);
        
    }
    let mut client_player = ClientPlayer {
        x: 2,
        y: 2,
        relative_x: 2,
        relative_y: 2,
        chunk_x: 0,
        chunk_y: 0,
        render_x: 2,
        render_y: 2,
    };
    if !load_check_if_client_with_id(client.clone(), username.clone(), id, client_player.chunk_x, client_player.chunk_y).await {
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
                    ("id".to_string(), format!("{}", id).to_string()),
                    ("name".to_string(), format!("{}", username).to_string()),

            ])
            }
        ).await;
    }
    else {
    }
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
    init_pair(5,COLOR_BLACK, COLOR_BLACK);
    init_pair(6,COLOR_WHITE, COLOR_WHITE);
    init_pair(7,COLOR_BLACK, COLOR_WHITE);
    init_pair(8,COLOR_WHITE, COLOR_MAGENTA);
    init_pair(9,COLOR_WHITE, COLOR_BLACK);
    let mut ui_tiles = HashMap::new();
    let mut ui_entities = HashMap::new();
    let mut ui_hud = HashMap::new();
    let mut ui_world_map_tiles = HashMap::new();
    ui_world_map_tiles.insert(
        "barren_land".to_string(),
        ui_tile {
            symbol: ".".to_string(),
            color: 1,
        },
    );
    ui_world_map_tiles.insert(
        "rock_desert".to_string(),
        ui_tile {
            symbol: "*".to_string(),
            color: 9,
        },
    );
    ui_world_map_tiles.insert(
        "salt_desert".to_string(),
        ui_tile {
            symbol: "_".to_string(),
            color: 7,
        },
    );
    ui_world_map_tiles.insert(
        "ice_desert".to_string(),
        ui_tile {
            symbol: "~".to_string(),
            color: 7,
        },
    );
    ui_world_map_tiles.insert(
        "ash_desert".to_string(),
        ui_tile {
            symbol: "`".to_string(),
            color: 7,
        },
    );
    ui_world_map_tiles.insert(
        "dunes".to_string(),
        ui_tile {
            symbol: "~".to_string(),
            color: 1,
        },
    );
    ui_hud.insert(
        "border".to_string(),
        ui_tile {
            symbol: " ".to_string(),
            color: 6,
        },
    );
    ui_hud.insert(
        "hud_body".to_string(),
        ui_tile {
            symbol: " ".to_string(),
            color: 5,
        },
    );
    ui_hud.insert(
        "hud_text".to_string(),
        ui_tile {
            symbol: " ".to_string(),
            color: 3,
        },
    );
    ui_tiles.insert(
        "sand".to_string(),
        ui_tile {
            symbol: ".".to_string(),
            color: 1,
        },
    );
    ui_tiles.insert(
        "ice".to_string(),
        ui_tile {
            symbol: ".".to_string(),
            color: 7,
        },
    );
    ui_tiles.insert(
        "dune_sand".to_string(),
        ui_tile {
            symbol: "~".to_string(),
            color: 1,
        },
    );
    ui_tiles.insert(
        "ash".to_string(),
        ui_tile {
            symbol: "`".to_string(),
            color: 7,
        },
    );
    ui_tiles.insert(
        "salt".to_string(),
        ui_tile {
            symbol: "_".to_string(),
            color: 7,
        },
    );
    ui_tiles.insert(
        "gravel".to_string(),
        ui_tile {
            symbol: "*".to_string(),
            color: 9,
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
    let mut current_chunk_tiles: Vec<Vec<Tiles>> = Vec::new();
    let mut current_world_map: Vec<Vec<WorldMapTile>> = Vec::new();
    let mut first_loop = true;
    let mut target_index = 0;
    let mut view = "game".to_string();
    let mut player_inited = false;
    let mut server_clientid = load_search_entity_clientid(client.clone(), username.clone(), id).await;
    client_player.chunk_x = server_clientid.chunk_x;
    client_player.chunk_y = server_clientid.chunk_y;
    camera.x = client_player.chunk_x * current_world_properties.chunk_size as i32 - current_world_properties.chunk_size as i32 / 2;
    camera.y = client_player.chunk_y * current_world_properties.chunk_size as i32 - current_world_properties.chunk_size as i32 / 2;
    while running {
    let mut refresh_tiles = first_loop;
    let mut refresh_entities = true;
    let mut current_chunk_entities = Vec::new();
    let mut targetable_entities: HashMap<u64, Entity> = HashMap::new(); 
    if first_loop {
        for i in 0..current_world_properties.world_width{
            current_world_map.push(Vec::new());
            for j in 0..current_world_properties.world_height{
                   current_world_map[i as usize].push(load_world_map_tile(client.clone(),j as i32,i as i32).await); 
            }
        }
    }
    first_loop = false;
    if refresh_entities {
        for i in 0..render_y*2 {
            current_chunk_entities.push(Vec::new());
            for j in 0..render_x*2 {
                let r_i = i as i32 - render_y as i32;
                let r_j = j as i32 - render_x as i32;
                let mut c_x = client_player.chunk_x + r_j as i32;
                let mut c_y = client_player.chunk_y + r_i as i32;
                if c_x < 0 {
                    c_x = 0;
                }
                if c_y < 0 {
                    c_y = 0;
                }
                if c_x > (current_world_properties.world_width - 1) as i32 {
                    c_x = current_world_properties.world_width as i32 - 1;
                }
                if c_y > (current_world_properties.world_height- 1) as i32 {
                    c_y = current_world_properties.world_width as i32 - 1;
                }
                let p_e = load_entities(client.clone(), c_x as i32,c_y as i32).await;
                current_chunk_entities[i].push(p_e.clone()); 
                targetable_entities.extend(p_e.entities.clone());
            }
        }
    }
    let mut targetable_entities_sorted = Vec::new();
    for e in targetable_entities.values(){
        targetable_entities_sorted.push(e);
    }
    targetable_entities_sorted.sort_by(|e1,e2| {
        e1.x.cmp(&e2.x)
    });
    let mut target = Entity::default();
    if targetable_entities_sorted.len() > 0  {
        target = targetable_entities_sorted[target_index].clone();
    }
    let attributes = ColorPair(3);
    
    window.attron(attributes);
    window.mv (0,1);
    window.printw("Barren Land\n");
        let delta = SystemTime::now().duration_since(compare_time).unwrap();
        let time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
        compare_time = SystemTime::now();
        match view.as_str() {
        "game" => {    
        // render tiles
        for tiles_row in current_chunk_tiles.iter() {
            for chunk_tiles in tiles_row.iter() {
                for row in chunk_tiles.tiles.iter() {
                    for tile in row.iter() {
                        let rel_y = chunk_tiles.y * current_world_properties.chunk_size as i32 + tile.relative_y - camera.y;
                        let rel_x = chunk_tiles.x * current_world_properties.chunk_size as i32 + tile.relative_x - camera.x;
                        if rel_x < 0 || rel_y < 0 ||rel_x > SCREEN_WIDTH as i32 - 1 || rel_y > SCREEN_HEIGHT as i32 - MARGIN_Y{
                            continue;
                        }
                        
                        window.mv(rel_y + MARGIN_Y, rel_x + MARGIN_X);
                        let attributes = ColorPair(ui_tiles[&tile.tile_type].color);
                        window.attron(attributes);
                        window.addstr(ui_tiles[&tile.tile_type].symbol.clone()); 
                    }
                }
            }
        }
        // render entities
        for entities_row in current_chunk_entities.iter() {
            for chunk_entities in entities_row.iter() {

                for (e_id,entity) in chunk_entities.entities.iter() {
                    let rel_y = chunk_entities.y * current_world_properties.chunk_size as i32 + entity.relative_y - camera.y;
                    let rel_x = chunk_entities.x * current_world_properties.chunk_size as i32 + entity.relative_x - camera.x;
                    if rel_x < 0 || rel_y < 0 {
                        continue;
                    }
                    if e_id == &id {
                        client_player.render_x = rel_x;
                        client_player.render_y = rel_y;
                        client_player.relative_x = entity.relative_x;
                        client_player.relative_y = entity.relative_y;
                        client_player.x = entity.x;
                        client_player.y = entity.y;
                    }
                    window.mv(rel_y + 1, rel_x + 1);
                    let attributes = ColorPair(ui_entities[&entity.entity_type].color);
                    window.attron(attributes);
                    window.addstr(ui_entities[&entity.entity_type].symbol.clone()); 

                }
            }
        }
        // draw hud
        for i in HUD_Y..(HUD_Y + HUD_HEIGHT) {
            for j in HUD_X..(HUD_X + HUD_WIDTH) {
                let mut hud_element = "border";
                if !(i == HUD_Y || i == HUD_Y + HUD_HEIGHT - 1 ||j == HUD_X ||j == HUD_X + HUD_WIDTH - 1) {
                    hud_element = "hud_body";
                }
                
                let attributes = ColorPair(ui_hud[hud_element].color);
                window.attron(attributes);
                window.mv(i as i32 + MARGIN_Y,j as i32 + MARGIN_X);
                window.addstr(ui_hud[hud_element].symbol.clone()); 
            }
        }
        let attributes = ColorPair(ui_hud["hud_text"].color);
        // abilities 
        window.attron(attributes);
        window.mv(HUD_Y as i32 + 2 + MARGIN_Y, HUD_X as i32 + 2 + MARGIN_X);
        window.addstr(format!("{}", username.clone()));
        window.mv(HUD_Y as i32 + 2 + MARGIN_Y, HUD_X as i32 + 16 + MARGIN_X);
        window.addstr(format!("ABILITIES: "));
        window.mv(HUD_Y as i32 + 4 + MARGIN_Y, HUD_X as i32 + 16 + MARGIN_X);
        window.addstr(format!("1. ability"));
        window.mv(HUD_Y as i32 + 5 + MARGIN_Y, HUD_X as i32 + 16 + MARGIN_X);
        window.addstr(format!("2. ability"));
        window.mv(HUD_Y as i32 + 6 + MARGIN_Y, HUD_X as i32 + 16 + MARGIN_X);
        window.addstr(format!("3. ability"));
        window.mv(HUD_Y as i32 + 7 + MARGIN_Y, HUD_X as i32 + 16 + MARGIN_X);
        window.addstr(format!("4. ability"));
        window.mv(HUD_Y as i32 + 8 + MARGIN_Y, HUD_X as i32 + 16 + MARGIN_X);
        window.addstr(format!("5. ability"));
        window.mv(HUD_Y as i32 + 4 + MARGIN_Y, HUD_X as i32 + 2 + MARGIN_X);
        window.addstr(format!("HP: 100"));
        window.mv(HUD_Y as i32 + 5 + MARGIN_Y, HUD_X as i32 + 2 + MARGIN_X);
        window.addstr(format!("ENERGY: 100"));
        // draw target
        window.mv(HUD_Y as i32 + 2 + MARGIN_Y, HUD_X as i32 + 32 + MARGIN_X);
        window.addstr(format!("TARGET: {}", target.entity_type));
        /*window.mv(client_player.render_y, client_player.render_x);
        let attributes = ColorPair(ui_entities["ogre"].color);
        window.attron(attributes);
        window.addstr(ui_entities["ogre"].symbol.clone()); */
        },
        "map" => {
            for row in current_world_map.iter() {
                for w_t in row.iter() {
                    let attributes = ColorPair(ui_world_map_tiles[&w_t.chunk_type].color);
                    window.mv(w_t.y + MARGIN_Y, w_t.x + MARGIN_X);
                    window.attron(attributes);
                    window.addstr(ui_world_map_tiles[&w_t.chunk_type.clone()].symbol.clone()); 
                } 
            } 
        },
        _ => {

        },
    }
        match window.getch() {
            Some(Input::Character(c)) => { 
                //    window.addch(c); 
                match c {
                    'w' => {
                        move_dir = 'w'; 
                    },
                    'a' => {
                        move_dir = 'a'; 
                    },
                    's' => {
                        move_dir = 's'; 
                    },
                    'd' => {
                        move_dir = 'd'; 
                    },

                    't' => {
                        endless_move_mode = !endless_move_mode;
                    },
                    'm' => {
                        match view.as_str() {
                            "game" => {
                            view = "map".to_string();
                            },
                            "map" => {
                            view = "game".to_string();
                        
                            },
                            _ => {}

                        }
                    },
                    '\t' => {
                        target_index += 1;
                        if target_index > targetable_entities.values().len() - 1 {
                            target_index = 0;
                        }
                    },
                    _ => {}

                }
                /*window.mv(0,0);
                window.addstr(&format!("{:?}", c)); 
            */
            },
            Some(Input::KeyDC) => running = false,
            Some(input) => {
            },
            None => ()
        }
        let mut do_not_move = false;
        match move_dir {
            'w' => {
                if input_change > INPUT_DELAY {
                    input_change = 0;
                }
                else {
                    do_not_move = true;
                }
                if !do_not_move {
                    move_player(client.clone(), id, "up".to_string(), client_player.clone()).await;
                    client_player.y -= 1;
                    client_player.relative_y -= 1;
                     
                    if client_player.render_y < EDGE_Y as i32 {
                        camera.y -= 1;   
                    }

                }
            },
            'a' => {
                if input_change > INPUT_DELAY {
                    input_change = 0;
                }
                else {
                    do_not_move = true;
                }
                if !do_not_move {
                    move_player(client.clone(), id, "left".to_string(),client_player.clone()).await;
                    client_player.x -= 1;
                    client_player.relative_x -= 1;
                    
                    if client_player.render_x < EDGE_X as i32 {
                        camera.x -= 1;   
                    }
                }
            },
            's' => {
                if input_change > INPUT_DELAY {
                    input_change = 0;
                }
                else {
                    do_not_move = true;
                }
                if !do_not_move {
                    move_player(client.clone(), id, "down".to_string(),client_player.clone()).await;
                    client_player.y += 1;
                    client_player.relative_y += 1;
                    
                    if client_player.render_y > (SCREEN_HEIGHT - EDGE_Y) as i32 {
                        camera.y += 1;   
                    }

                }
            },
            'd' => {
                if input_change > INPUT_DELAY {
                    input_change = 0;
                }
                else {
                    do_not_move = true;
                }
                if !do_not_move {
                    move_player(client.clone(), id, "right".to_string(),client_player.clone()).await;
                    client_player.x += 1;
                    client_player.relative_x += 1;
                    
                    if client_player.render_x > (SCREEN_WIDTH - EDGE_X) as i32 {
                        camera.x += 1;   
                    }

                }
            },
            _ => {}
        }
        match move_dir {
        'd' => {
        if client_player.relative_x == current_world_properties.chunk_size as i32 {
            client_player.chunk_x += 1;
            refresh_tiles = true;
        }
        },
         's' => {
        if client_player.relative_y == current_world_properties.chunk_size as i32 {
            client_player.chunk_y += 1;
            refresh_tiles = true;
        }
        },
        'a' => {
        if client_player.relative_x == -1 {
            client_player.chunk_x -= 1;
            refresh_tiles = true;
        }
        },
        'w' => {
        if client_player.relative_y == -1 { 
            client_player.chunk_y -= 1;
            refresh_tiles = true;
        }
        },
        _ => {}
        }
        if !endless_move_mode {
            move_dir = '?';
        }
        if refresh_tiles {
            current_chunk_tiles = Vec::new();
            for i in 0..render_y*2 {
                    current_chunk_tiles.push(Vec::new());
                    for j in 0..render_x*2 {
                        let r_i = i as i32 - render_y as i32;
                        let r_j = j as i32 - render_x as i32;
                        let mut c_x = client_player.chunk_x + r_j as i32;
                        let mut c_y = client_player.chunk_y + r_i as i32;
                        if c_x < 0 {
                            c_x = 0;
                        }
                        if c_y < 0 {
                            c_y = 0;
                        }
                        if c_x > (current_world_properties.world_width - 1) as i32 {
                            c_x = current_world_properties.world_width as i32 - 1;
                        }
                        if c_y > (current_world_properties.world_height- 1) as i32 {
                            c_y = current_world_properties.world_width as i32 - 1;
                        }
                        current_chunk_tiles[i].push(load_tiles(client.clone(), c_x as i32,c_y as i32).await); 

                    }
                }
        }
        let delta_as_millis = delta.as_millis() as u64;
        input_change += delta_as_millis as u64;
       // window.addstr(format!("{}", input_change));
        // draw hud
        window.refresh();
        window.erase();
        thread::sleep(time::Duration::from_millis(REFRESH_TIME));
        
        }

    endwin();
}
async fn move_player(client: reqwest::Client, id: u64, dir: String, client_player: ClientPlayer) {
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
