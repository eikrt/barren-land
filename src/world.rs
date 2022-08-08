use crate::server::*;
use bincode;
use rand::Rng;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use simdnoise::*;
use std::collections::HashMap;
use std::fs;
use std::io::prelude::*;
use std::path::Path;
use crate::entities::*;
use crate::tiles::*;
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Biome {
    biome: String,
    count: i32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WorldMap {
    pub chunks: Vec<Vec<WorldMapTile>>,
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
impl Default for WorldProperties {
    fn default() -> WorldProperties {
        WorldProperties {
            seed: 1,
            sealevel: 1.0,
            chunk_size: 1,
            world_width: 1,
            world_height: 1,
            name: "".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Tiles {
    pub tiles: Vec<Vec<Tile>>,
    pub x: i32,
    pub y: i32,
    pub biome: String,
}
impl Default for Tiles {
    fn default() -> Tiles {
        Tiles {
            tiles: Vec::new(),
            x: 0,
            y: 0,
            biome: "No Biome".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug)]
pub struct World {
    pub tilemap: Vec<Vec<Tiles>>,
}
fn get_generated_chunk(
    seed: i32,
    _sealevel: f32,
    chunk_size: u32,
    world_width: u32,
    world_height: u32,
    x: i32,
    y: i32,
) -> (Tiles, Entities) {
    let mut tiles_vec = Vec::new();
    let mut entities_map = HashMap::new();
    let mut rng = rand::thread_rng();
    let height_noise = NoiseBuilder::fbm_2d(
        (chunk_size * world_width).try_into().unwrap(),
        (chunk_size * world_height).try_into().unwrap(),
    )
    .with_freq(0.05)
    .with_octaves(8.0 as u8)
    .with_gain(2.0)
    .with_seed(seed * 2)
    .with_lacunarity(0.8)
    .generate_scaled(0.0, 512.0);
    let biome_noise_1 = NoiseBuilder::fbm_2d(
        (chunk_size * world_width).try_into().unwrap(),
        (chunk_size * world_height).try_into().unwrap(),
    )
    .with_freq(0.05)
    .with_octaves(3.0 as u8)
    .with_gain(1.0)
    .with_seed(seed * 4)
    .with_lacunarity(0.2)
    .generate_scaled(0.0, 512.0);
    let biome_noise_2 = NoiseBuilder::fbm_2d(
        (chunk_size * world_width).try_into().unwrap(),
        (chunk_size * world_height).try_into().unwrap(),
    )
    .with_freq(0.05)
    .with_octaves(4.0 as u8)
    .with_gain(1.0)
    .with_seed(seed * 5)
    .with_lacunarity(0.2)
    .generate_scaled(0.0, 512.0);
    let biome_noise_3 = NoiseBuilder::fbm_2d(
        (chunk_size * world_width).try_into().unwrap(),
        (chunk_size * world_height).try_into().unwrap(),
    )
    .with_freq(0.05)
    .with_octaves(5.0 as u8)
    .with_gain(1.0)
    .with_seed(seed * 6)
    .with_lacunarity(0.2)
    .generate_scaled(0.0, 512.0);
    let biome_noise_4 = NoiseBuilder::fbm_2d(
        (chunk_size * world_width).try_into().unwrap(),
        (chunk_size * world_height).try_into().unwrap(),
    )
    .with_freq(0.05)
    .with_octaves(6.0 as u8)
    .with_gain(1.0)
    .with_seed(seed * 7)
    .with_lacunarity(0.2)
    .generate_scaled(0.0, 512.0);
    let biome_noise_5 = NoiseBuilder::fbm_2d(
        (chunk_size * world_width).try_into().unwrap(),
        (chunk_size * world_height).try_into().unwrap(),
    )
    .with_freq(0.05)
    .with_octaves(7.0 as u8)
    .with_gain(1.0)
    .with_seed(seed * 8)
    .with_lacunarity(0.2)
    .generate_scaled(0.0, 512.0);
    let biome_threshold = 256.0;
    let mut biome_counts: HashMap<String, i32> = HashMap::new();

    biome_counts.insert("dunes".to_string(), 0);
    biome_counts.insert("ash_desert".to_string(), 0);
    biome_counts.insert("salt_desert".to_string(), 0);
    biome_counts.insert("ice_desert".to_string(), 0);
    biome_counts.insert("rock_desert".to_string(), 0);
    biome_counts.insert("barren_land".to_string(), 0);
    for i in 0..chunk_size {
        tiles_vec.push(Vec::new());
        for j in 0..chunk_size {
            let mut biome = "barren_land".to_string();
            let tile_x = j as i32 + x * chunk_size as i32;
            let tile_y = i as i32 + y * chunk_size as i32;
            let perlin_coord =
                ((tile_x + tile_y * world_width as i32 * chunk_size as i32) as i32) as usize;
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
            if biome_noise_1[perlin_coord] < biome_threshold {
                biome = "dunes".to_string();
            } else if biome_noise_2[perlin_coord] < biome_threshold {
                biome = "ash_desert".to_string();
            } else if biome_noise_3[perlin_coord] < biome_threshold {
                biome = "salt_desert".to_string();
            } else if biome_noise_4[perlin_coord] < biome_threshold {
                biome = "ice_desert".to_string();
            } else if biome_noise_5[perlin_coord] < biome_threshold {
                biome = "rock_desert".to_string();
            }
            *biome_counts.get_mut(&biome).unwrap() += 1;
            let id: u64 = rng.gen::<u64>();
            if true {
                //tile.h < sealevel {
                match biome.as_str() {
                    "dunes" => {
                        tile.tile_type = "dune_sand".to_string();
                        let biome_entity = Entity::scarab(tile_x, tile_y, j as i32, i as i32, x, y, id, "scarab".to_string(), "scarab".to_string());
                        if rng.gen_range(0..32) == 1 {
                            entities_map.insert(id, biome_entity);
                        }
                    },
                    "ash_desert" => tile.tile_type = "ash".to_string(),
                    "salt_desert" => tile.tile_type = "salt".to_string(),
                    "ice_desert" => tile.tile_type = "ice".to_string(),
                    "rock_desert" => tile.tile_type = "gravel".to_string(),
                    "barren_land" => tile.tile_type = "sand".to_string(),
                    _ => tile.tile_type = "sand".to_string(),
                };
            }
            /* let mut entity = Entity {
                x: tile_x,
                y: tile_y,
                relative_x: j as i32,
                relative_y: i as i32,
                chunk_x: x,
                chunk_y: y,
                entity_type: "ogre".to_string(),

            };
            */
            tiles_vec[i as usize].push(tile);
        }
    }
    let max_biomes = biome_counts.values().max().unwrap();
    let mut biome = "barren_land".to_string();
    for (key, val) in biome_counts.iter() {
        if val == max_biomes {
            biome = key.to_string();
        }
    }
    let tiles_ret = Tiles {
        tiles: tiles_vec,
        x: x,
        y: y,
        biome: biome,
    };
    let entities_ret = Entities {
        entities: entities_map,
        x: x,
        y: y,
    };
    return (tiles_ret, entities_ret);
}
pub fn generate_world(
    seed: i32,
    chunk_size: u32,
    world_width: u32,
    world_height: u32,
    sealevel: f32,
    name: String,
) {
    generate_chunks(seed, chunk_size, world_width, world_height, sealevel);
    write_world_properties(seed, chunk_size, world_width, world_height, sealevel, name);
    write_client_ids_to_file(ClientIds::default());
}
pub fn write_world_properties(
    seed: i32,
    chunk_size: u32,
    world_width: u32,
    world_height: u32,
    sealevel: f32,
    name: String,
) {
    let w_p = WorldProperties {
        seed: seed,
        chunk_size: chunk_size,
        world_width: world_width,
        world_height: world_height,
        sealevel: sealevel,
        name: name,
    };
    let mut world_properties_file = fs::File::create("world/world_properties.dat").unwrap();
    let encoded: Vec<u8> = bincode::serialize(&w_p).unwrap();
    world_properties_file.write_all(&encoded);
}
fn generate_chunks(seed: i32, chunk_size: u32, world_width: u32, world_height: u32, _sealevel: f32) {
    (0..world_height).into_par_iter().for_each(|i| {
        (0..world_width).into_par_iter().for_each(|j| {
            let (generated_tiles, generated_entities) = get_generated_chunk(
                seed,
                _sealevel,
                chunk_size,
                world_width,
                world_height,
                j as i32,
                i as i32,
            );
            let chunk_dir_path = format!("world/chunks/chunk_{}_{}", j, i);
            if !Path::new(&chunk_dir_path).exists() {
                fs::create_dir(&chunk_dir_path).unwrap();
            }
            let mut tiles_file =
                fs::File::create(format!("world/chunks/chunk_{}_{}/tiles.dat", j, i)).unwrap();
            let encoded: Vec<u8> = bincode::serialize(&generated_tiles).unwrap();

            tiles_file.write_all(&encoded);
            let mut entities_file =
                fs::File::create(format!("world/chunks/chunk_{}_{}/entities.dat", j, i)).unwrap();
            let encoded: Vec<u8> = bincode::serialize(&generated_entities).unwrap();

            entities_file.write_all(&encoded);
            write_world_map_to_file(j, i, generated_tiles.biome);
        });
    });
}
fn write_world_map_to_file(x: u32, y: u32, chunk_type: String) {
    let mut world_map_file =
        fs::File::create(format!("world/chunks/chunk_{}_{}/world_map.dat", x, y)).unwrap();
    let encoded: Vec<u8> = bincode::serialize(&WorldMapTile {
        x: x as i32,
        y: y as i32,
        chunk_type: chunk_type,
    })
    .unwrap();

    world_map_file.write_all(&encoded);
}
