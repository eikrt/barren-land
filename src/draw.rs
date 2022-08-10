use crate::entities::*;
use crate::tiles::*;
use pancurses::colorpair::ColorPair;
use pancurses::*;
use std::collections::HashMap;
pub const SCREEN_WIDTH: u8 = 64;
pub const SCREEN_HEIGHT: u8 = 32;
pub const EDGE_X: u8 = 16;
pub const EDGE_Y: u8 = 8;
pub const HUD_X: u8 = 0;
pub const HUD_Y: u8 = 32;
pub const HUD_WIDTH: u8 = 64;
pub const HUD_HEIGHT: u8 = 12;
pub const MARGIN_X: i32 = 0;
pub const MARGIN_Y: i32 = 1;
pub const REFRESH_TIME: u64 = 10;
pub trait NotFound {
    fn not_found() -> UiTile;
}
#[derive(Clone)]
pub struct UiTile {
    symbol: String,
    color: u8,
}
impl NotFound for UiTile {
    fn not_found() -> UiTile {
        return UiTile {
            symbol: "?".to_string(),
            color: 0,
        };
    }
}
pub struct Curses {
    pub window: Window,
    pub ui_world_map_tiles: HashMap<String, UiTile>,
    pub ui_hud: HashMap<String, UiTile>,
    pub ui_tiles: HashMap<String, UiTile>,
    pub ui_entities: HashMap<String, UiTile>,
}
impl Default for Curses {
    fn default() -> Curses {
        Curses {
            window: initscr(),
            ui_world_map_tiles: HashMap::from([
                (
                    "barren_land".to_string(),
                    UiTile {
                        symbol: ".".to_string(),
                        color: 1,
                    },
                ),
                (
                    "rock_desert".to_string(),
                    UiTile {
                        symbol: "*".to_string(),
                        color: 9,
                    },
                ),
                (
                    "salt_desert".to_string(),
                    UiTile {
                        symbol: "_".to_string(),
                        color: 7,
                    },
                ),
                (
                    "ice_desert".to_string(),
                    UiTile {
                        symbol: "~".to_string(),
                        color: 7,
                    },
                ),
                (
                    "ash_desert".to_string(),
                    UiTile {
                        symbol: "`".to_string(),
                        color: 7,
                    },
                ),
                (
                    "dunes".to_string(),
                    UiTile {
                        symbol: "~".to_string(),
                        color: 1,
                    },
                ),
                (
                    "oasis".to_string(),
                    UiTile {
                        symbol: ".".to_string(),
                        color: 4,
                    },
                ),
            ]),
            ui_hud: HashMap::from([
                (
                    "border".to_string(),
                    UiTile {
                        symbol: " ".to_string(),
                        color: 6,
                    },
                ),
                (
                    "hud_body".to_string(),
                    UiTile {
                        symbol: " ".to_string(),
                        color: 5,
                    },
                ),
                (
                    "hud_text".to_string(),
                    UiTile {
                        symbol: " ".to_string(),
                        color: 3,
                    },
                ),
            ]),
            ui_tiles: HashMap::from([
                (
                    "sand".to_string(),
                    UiTile {
                        symbol: ".".to_string(),
                        color: 1,
                    },
                ),
                (
                    "ice".to_string(),
                    UiTile {
                        symbol: ".".to_string(),
                        color: 7,
                    },
                ),
                (
                    "dune_sand".to_string(),
                    UiTile {
                        symbol: "~".to_string(),
                        color: 1,
                    },
                ),
                (
                    "ash".to_string(),
                    UiTile {
                        symbol: "`".to_string(),
                        color: 9,
                    },
                ),
                (
                    "salt".to_string(),
                    UiTile {
                        symbol: "_".to_string(),
                        color: 7,
                    },
                ),
                (
                    "gravel".to_string(),
                    UiTile {
                        symbol: "*".to_string(),
                        color: 9,
                    },
                ),
                (
                    "rock".to_string(),
                    UiTile {
                        symbol: "^".to_string(),
                        color: 1,
                    },
                ),
                (
                    "water".to_string(),
                    UiTile {
                        symbol: "~".to_string(),
                        color: 2,
                    },
                ),
                (
                    "grass".to_string(),
                    UiTile {
                        symbol: ".".to_string(),
                        color: 4,
                    },
                ),
                (
                    "lava".to_string(),
                    UiTile {
                        symbol: "~".to_string(),
                        color: 10,
                    },
                ),
            ]),
            ui_entities: HashMap::from([
                (
                    "ogre".to_string(),
                    UiTile {
                        symbol: "o".to_string(),
                        color: 3,
                    },
                ),
                (
                    "kobold".to_string(),
                    UiTile {
                        symbol: "p".to_string(),
                        color: 3,
                    },
                ),
                (
                    "goblin".to_string(),
                    UiTile {
                        symbol: "G".to_string(),
                        color: 3,
                    },
                ),
                (
                    "gnoll".to_string(),
                    UiTile {
                        symbol: "g".to_string(),
                        color: 3,
                    },
                ),
                (
                    "rat".to_string(),
                    UiTile {
                        symbol: "r".to_string(),
                        color: 3,
                    },
                ),
                (
                    "scarab".to_string(),
                    UiTile {
                        symbol: "S".to_string(),
                        color: 9,
                    },
                ),
                (
                    "no entity".to_string(),
                    UiTile {
                        symbol: " ".to_string(),
                        color: 3,
                    },
                ),
            ]),
        }
    }
}
impl Curses {
    pub fn init(&mut self) {
        self.window.refresh();
        self.window.keypad(true);
        self.window.timeout(REFRESH_TIME as i32);
        curs_set(0);
        noecho();
        start_color();
        use_default_colors();
        init_pair(1, COLOR_BLACK, COLOR_YELLOW);
        init_pair(2, COLOR_WHITE, COLOR_BLUE);
        init_pair(3, COLOR_WHITE, COLOR_BLACK);
        init_pair(4, COLOR_BLACK, COLOR_GREEN);
        init_pair(5, COLOR_BLACK, COLOR_BLACK);
        init_pair(6, COLOR_WHITE, COLOR_WHITE);
        init_pair(7, COLOR_BLACK, COLOR_WHITE);
        init_pair(8, COLOR_WHITE, COLOR_MAGENTA);
        init_pair(9, COLOR_WHITE, COLOR_BLACK);
        init_pair(10, COLOR_BLACK, COLOR_RED);
    }
    pub fn draw_tile(&self, tile: Tile, rel_x: i32, rel_y: i32) {
        if rel_x < 0
            || rel_y < 0
            || rel_x > SCREEN_WIDTH as i32 - 1
            || rel_y > SCREEN_HEIGHT as i32 - MARGIN_Y
        {
            return;
        }
        self.window.mv(rel_y + MARGIN_Y, rel_x + MARGIN_X);
        let attributes = ColorPair(self.ui_tiles[&tile.tile_type].color);
        self.window.attron(attributes);
        self.window
            .addstr(self.ui_tiles[&tile.tile_type].symbol.clone());
    }
    pub fn draw_entity(&self, entity: Entity, rel_x: i32, rel_y: i32) {
        if rel_x < 0
            || rel_y < 0
            || rel_x > SCREEN_WIDTH as i32 - 1
            || rel_y > SCREEN_HEIGHT as i32 - MARGIN_Y
        {
            return;
        }
        self.window.mv(rel_y + MARGIN_Y, rel_x + MARGIN_X);
        let attributes = ColorPair(
            self.ui_entities
                .get(&entity.entity_type)
                .unwrap_or(&UiTile::not_found())
                .color,
        );
        self.window.attron(attributes);
        self.window.addstr(
            self.ui_entities
                .get(&entity.entity_type)
                .unwrap_or(&UiTile::not_found())
                .symbol
                .clone(),
        );
    }
    pub fn draw_world_tile(&self, tile: WorldMapTile) {
        self.window.mv(tile.y + MARGIN_Y, tile.x + MARGIN_X);

        let attributes = ColorPair(self.ui_world_map_tiles[&tile.chunk_type].color);
        self.window.attron(attributes);
        self.window
            .addstr(self.ui_world_map_tiles[&tile.chunk_type].symbol.clone());
    }
    pub fn draw_cursor(&self, y: i32, x: i32) {
        let mut attributes = Attributes::new();
        attributes.set_blink(true);
        self.window.attron(attributes);
        self.window.mv(y, x);
        self.window.addch('X');
        attributes.set_blink(false);
        self.window.attrset(attributes);
    }
    pub fn draw_str(&self, y: i32, x: i32, content: String) {
        let attributes = ColorPair(self.ui_hud["hud_text"].color);
        self.window.attron(attributes);
        self.window.mv(MARGIN_Y + y, MARGIN_X + x);
        self.window.addstr(&content);
    }
    pub fn draw_str_hud(&self, y: i32, x: i32, content: String) {
        let attributes = ColorPair(self.ui_hud["hud_text"].color);
        self.window.attron(attributes);
        self.window.mv(MARGIN_Y + HUD_Y as i32 + y, MARGIN_X + HUD_X as i32 + x);
        self.window.addstr(&content);
    }
    pub fn draw_hud(&self) {
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
                self.window.attron(attributes);
                self.window.mv(i as i32 + MARGIN_Y, j as i32 + MARGIN_X);
                self.window.addstr(self.ui_hud[hud_element].symbol.clone());
            }
        }

    }
    pub fn end_win(&self) {
        endwin();
    }
    pub fn end_loop(&mut self) {
        self.window.refresh();
        self.window.erase();
    }
}
