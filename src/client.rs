use crate::classes::CharacterStats;
use crate::client_utils::*;
use crate::draw::*;
use crate::entities::*;
use crate::queue::PostData;
use crate::server::*;
use crate::tiles::*;
use crate::world::*;
use pancurses::*;
use rand::Rng;
use std::collections::HashMap;
use std::env;
use std::time::SystemTime;
use std::{thread, time};
pub const INPUT_DELAY: u64 = 500;
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
    units: HashMap<u64, Unit>,
    resources: HashMap<String, i32>,
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
    pub graphics_mode: String,
    pub curses: Curses,
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
            graphics_mode: "curses".to_string(),
            curses: Curses::default(),
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
                units: Entity::generate_default_units(),
                resources: HashMap::new(),
            },
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
        let server_clientid: ClientId =
            load_search_entity_clientid(client.clone(), username.clone(), id).await;
        self.client_player.chunk_x = server_clientid.chunk_x;
        self.client_player.chunk_y = server_clientid.chunk_y;
        self.camera.x = self.client_player.chunk_x
            * self.current_world_properties.chunk_size as i32
            - self.current_world_properties.chunk_size as i32 / 2;
        self.camera.y = self.client_player.chunk_y
            * self.current_world_properties.chunk_size as i32
            - self.current_world_properties.chunk_size as i32 / 4;
        self.curses.init();
        while self.running {
            let mut refresh_tiles = self.first_loop;
            let refresh_entities = true;
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

            let delta = SystemTime::now().duration_since(compare_time).unwrap();
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
                                    if self.client_player.x == tile.x
                                        && self.client_player.y == tile.y
                                    {
                                        self.standing_tile = tile.clone();
                                    }
                                    self.curses.draw_tile(tile.clone(), rel_x, rel_y);
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
                                    self.client_player.level = entity.level;
                                    self.client_player.experience = entity.experience;
                                    self.client_player.stats = entity.stats.clone();
                                    self.client_player.units = entity.units.clone();
                                    self.client_player.resources = entity.resources.clone();
                                } else if self.client_player.x == entity.x
                                    && self.client_player.y == entity.y
                                {
                                    self.standing_entity = entity.clone();
                                }
                                if e_id == &(self.target.id) {
                                    continue;
                                }
                                self.curses.draw_entity(entity.clone(), rel_x, rel_y);
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
                    if self.target.entity_type != "no entity".to_string() {
                        self.curses.draw_cursor(rel_y, rel_x);
                    }
                    // draw hud
                    self.curses.draw_hud();
                    self.curses.draw_str_hud(2, 2, username.clone());
                    self.curses
                        .draw_str_hud(2, 16, format!("ABILITIES: ").to_string());
                    self.curses.draw_str_hud(
                        3,
                        16,
                        format!("1. {}", self.client_player.stats.abilities["1"]).to_string(),
                    );
                    self.curses.draw_str_hud(
                        4,
                        16,
                        format!("2. {}", self.client_player.stats.abilities["2"]).to_string(),
                    );
                    self.curses.draw_str_hud(
                        5,
                        16,
                        format!("3. {}", self.client_player.stats.abilities["3"]).to_string(),
                    );
                    self.curses.draw_str_hud(
                        6,
                        16,
                        format!("4. {}", self.client_player.stats.abilities["4"]).to_string(),
                    );
                    self.curses.draw_str_hud(
                        7,
                        16,
                        format!("5. {}", self.client_player.stats.abilities["5"]).to_string(),
                    );
                    self.curses.draw_str_hud(
                        9,
                        2,
                        format!("UNITS: {}", self.client_player.units.values().len()).to_string(),
                    );
                    self.curses.draw_str_hud(
                        4,
                        2,
                        format!("LEVEL: {}", self.client_player.level).to_string(),
                    );
                    self.curses.draw_str_hud(
                        5,
                        2,
                        format!("EXPERIENCE: {}", self.client_player.experience).to_string(),
                    );
                    // draw target

                    self.curses
                        .draw_str_hud(10, 2, format!("TILE: {}", self.standing_tile.tile_type));
                    self.curses.draw_str_hud(
                        10,
                        18,
                        format!("ENTITY: {}", self.standing_entity.entity_type),
                    );
                    self.curses.draw_str_hud(
                        3,
                        32,
                        format!("TARGET TYPE: {}", self.target.entity_type),
                    );
                    self.curses
                        .draw_str_hud(4, 32, format!("TARGET NAME : {}", self.target.name));
                    self.curses.draw_str_hud(
                        5,
                        32,
                        format!("TARGET UNITS: {}", self.target.units.values().len()),
                    );
                    self.curses
                        .draw_str_hud(7, 32, format!("TARGET LEVEL: {}", self.target.level));
                    if self.attacking {
                        self.curses.draw_str_hud(1, 1, format!("/"));
                    }
                }
                "map" => {
                    for row in self.current_world_map.iter() {
                        for w_t in row.iter() {
                            self.curses.draw_world_tile(w_t.clone());
                        }
                    }
                }
                "unit" => {
                    self.curses.draw_str(0,0,format!("UNIT LIST"));
                    let column_margin = 14;
                    let mut text_y = 1;
                    for (_id, unit) in self.client_player.units.iter() {
                        self.curses.draw_str(2,0,(format!("NAME")));
                        self.curses.draw_str(2 + text_y,0,format!("{}", unit.name));
                        self.curses.draw_str(2,column_margin,format!("OCCUPATION"));
                        self.curses.draw_str(2 + text_y, column_margin, format!("{}", unit.profession));
                        self.curses.draw_str(2,column_margin*2,format!("HP"));
                        self.curses.draw_str(2 + text_y, column_margin * 2, format!("{}", unit.hp));
                        self.curses.draw_str(2, column_margin*3, format!("ENERGY"));
                        self.curses.draw_str(2 + text_y, column_margin*3, format!("{}", unit.energy));
                        text_y += 1;
                    }
                }
                "resources" => {
                    self.curses.draw_str(0,0,format!("STATUS"));
                    let column_margin = 14;
                    let mut text_y = 1;
                    self.curses.draw_str(2,0,format!("RESOURCE"));
                    self.curses.draw_str(2,column_margin,format!("AMOUNT"));
                    for (key, resource) in self.client_player.resources.iter() {
                        self.curses.draw_str(2 + text_y,0,format!("{}", key));
                        self.curses.draw_str(2 + text_y, column_margin, format!("{}", resource));
                        text_y += 1;
                    }
                }
                _ => {}
            }
            if targetable_entities.contains_key(&self.target.id) {
                // refresh target
                self.target = targetable_entities[&self.target.id].clone();
            }
            match self.curses.window.getch() {
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
                        'u' => match self.view.as_str() {
                            "unit" => {
                                self.view = "game".to_string();
                            }
                            "game" => {
                                self.view = "unit".to_string();
                            }

                            _ => {}
                        },
                        'r' => match self.view.as_str() {
                            "resources" => {
                                self.view = "game".to_string();
                            }
                            "game" => {
                                self.view = "resources".to_string();
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
                        'g' => {
                            conduct_action(
                                client.clone(),
                                id,
                                self.client_player.clone(),
                                self.target.clone(),
                                "gather_resource".to_string(),
                            )
                            .await;
                            refresh_tiles = true;
                        }
                        'f' => {
                            conduct_action(
                                client.clone(),
                                id,
                                self.client_player.clone(),
                                self.target.clone(),
                                "gather_food".to_string(),
                            )
                            .await;
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
                Some(_input) => {}
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
            self.curses.end_loop();
            thread::sleep(time::Duration::from_millis(REFRESH_TIME));
        }

        self.curses.end_win();
    }
}

async fn conduct_action(
    client: reqwest::Client,
    id: u64,
    client_player: ClientPlayer,
    target: Entity,
    action_type: String,
) {
    post_to_queue(
        client.clone(),
        PostData {
            params: HashMap::from([
                ("command".to_string(), action_type),
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
