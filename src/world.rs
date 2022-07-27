use rand::seq::IteratorRandom;
use rand::Rng;
use simdnoise::*;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use rayon::prelude::*;
use bincode;
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct Tile {
    pub x: i32,
    pub y: i32,
    pub h: f32,
    pub tile_type: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Chunk {
    pub tiles: Vec<Vec<Tile>>
}
#[derive(Serialize, Deserialize, Debug)]
pub struct World {
    pub chunks: Vec<Vec<Chunk>>
}
fn generate_chunk(seed: i32, sealevel: f32,chunk_size: u32, world_width: u32, world_height: u32, x: i32, y: i32) -> Chunk {
    let mut tiles = Vec::new();
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
    for i in 0..chunk_size {
        tiles.push(Vec::new());
        for j in 0..chunk_size {
            let tile_x = (j as i32 + x*chunk_size as i32);
            let tile_y = (i as i32 + y*chunk_size as i32);
            let mut tile = Tile {
                x: tile_x,
                y: tile_y,
                h: height_noise[((tile_x + tile_y * world_width as i32 *chunk_size as i32 ) as i32) as usize],
                tile_type: "rock".to_string(),
            };
            if tile.h < sealevel {
                tile.tile_type = "sand".to_string();
            } 
            tiles[i as usize].push(tile);
        }
    }
    let chunk = Chunk {
        tiles: tiles
    };
    return chunk;
} 
fn get_generated_chunks(seed: i32, chunk_size: u32, world_width: u32, world_height: u32, sealevel: f32) -> Vec<Vec<Chunk>>{
    let mut chunks = vec![];
    for i in 0..world_width {
        chunks.push(vec![]);
        for j in 0..world_height {
            chunks[i as usize].push(generate_chunk(seed,sealevel,chunk_size,world_width,world_height, i as i32,j as i32));
        }
    }
    return chunks;
}
pub fn get_generated_world(seed: i32, chunk_size: u32, world_width: u32, world_height: u32, sealevel: f32, name: String) -> World {
    let world = World {
        chunks: get_generated_chunks(seed,chunk_size,world_width,world_height, sealevel)
    };
    return world;
}
pub fn generate_world(seed: i32, chunk_size: u32, world_width: u32, world_height: u32, sealevel: f32, name: String) {
    generate_chunks(seed,chunk_size,world_width,world_height, sealevel)
}
fn generate_chunks(seed: i32, chunk_size: u32, world_width: u32, world_height: u32, sealevel: f32)  {
    (0..world_width).into_par_iter().for_each(|i| {
        (0..world_height).into_par_iter().for_each(|j| {

            let chunk = generate_chunk(seed,sealevel,chunk_size,world_width,world_height, j as i32,i as i32);
            let mut file = File::create(format!("world/chunks/chunk_{}_{}",j,i)).unwrap();
            let encoded: Vec<u8> = bincode::serialize(&chunk).unwrap();

            file.write_all(&encoded);

        });
    });
}
