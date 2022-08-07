use crate::client_utils::*;
use crate::entities::Player;
use crate::queue::PostData;
use crate::world::{
    CharacterStats, Entities, Entity, Tile, Tiles, World, WorldMap, WorldMapTile, WorldProperties,
};
use bincode;
use once_cell::sync::Lazy;
use pancurses::colorpair::ColorPair;
use pancurses::*;
use rand::Rng;
use std::collections::hash_map::DefaultHasher;
use std::env;
use std::fs;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io;
use std::io::prelude::*;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::TryRecvError;
use std::time::{SystemTime, UNIX_EPOCH};
use std::{collections::HashMap, sync::Mutex};
use std::{thread, time};

use crate::server::*;
use serde_json;
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
#[derive(Clone)]
pub struct ui_tile {
    symbol: String,
    color: u8,
}
pub struct ui_entity {
    symbol: String,
    color: u8,
}
#[derive(Clone)]
pub struct ClientPlayer {
    x: i32,
    y: i32,
    relative_x: i32,
    relative_y: i32,
    chunk_x: i32,
    chunk_y: i32,
    render_x: i32,
    render_y: i32,
    level: i32,
    experience: i32,
    hp: i32,
    energy: i32,
    autoattack_change: u64,
    autoattack_time: u64,
    special_attack_change: u64,
    special_attack_time: u64,
    stats: CharacterStats,
}
pub struct Camera {
    x: i32,
    y: i32,
}
pub struct Client {
    pub running: bool,
    pub move_dir: char,
    pub attacking: bool,
    pub endless_move_mode: bool,
    pub input_change: u64,
    pub render_x: i32,
    pub render_y: i32,
    pub target: Entity,
    pub has_target: bool,
    pub camera: Camera,
    pub ui_tiles: HashMap<String, ui_tile>,
    pub ui_entities: HashMap<String, ui_tile>,
    pub ui_hud: HashMap<String, ui_tile>,
    pub ui_world_map_tiles: HashMap<String, ui_tile>,
    pub current_chunk_tiles: Vec<Vec<Tiles>>,
    pub current_world_map: Vec<Vec<WorldMapTile>>,
    pub first_loop: bool,
    pub target_index: usize,
    pub view: String,
    pub player_inited: bool,
    pub client_player: ClientPlayer,
    pub current_world_properties: WorldProperties,
    pub standing_tile: Tile,
    pub standing_entity: Entity,
}
impl Default for Client {
    fn default() -> Client {
        Client {
            current_chunk_tiles: Vec::new(),
            current_world_map: Vec::new(),
            first_loop: true,
            target_index: 0,
            view: "game".to_string(),
            player_inited: false,
            running: true,
            move_dir: '?',
            attacking: false,
            endless_move_mode: false,
            input_change: 0,
            render_x: 2,
            render_y: 1,
            target: Entity::default(),
            has_target: false,
            camera: Camera { x: 0, y: 0 },
            current_world_properties: WorldProperties::default(),
            standing_tile: Tile::default(),
            standing_entity: Entity::default(),

            client_player: ClientPlayer {
                x: 2,
                y: 2,
                relative_x: 2,
                relative_y: 2,
                chunk_x: 0,
                chunk_y: 0,
                render_x: 2,
                render_y: 2,
                hp: 100,
                energy: 100,
                experience: 0,
                level: 1,
                autoattack_time: 1000,
                autoattack_change: 0,
                special_attack_time: 1000,
                special_attack_change: 1000,
                stats: CharacterStats::default(),
            },
            ui_world_map_tiles: HashMap::from([
                (
                    "barren_land".to_string(),
                    ui_tile {
                        symbol: ".".to_string(),
                        color: 1,
                    },
                ),
                (
                    "rock_desert".to_string(),
                    ui_tile {
                        symbol: "*".to_string(),
                        color: 9,
                    },
                ),
                (
                    "salt_desert".to_string(),
                    ui_tile {
                        symbol: "_".to_string(),
                        color: 7,
                    },
                ),
                (
                    "ice_desert".to_string(),
                    ui_tile {
                        symbol: "~".to_string(),
                        color: 7,
                    },
                ),
                (
                    "ash_desert".to_string(),
                    ui_tile {
                        symbol: "`".to_string(),
                        color: 7,
                    },
                ),
                (
                    "dunes".to_string(),
                    ui_tile {
                        symbol: "~".to_string(),
                        color: 1,
                    },
                ),
            ]),
            ui_hud: HashMap::from([
                (
                    "border".to_string(),
                    ui_tile {
                        symbol: " ".to_string(),
                        color: 6,
                    },
                ),
                (
                    "hud_body".to_string(),
                    ui_tile {
                        symbol: " ".to_string(),
                        color: 5,
                    },
                ),
                (
                    "hud_text".to_string(),
                    ui_tile {
                        symbol: " ".to_string(),
                        color: 3,
                    },
                ),
            ]),
            ui_tiles: HashMap::from([
                (
                    "sand".to_string(),
                    ui_tile {
                        symbol: ".".to_string(),
                        color: 1,
                    },
                ),
                (
                    "ice".to_string(),
                    ui_tile {
                        symbol: ".".to_string(),
                        color: 7,
                    },
                ),
                (
                    "dune_sand".to_string(),
                    ui_tile {
                        symbol: "~".to_string(),
                        color: 1,
                    },
                ),
                (
                    "ash".to_string(),
                    ui_tile {
                        symbol: "`".to_string(),
                        color: 9,
                    },
                ),
                (
                    "salt".to_string(),
                    ui_tile {
                        symbol: "_".to_string(),
                        color: 7,
                    },
                ),
                (
                    "gravel".to_string(),
                    ui_tile {
                        symbol: "*".to_string(),
                        color: 9,
                    },
                ),
                (
                    "rock".to_string(),
                    ui_tile {
                        symbol: "^".to_string(),
                        color: 1,
                    },
                ),
                (
                    "water".to_string(),
                    ui_tile {
                        symbol: "~".to_string(),
                        color: 2,
                    },
                ),
                (
                    "grass".to_string(),
                    ui_tile {
                        symbol: ".".to_string(),
                        color: 4,
                    },
                ),
            ]),
            ui_entities: HashMap::from([
                (
                    "ogre".to_string(),
                    ui_tile {
                        symbol: "O".to_string(),
                        color: 3,
                    },
                ),
                (
                    "scarab".to_string(),
                    ui_tile {
                        symbol: "S".to_string(),
                        color: 9,
                    },
                ),
                (
                    "no entity".to_string(),
                    ui_tile {
                        symbol: " ".to_string(),
                        color: 3,
                    },
                ),
                (
                    "gatherer".to_string(),
                    ui_tile {
                        symbol: "g".to_string(),
                        color: 3,
                    },
                ),
            ]),
        }
    }
}
impl Client {
    pub async fn run(&mut self) {
        let args: Vec<String> = env::args().collect();
        let mut compare_time = SystemTime::now();
        let client = reqwest::Client::new();
        self.current_world_properties = load_world_properties(client.clone()).await;
        let mut rng = rand::thread_rng();
        let mut id = rng.gen::<u64>();
        let mut username = "".to_string();
        if args.len() == 3 {
            username = args[1].clone();
            let to_hashed: String = args[2].parse::<String>().unwrap() + &username;
            id = calculate_hash(&to_hashed);
        }
        if !load_check_if_client_with_id(
            // if not client with id in server, spawn
            client.clone(),
            username.clone(),
            id,
            self.client_player.chunk_x,
            self.client_player.chunk_y,
        )
        .await
        {
            post_to_queue(
                client.clone(),
                PostData {
                    params: HashMap::from([
                        ("command".to_string(), "spawn".to_string()),
                        ("id".to_string(), id.to_string()),
                        (
                            "x".to_string(),
                            format!("{}", self.client_player.x).to_string(),
                        ),
                        (
                            "y".to_string(),
                            format!("{}", self.client_player.y).to_string(),
                        ),
                        (
                            "chunk_x".to_string(),
                            format!("{}", self.client_player.chunk_x).to_string(),
                        ),
                        (
                            "chunk_y".to_string(),
                            format!("{}", self.client_player.chunk_y).to_string(),
                        ),
                        ("id".to_string(), format!("{}", id).to_string()),
                        ("name".to_string(), format!("{}", username).to_string()),
                    ]),
                },
            )
            .await;
        } else {
        }
        let window = initscr();
        window.refresh();
        window.keypad(true);
        window.timeout(REFRESH_TIME as i32);
        curs_set(0);
        noecho();
        start_color();
        use_default_colors();
        init_pair(1, COLOR_WHITE, COLOR_YELLOW);
        init_pair(2, COLOR_WHITE, COLOR_BLUE);
        init_pair(3, COLOR_WHITE, COLOR_BLACK);
        init_pair(4, COLOR_WHITE, COLOR_GREEN);
        init_pair(5, COLOR_BLACK, COLOR_BLACK);
        init_pair(6, COLOR_WHITE, COLOR_WHITE);
        init_pair(7, COLOR_BLACK, COLOR_WHITE);
        init_pair(8, COLOR_WHITE, COLOR_MAGENTA);
        init_pair(9, COLOR_WHITE, COLOR_BLACK);
        let mut server_clientid: ClientId =
            load_search_entity_clientid(client.clone(), username.clone(), id).await;
        self.client_player.chunk_x = server_clientid.chunk_x;
        self.client_player.chunk_y = server_clientid.chunk_y;
        self.camera.x = self.client_player.chunk_x
            * self.current_world_properties.chunk_size as i32
            - self.current_world_properties.chunk_size as i32 / 2;
        self.camera.y = self.client_player.chunk_y
            * self.current_world_properties.chunk_size as i32
            - self.current_world_properties.chunk_size as i32 / 4;
        while self.running {
            let mut refresh_tiles = self.first_loop;
            let mut refresh_entities = true;
            let mut current_chunk_entities = Vec::new();
            let mut targetable_entities: HashMap<u64, Entity> = HashMap::new();
            if self.first_loop {
                // load world map tiles if first loop
                for i in 0..self.current_world_properties.world_width {
                    self.current_world_map.push(Vec::new());
                    for j in 0..self.current_world_properties.world_height {
                        self.current_world_map[i as usize]
                            .push(load_world_map_tile(client.clone(), j as i32, i as i32).await);
                    }
                }
            }
            self.first_loop = false;
            if refresh_entities {
                // fetch entities from server
                for i in 0..self.render_y * 2 {
                    current_chunk_entities.push(Vec::new());
                    for j in 0..self.render_x * 2 {
                        let r_i = i as i32 - self.render_y as i32;
                        let r_j = j as i32 - self.render_x as i32;
                        let mut c_x = self.client_player.chunk_x + r_j as i32;
                        let mut c_y = self.client_player.chunk_y + r_i as i32;
                        if c_x < 0 {
                            c_x = 0;
                        }
                        if c_y < 0 {
                            c_y = 0;
                        }
                        if c_x > (self.current_world_properties.world_width - 1) as i32 {
                            c_x = self.current_world_properties.world_width as i32 - 1;
                        }
                        if c_y > (self.current_world_properties.world_height - 1) as i32 {
                            c_y = self.current_world_properties.world_width as i32 - 1;
                        }
                        let p_e = load_entities(client.clone(), c_x as i32, c_y as i32).await;
                        current_chunk_entities[i as usize].push(p_e.clone());
                        targetable_entities.extend(p_e.entities.clone());
                    }
                }
            }
            let mut targetable_entities_sorted = Vec::new(); // sort targetable_entities into vector by x and y
            for e in targetable_entities.values() {
                targetable_entities_sorted.push(e);
            }
            targetable_entities_sorted.sort_by(|e1, e2| e1.x.cmp(&e2.x));
            targetable_entities_sorted.sort_by(|e1, e2| e1.y.cmp(&e2.y));
            let attributes = ColorPair(3);

            window.attron(attributes);
            window.mv(0, 1);
            window.printw("Barren Land\n");
            let delta = SystemTime::now().duration_since(compare_time).unwrap();
            let time = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis();
            compare_time = SystemTime::now();
            self.standing_tile = Tile::default();
            match self.view.as_str() {
                "game" => {
                    // draw tiles
                    for tiles_row in self.current_chunk_tiles.iter() {
                        for chunk_tiles in tiles_row.iter() {
                            for row in chunk_tiles.tiles.iter() {
                                for tile in row.iter() {
                                    let rel_y = chunk_tiles.y
                                        * self.current_world_properties.chunk_size as i32
                                        + tile.relative_y
                                        - self.camera.y;
                                    let rel_x = chunk_tiles.x
                                        * self.current_world_properties.chunk_size as i32
                                        + tile.relative_x
                                        - self.camera.x;
                                    if rel_x < 0
                                        || rel_y < 0
                                        || rel_x > SCREEN_WIDTH as i32 - 1
                                        || rel_y > SCREEN_HEIGHT as i32 - MARGIN_Y
                                    {
                                        continue;
                                    }
                                    if self.client_player.x == tile.x
                                        && self.client_player.y == tile.y
                                    {
                                        self.standing_tile = tile.clone();
                                    }

                                    window.mv(rel_y + MARGIN_Y, rel_x + MARGIN_X);
                                    let attributes =
                                        ColorPair(self.ui_tiles[&tile.tile_type].color);
                                    window.attron(attributes);
                                    window.addstr(self.ui_tiles[&tile.tile_type].symbol.clone());
                                }
                            }
                        }
                    }
                    // draw entities
                    self.standing_entity = Entity::default();
                    for entities_row in current_chunk_entities.iter() {
                        for chunk_entities in entities_row.iter() {
                            for (e_id, entity) in chunk_entities.entities.iter() {
                                let rel_y = chunk_entities.y
                                    * self.current_world_properties.chunk_size as i32
                                    + entity.relative_y
                                    - self.camera.y;
                                let rel_x = chunk_entities.x
                                    * self.current_world_properties.chunk_size as i32
                                    + entity.relative_x
                                    - self.camera.x;
                                if rel_x < 0
                                    || rel_y < 0
                                    || rel_x > SCREEN_WIDTH as i32 - 1
                                    || rel_y > SCREEN_HEIGHT as i32 - MARGIN_Y
                                {
                                    continue;
                                }
                                if e_id == &id {
                                    self.client_player.render_x = rel_x;
                                    self.client_player.render_y = rel_y;
                                    self.client_player.relative_x = entity.relative_x;
                                    self.client_player.relative_y = entity.relative_y;
                                    self.client_player.x = entity.x;
                                    self.client_player.y = entity.y;
                                    self.client_player.hp = entity.hp;
                                    self.client_player.level = entity.level;
                                    self.client_player.experience = entity.experience;
                                    self.client_player.energy = entity.energy;
                                    self.client_player.stats = entity.stats.clone();
                                } else if self.client_player.x == entity.x
                                    && self.client_player.y == entity.y
                                {
                                    self.standing_entity = entity.clone();
                                }
                                if e_id == &(self.target.id) {
                                    continue;
                                }
                                window.mv(rel_y + MARGIN_Y, rel_x + MARGIN_X);

                                let attributes =
                                    ColorPair(self.ui_entities[&entity.entity_type].color);
                                window.attron(attributes);
                                window.addstr(self.ui_entities[&entity.entity_type].symbol.clone());
                            }
                        }
                    }
                    let rel_y = self.target.chunk_y
                        * self.current_world_properties.chunk_size as i32
                        + self.target.relative_y
                        - self.camera.y
                        + MARGIN_Y;
                    let rel_x = self.target.chunk_x
                        * self.current_world_properties.chunk_size as i32
                        + self.target.relative_x
                        - self.camera.x
                        + MARGIN_X;
                    let mut attributes = Attributes::new();
                    if self.target.entity_type != "no entity".to_string() {
                        attributes.set_blink(true);
                        window.attron(attributes);
                        window.mv(rel_y, rel_x);
                        window.addch('X');
                        attributes.set_blink(false);
                        window.attrset(attributes);
                    }
                    // draw hud
                    for i in HUD_Y..(HUD_Y + HUD_HEIGHT) {
                        for j in HUD_X..(HUD_X + HUD_WIDTH) {
                            let mut hud_element = "border";
                            if !(i == HUD_Y
                                || i == HUD_Y + HUD_HEIGHT - 1
                                || j == HUD_X
                                || j == HUD_X + HUD_WIDTH - 1)
                            {
                                hud_element = "hud_body";
                            }

                            let attributes = ColorPair(self.ui_hud[hud_element].color);
                            window.attron(attributes);
                            window.mv(i as i32 + MARGIN_Y, j as i32 + MARGIN_X);
                            window.addstr(self.ui_hud[hud_element].symbol.clone());
                        }
                    }
                    let attributes = ColorPair(self.ui_hud["hud_text"].color);
                    // abilities
                    window.attron(attributes);
                    window.mv(HUD_Y as i32 + 2 + MARGIN_Y, HUD_X as i32 + 2 + MARGIN_X);
                    window.addstr(format!("{}", username.clone()));
                    window.mv(HUD_Y as i32 + 2 + MARGIN_Y, HUD_X as i32 + 16 + MARGIN_X);
                    window.addstr(format!("ABILITIES: "));
                    window.mv(HUD_Y as i32 + 4 + MARGIN_Y, HUD_X as i32 + 16 + MARGIN_X);
                    window.addstr(format!("1. {}", self.client_player.stats.abilities["1"]));
                    window.mv(HUD_Y as i32 + 5 + MARGIN_Y, HUD_X as i32 + 16 + MARGIN_X);
                    window.addstr(format!("2. {}", self.client_player.stats.abilities["2"]));
                    window.mv(HUD_Y as i32 + 6 + MARGIN_Y, HUD_X as i32 + 16 + MARGIN_X);
                    window.addstr(format!("3. {}", self.client_player.stats.abilities["3"]));
                    window.mv(HUD_Y as i32 + 7 + MARGIN_Y, HUD_X as i32 + 16 + MARGIN_X);
                    window.addstr(format!("4. {}", self.client_player.stats.abilities["4"]));
                    window.mv(HUD_Y as i32 + 8 + MARGIN_Y, HUD_X as i32 + 16 + MARGIN_X);
                    window.addstr(format!("5. {}", self.client_player.stats.abilities["5"]));
                    window.mv(HUD_Y as i32 + 4 + MARGIN_Y, HUD_X as i32 + 2 + MARGIN_X);
                    window.addstr(format!("HP: {}", self.client_player.hp));
                    window.mv(HUD_Y as i32 + 5 + MARGIN_Y, HUD_X as i32 + 2 + MARGIN_X);
                    window.addstr(format!("ENERGY: {}", self.client_player.energy));
                    window.mv(HUD_Y as i32 + 6 + MARGIN_Y, HUD_X as i32 + 2 + MARGIN_X);
                    window.addstr(format!("LEVEL: {}", self.client_player.level));
                    window.mv(HUD_Y as i32 + 7 + MARGIN_Y, HUD_X as i32 + 2 + MARGIN_X);
                    window.addstr(format!("EXPERIENCE: {}", self.client_player.experience));
                    // draw target
                    window.mv(HUD_Y as i32 + 2 + MARGIN_Y, HUD_X as i32 + 32 + MARGIN_X);
                    window.addstr(format!(
                        "TARGET: {}",
                        self.ui_entities[&self.target.entity_type].symbol
                    ));
                    window.mv(HUD_Y as i32 + 10 + MARGIN_Y, HUD_X as i32 + MARGIN_X + 1);
                    window.addstr(format!("TILE: {}", self.standing_tile.tile_type));
                    window.mv(HUD_Y as i32 + 10 + MARGIN_Y, HUD_X as i32 + MARGIN_X + 18);
                    window.addstr(format!("ENTITY: {}", self.standing_entity.entity_type));
                    window.mv(HUD_Y as i32 + 3 + MARGIN_Y, HUD_X as i32 + 32 + MARGIN_X);
                    window.addstr(format!("TARGET TYPE: {}", self.target.entity_type));
                    window.mv(HUD_Y as i32 + 4 + MARGIN_Y, HUD_X as i32 + 32 + MARGIN_X);
                    window.addstr(format!("TARGET NAME : {}", self.target.name));
                    window.mv(HUD_Y as i32 + 5 + MARGIN_Y, HUD_X as i32 + 32 + MARGIN_X);
                    window.addstr(format!("TARGET HEALTH: {}", self.target.hp));
                    window.mv(HUD_Y as i32 + 6 + MARGIN_Y, HUD_X as i32 + 32 + MARGIN_X);
                    window.addstr(format!("TARGET ENERGY: {}", self.target.energy));
                    window.mv(HUD_Y as i32 + 7 + MARGIN_Y, HUD_X as i32 + 32 + MARGIN_X);
                    window.addstr(format!("TARGET LEVEL: {}", self.target.level));
                    if self.attacking {
                        window.mv(HUD_Y as i32 + 1 + MARGIN_Y, HUD_X as i32 + 1 + MARGIN_X);
                        window.addstr(format!("/"));
                    }
                    /*window.mv(self.client_player.self.render_y, self.client_player.self.render_x);
                    let attributes = ColorPair(self.ui_entities["ogre"].color);
                    window.attron(attributes);
                    window.addstr(self.ui_entities["ogre"].symbol.clone()); */
                }
                "map" => {
                    for row in self.current_world_map.iter() {
                        for w_t in row.iter() {
                            let attributes =
                                ColorPair(self.ui_world_map_tiles[&w_t.chunk_type].color);
                            window.mv(w_t.y + MARGIN_Y, w_t.x + MARGIN_X);
                            window.attron(attributes);
                            window.addstr(
                                self.ui_world_map_tiles[&w_t.chunk_type.clone()]
                                    .symbol
                                    .clone(),
                            );
                        }
                    }
                }
                _ => {}
            }
            if targetable_entities.contains_key(&self.target.id) {
                // refresh target
                self.target = targetable_entities[&self.target.id].clone();
            }
            match window.getch() {
                Some(Input::Character(c)) => {
                    //    window.addch(c);
                    match c {
                        'w' => {
                            self.move_dir = 'w';
                        }
                        'a' => {
                            self.move_dir = 'a';
                        }
                        's' => {
                            self.move_dir = 's';
                        }
                        'd' => {
                            self.move_dir = 'd';
                        }

                        't' => {
                            self.endless_move_mode = !self.endless_move_mode;
                        }
                        'm' => match self.view.as_str() {
                            "game" => {
                                self.view = "map".to_string();
                            }
                            "map" => {
                                self.view = "game".to_string();
                            }
                            _ => {}
                        },
                        '\t' => {
                            self.target_index += 1;
                            if self.target_index > targetable_entities.values().len() - 1 {
                                self.target_index = 0;
                            }

                            if targetable_entities_sorted.len() > 0 {
                                if self.target_index > targetable_entities_sorted.len() - 1 {
                                    self.target_index = targetable_entities_sorted.len() - 1;
                                }
                                if targetable_entities_sorted[self.target_index].clone().id != id {
                                    self.target =
                                        targetable_entities_sorted[self.target_index].clone();
                                }
                            }

                            if targetable_entities_sorted.len() > 0 {
                                if targetable_entities_sorted[self.target_index].clone().id != id {
                                    self.target =
                                        targetable_entities_sorted[self.target_index].clone();
                                }
                            }
                        }
                        'c' => {
                            self.attacking = !self.attacking;
                        }
                        '1' => {
                            if self.client_player.special_attack_change
                                > self.client_player.special_attack_time
                            {
                                attack(
                                    client.clone(),
                                    id,
                                    self.client_player.clone(),
                                    self.target.clone(),
                                    "special".to_string(),
                                    format!("{}", self.client_player.stats.abilities["1"])
                                        .to_string(),
                                )
                                .await;
                                self.client_player.special_attack_change = 0;
                            }
                        }
                        '2' => {
                            if self.client_player.special_attack_change
                                > self.client_player.special_attack_time
                            {
                                attack(
                                    client.clone(),
                                    id,
                                    self.client_player.clone(),
                                    self.target.clone(),
                                    "special".to_string(),
                                    format!("{}", self.client_player.stats.abilities["2"])
                                        .to_string(),
                                )
                                .await;
                                self.client_player.special_attack_change = 0;
                            }
                        }
                        '3' => {
                            if self.client_player.special_attack_change
                                > self.client_player.special_attack_time
                            {
                                attack(
                                    client.clone(),
                                    id,
                                    self.client_player.clone(),
                                    self.target.clone(),
                                    "special".to_string(),
                                    format!("{}", self.client_player.stats.abilities["3"])
                                        .to_string(),
                                )
                                .await;
                                self.client_player.special_attack_change = 0;
                            }
                        }
                        '4' => {
                            if self.client_player.special_attack_change
                                > self.client_player.special_attack_time
                            {
                                attack(
                                    client.clone(),
                                    id,
                                    self.client_player.clone(),
                                    self.target.clone(),
                                    "special".to_string(),
                                    format!("{}", self.client_player.stats.abilities["4"])
                                        .to_string(),
                                )
                                .await;
                                self.client_player.special_attack_change = 0;
                            }
                        }
                        '5' => {
                            if self.client_player.special_attack_change
                                > self.client_player.special_attack_time
                            {
                                attack(
                                    client.clone(),
                                    id,
                                    self.client_player.clone(),
                                    self.target.clone(),
                                    "special".to_string(),
                                    format!("{}", self.client_player.stats.abilities["5"])
                                        .to_string(),
                                )
                                .await;
                                self.client_player.special_attack_change = 0;
                            }
                        }
                        _ => {}
                    }
                    /*window.mv(0,0);
                        window.addstr(&format!("{:?}", c));
                    */
                }
                Some(Input::KeyDC) => self.running = false,
                Some(input) => {}
                None => (),
            }
            let mut do_not_move = false;
            match self.move_dir {
                'w' => {
                    if self.input_change > INPUT_DELAY {
                        self.input_change = 0;
                    } else {
                        do_not_move = true;
                    }
                    if !do_not_move {
                        move_player(
                            client.clone(),
                            id,
                            "up".to_string(),
                            self.client_player.clone(),
                        )
                        .await;
                        self.client_player.y -= 1;
                        self.client_player.relative_y -= 1;

                        if self.client_player.render_y < EDGE_Y as i32 {
                            self.camera.y -= 1;
                        }
                    }
                }
                'a' => {
                    if self.input_change > INPUT_DELAY {
                        self.input_change = 0;
                    } else {
                        do_not_move = true;
                    }
                    if !do_not_move {
                        move_player(
                            client.clone(),
                            id,
                            "left".to_string(),
                            self.client_player.clone(),
                        )
                        .await;
                        self.client_player.x -= 1;
                        self.client_player.relative_x -= 1;

                        if self.client_player.render_x < EDGE_X as i32 {
                            self.camera.x -= 1;
                        }
                    }
                }
                's' => {
                    if self.input_change > INPUT_DELAY {
                        self.input_change = 0;
                    } else {
                        do_not_move = true;
                    }
                    if !do_not_move {
                        move_player(
                            client.clone(),
                            id,
                            "down".to_string(),
                            self.client_player.clone(),
                        )
                        .await;
                        self.client_player.y += 1;
                        self.client_player.relative_y += 1;

                        if self.client_player.render_y > (SCREEN_HEIGHT - EDGE_Y) as i32 {
                            self.camera.y += 1;
                        }
                    }
                }
                'd' => {
                    if self.input_change > INPUT_DELAY {
                        self.input_change = 0;
                    } else {
                        do_not_move = true;
                    }
                    if !do_not_move {
                        move_player(
                            client.clone(),
                            id,
                            "right".to_string(),
                            self.client_player.clone(),
                        )
                        .await;
                        self.client_player.x += 1;
                        self.client_player.relative_x += 1;

                        if self.client_player.render_x > (SCREEN_WIDTH - EDGE_X) as i32 {
                            self.camera.x += 1;
                        }
                    }
                }
                _ => {}
            }
            match self.move_dir {
                'd' => {
                    if self.client_player.relative_x
                        == self.current_world_properties.chunk_size as i32
                    {
                        self.client_player.chunk_x += 1;
                        refresh_tiles = true;
                    }
                }
                's' => {
                    if self.client_player.relative_y
                        == self.current_world_properties.chunk_size as i32
                    {
                        self.client_player.chunk_y += 1;
                        refresh_tiles = true;
                    }
                }
                'a' => {
                    if self.client_player.relative_x == -1 {
                        self.client_player.chunk_x -= 1;
                        refresh_tiles = true;
                    }
                }
                'w' => {
                    if self.client_player.relative_y == -1 {
                        self.client_player.chunk_y -= 1;
                        refresh_tiles = true;
                    }
                }
                _ => {}
            }

            if !self.endless_move_mode {
                self.move_dir = '?';
            }
            if refresh_tiles {
                self.current_chunk_tiles = Vec::new();
                for i in 0..self.render_y * 2 {
                    self.current_chunk_tiles.push(Vec::new());
                    for j in 0..self.render_x * 2 {
                        let r_i = i as i32 - self.render_y as i32;
                        let r_j = j as i32 - self.render_x as i32;
                        let mut c_x = self.client_player.chunk_x + r_j as i32;
                        let mut c_y = self.client_player.chunk_y + r_i as i32;
                        if c_x < 0 {
                            c_x = 0;
                        }
                        if c_y < 0 {
                            c_y = 0;
                        }
                        if c_x > (self.current_world_properties.world_width - 1) as i32 {
                            c_x = self.current_world_properties.world_width as i32 - 1;
                        }
                        if c_y > (self.current_world_properties.world_height - 1) as i32 {
                            c_y = self.current_world_properties.world_width as i32 - 1;
                        }
                        self.current_chunk_tiles[i as usize]
                            .push(load_tiles(client.clone(), c_x as i32, c_y as i32).await);
                    }
                }
            }
            let delta_as_millis = delta.as_millis() as u64;
            self.input_change += delta_as_millis as u64;
            if self.attacking {
                self.client_player.autoattack_change += delta_as_millis;
                self.client_player.special_attack_change += delta_as_millis;
                if self.client_player.autoattack_change > self.client_player.autoattack_time {
                    attack(
                        client.clone(),
                        id,
                        self.client_player.clone(),
                        self.target.clone(),
                        "auto".to_string(),
                        "".to_string(),
                    )
                    .await;
                    self.client_player.autoattack_change = 0;
                }
            }

            // window.addstr(format!("{}", self.input_change));
            // draw hud
            window.refresh();
            window.erase();
            thread::sleep(time::Duration::from_millis(REFRESH_TIME));
        }

        endwin();
    }
}
async fn attack(
    client: reqwest::Client,
    id: u64,
    client_player: ClientPlayer,
    target: Entity,
    attack_type: String,
    ability: String,
) {
    post_to_queue(
        client.clone(),
        PostData {
            params: HashMap::from([
                ("command".to_string(), "attack".to_string()),
                ("type".to_string(), attack_type),
                ("ability".to_string(), ability),
                ("id".to_string(), id.to_string()),
                ("target_id".to_string(), format!("{}", target.id)),
                (
                    "chunk_x".to_string(),
                    format!("{}", client_player.chunk_x).to_string(),
                ),
                (
                    "chunk_y".to_string(),
                    format!("{}", client_player.chunk_y).to_string(),
                ),
            ]),
        },
    )
    .await;
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
                        (
                            "chunk_x".to_string(),
                            format!("{}", client_player.chunk_x).to_string(),
                        ),
                        (
                            "chunk_y".to_string(),
                            format!("{}", client_player.chunk_y).to_string(),
                        ),
                    ]),
                },
            )
            .await;
        }
        "down" => {
            post_to_queue(
                client.clone(),
                PostData {
                    params: HashMap::from([
                        ("command".to_string(), "move".to_string()),
                        ("move_dir".to_string(), "down".to_string()),
                        ("id".to_string(), id.to_string()),
                        (
                            "chunk_x".to_string(),
                            format!("{}", client_player.chunk_x).to_string(),
                        ),
                        (
                            "chunk_y".to_string(),
                            format!("{}", client_player.chunk_y).to_string(),
                        ),
                    ]),
                },
            )
            .await;
        }
        "left" => {
            post_to_queue(
                client.clone(),
                PostData {
                    params: HashMap::from([
                        ("command".to_string(), "move".to_string()),
                        ("move_dir".to_string(), "left".to_string()),
                        ("id".to_string(), id.to_string()),
                        (
                            "chunk_x".to_string(),
                            format!("{}", client_player.chunk_x).to_string(),
                        ),
                        (
                            "chunk_y".to_string(),
                            format!("{}", client_player.chunk_y).to_string(),
                        ),
                    ]),
                },
            )
            .await;
        }
        "right" => {
            post_to_queue(
                client.clone(),
                PostData {
                    params: HashMap::from([
                        ("command".to_string(), "move".to_string()),
                        ("move_dir".to_string(), "right".to_string()),
                        ("id".to_string(), id.to_string()),
                        (
                            "chunk_x".to_string(),
                            format!("{}", client_player.chunk_x).to_string(),
                        ),
                        (
                            "chunk_y".to_string(),
                            format!("{}", client_player.chunk_y).to_string(),
                        ),
                    ]),
                },
            )
            .await;
        }

        _ => {}
    }
}
