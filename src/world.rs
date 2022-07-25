use rand::seq::IteratorRandom;
use rand::Rng;
use simdnoise::*;
use std::collections::HashMap;
use std::env;
use std::fs;


pub struct Tile {
    pub x: f32,
    pub y: f32,
    pub h: f32,
    pub current_sprite: String,
}
pub struct Chunk {
    pub tiles: Vec<Vec<Tile>>
}
pub struct World {
    pub chunks: Vec<Vec<Chunk>>
}
fn generate_chunk(seed: i32, chunk_size: u32, world_width: u32, world_height: u32, x: i32, y: i32) -> Chunk {
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
        .with_freq(1000.15)
        .with_octaves(16.0 as u8)
        .with_gain(2.0)
        .with_seed(seed * 2)
        .with_lacunarity(0.4)
        .generate_scaled(0.0, 512.0);
    for i in 0..chunk_size {
        tiles.push(Vec::new());
        for j in 0..chunk_size {
            let tile_x = (i as i32 + x*chunk_size as i32) as f32;
            let tile_y = (j as i32 + y*chunk_size as i32) as f32;
            let tile = Tile {
                x: tile_x,
                y: tile_y,
                h: height_noise[(tile_x + tile_y * chunk_size as f32 * x as f32) as usize],
                current_sprite: "grass".to_string(),
            };
            
            tiles[i as usize].push(tile);
        }
    }
    let chunk = Chunk {
        tiles: tiles
    };
    return chunk;
} 
fn get_generated_chunks(seed: i32, chunk_size: u32, world_width: u32, world_height: u32) -> Vec<Vec<Chunk>>{
    let mut chunks = vec![];
    for i in 0..world_width {
        chunks.push(vec![]);
        for j in 0..world_height {
            chunks[i as usize].push(generate_chunk(seed,chunk_size,world_width,world_height, i as i32,j as i32));
        }
    }
    return chunks;
}
pub fn get_generated_world() -> World {
    let world = World {
        chunks: get_generated_chunks(100, 32, 2,2)
    }; 
    return world;
}
