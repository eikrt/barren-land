use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Tile {
    pub x: i32,
    pub y: i32,
    pub h: f32,
    pub relative_x: i32,
    pub relative_y: i32,
    pub chunk_x: i32,
    pub chunk_y: i32,
    pub tile_type: String,
}
impl Default for Tile {
    fn default() -> Tile {
        Tile {
            x: 1,
            y: 1,
            h: 1.0,
            relative_x: 1,
            relative_y: 1,
            chunk_x: 1,
            chunk_y: 1,
            tile_type: "sand".to_string()
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WorldMapTile {
    pub x: i32,
    pub y: i32,
    pub chunk_type: String,
}
