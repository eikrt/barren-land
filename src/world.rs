use rand::seq::IteratorRandom;
use rand::Rng;
use simdnoise::*;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::prelude::*;
use rayon::prelude::*;
use bincode;
use serde::{Serialize, Deserialize};
use std::path::Path;
#[derive(Serialize, Deserialize, Debug)]
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
#[derive(Serialize, Deserialize, Debug)]
pub struct Entity {
    pub x: i32,
    pub y: i32,
    pub relative_x: i32,
    pub relative_y: i32,
    pub chunk_x: i32,
    pub chunk_y: i32,
    pub entity_type: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct WorldProperties {
    pub seed: i32,
    pub sealevel: f32,
    pub chunk_size: u32,
    pub world_width: u32,
    pub world_height: u32,
    pub name: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Tiles {
    pub tiles: Vec<Vec<Tile>>
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Entities {
    pub entities: HashMap<u32, Entity>
}
#[derive(Serialize, Deserialize, Debug)]
pub struct World {
    pub tilemap: Vec<Vec<Tiles>>
}
fn get_generated_chunk(seed: i32, sealevel: f32,chunk_size: u32, world_width: u32, world_height: u32, x: i32, y: i32) -> (Tiles, Entities) {
    let mut tiles = Vec::new();
    let mut entities = HashMap::new();
    let mut rng = rand::thread_rng();
    let ground_noise = NoiseBuilder::fbm_2d((chunk_size * world_width).try_into().unwrap(), (chunk_size * world_height).try_into().unwrap())
        .with_freq(0.15)
        .with_octaves(9.0 as u8)
        .with_gain(2.0)
        .with_seed(seed)
        .with_lacunarity(0.8)
        .generate_scaled(0.0, 512.0);
    let height_noise = NoiseBuilder::fbm_2d((chunk_size * world_width).try_into().unwrap(), (chunk_size * world_height).try_into().unwrap())
        .with_freq(0.05)
        .with_octaves(8.0 as u8)
        .with_gain(2.0)
        .with_seed(seed * 2)
        .with_lacunarity(0.8)
        .generate_scaled(0.0, 512.0);
    let npc_noise = NoiseBuilder::fbm_2d((chunk_size * world_width).try_into().unwrap(), (chunk_size * world_height).try_into().unwrap())
        .with_freq(0.85)
        .with_octaves(8.0 as u8)
        .with_gain(2.0)
        .with_seed(seed * 3)
        .with_lacunarity(0.8)
        .generate_scaled(0.0, 512.0);
    for i in 0..chunk_size {
        tiles.push(Vec::new());
        for j in 0..chunk_size {
            let tile_x = (j as i32 + x*chunk_size as i32);
            let tile_y = (i as i32 + y*chunk_size as i32);
            let perlin_coord = ((tile_x + tile_y * world_width as i32 *chunk_size as i32 ) as i32) as usize;
            let mut tile = Tile {
                x: tile_x,
                y: tile_y,
                relative_x: j as i32,
                relative_y: i as i32,
                chunk_x: x,
                chunk_y: y,
                h: height_noise[perlin_coord],
                tile_type: "rock".to_string(),
            };
            if tile.h < sealevel {
                tile.tile_type = "sand".to_string();
            }
            let mut entity = Entity {
                x: tile_x,
                y: tile_y,
                relative_x: j as i32,
                relative_y: i as i32,
                chunk_x: x,
                chunk_y: y,
                entity_type: "ogre".to_string(),
            };
            
            tiles[i as usize].push(tile);
            if npc_noise[perlin_coord] < 300.0 {
                let id: u32 = rng.gen::<u32>(); 
                //entities.insert(id, entity);
            }
        }
    }
    let tiles = Tiles {
        tiles: tiles
    };
    let entities = Entities {
        entities: entities
    };
    return (tiles, entities);
} 
pub fn generate_world(seed: i32, chunk_size: u32, world_width: u32, world_height: u32, sealevel: f32, name: String) {
    generate_chunks(seed,chunk_size,world_width,world_height, sealevel);
    write_world_properties(seed,chunk_size,world_width,world_height, sealevel, name);
}
pub fn write_world_properties(seed: i32, chunk_size: u32, world_width: u32, world_height: u32, sealevel: f32, name: String) {
    let world_properties = WorldProperties {
        seed: seed,
        chunk_size: chunk_size,
        world_width: world_width,
        world_height: world_height,
        sealevel: sealevel,
        name: name,
    };
    let mut world_properties_file = fs::File::create("world/world_properties.dat").unwrap();
    let encoded: Vec<u8> = bincode::serialize(&world_properties).unwrap();
    world_properties_file.write_all(&encoded);
}
fn generate_chunks(seed: i32, chunk_size: u32, world_width: u32, world_height: u32, sealevel: f32)  {
    (0..world_width).into_par_iter().for_each(|i| {
        (0..world_height).into_par_iter().for_each(|j| {

            let (generated_tiles, generated_entities) = get_generated_chunk(seed,sealevel,chunk_size,world_width,world_height, j as i32,i as i32);
            let chunk_dir_path = format!("world/chunks/chunk_{}_{}",j,i);
            if !Path::new(&chunk_dir_path).exists() {
                fs::create_dir(&chunk_dir_path).unwrap();
            }
            let mut tiles_file = fs::File::create(format!("world/chunks/chunk_{}_{}/tiles.dat",j,i)).unwrap();
            let encoded: Vec<u8> = bincode::serialize(&generated_tiles).unwrap();

            tiles_file.write_all(&encoded);
            let mut entities_file = fs::File::create(format!("world/chunks/chunk_{}_{}/entities.dat",j,i)).unwrap();
            let encoded: Vec<u8> = bincode::serialize(&generated_entities).unwrap();

            entities_file.write_all(&encoded);

        });
    });
}
