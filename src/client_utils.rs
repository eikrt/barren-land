use crate::entities::Player;
use crate::queue::PostData;
use crate::server::ClientId;
use crate::world::{Entities, Entity, Tile, Tiles, World, WorldMap, WorldMapTile, WorldProperties};
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

pub fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}
pub fn open_tiles(x: i32, y: i32) -> Tiles {
    let path = format!("world/chunks/chunk_{}_{}/tiles.dat", x, y);
    let t = fs::read(path).unwrap();

    let decoded: Tiles = bincode::deserialize(&t).unwrap();
    return decoded;
}
pub async fn load_tiles(client: reqwest::Client, x: i32, y: i32) -> Tiles {
    let resp = client
        .get(format!("http://localhost:8080/tiles/{}/{}", x, y))
        .send()
        .await
        .unwrap();
    let body = resp.text().await.unwrap();

    let decoded = serde_json::from_str(&body).unwrap();
    return decoded;
}
pub async fn load_world_map_tile(client: reqwest::Client, x: i32, y: i32) -> WorldMapTile {
    let resp = client
        .get(format!("http://localhost:8080/world_map/{}/{}", x, y))
        .send()
        .await
        .unwrap();
    let body = resp.text().await.unwrap();

    let decoded = serde_json::from_str(&body).unwrap();
    return decoded;
}
pub async fn load_chunk_tile(client: reqwest::Client, x: i32, y: i32) -> WorldMapTile {
    let resp = client
        .get(format!("http://localhost:8080/world_map/{}/{}", x, y))
        .send()
        .await
        .unwrap();
    let body = resp.text().await.unwrap();

    let decoded = serde_json::from_str(&body).unwrap();
    return decoded;
}
pub async fn load_entities(client: reqwest::Client, x: i32, y: i32) -> Entities {
    let resp = client
        .get(format!("http://localhost:8080/entities/{}/{}", x, y))
        .send()
        .await;
    let resp = match resp {
        Ok(r) => r,
        Err(e) => {
            endwin();
            panic!();
        }
    };
    let body = resp.text().await.unwrap();
    let decoded = serde_json::from_str(&body).unwrap();
    return decoded;
}
pub async fn load_player(client: reqwest::Client, x: i32, y: i32, id: u64) -> Entity {
    let resp = client
        .get("http://localhost:8080/entities/{}/{}")
        .send()
        .await
        .unwrap();
    let body = resp.text().await.unwrap();
    let decoded: Entities = serde_json::from_str(&body).unwrap();
    return decoded.entities.get(&id).unwrap().clone();
}
pub async fn load_properties(client: reqwest::Client) -> WorldProperties {
    let resp = client
        .get("http://localhost:8080/world_properties")
        .send()
        .await
        .unwrap();
    let body = resp.text().await.unwrap();
    let decoded = serde_json::from_str(&body).unwrap();
    return decoded;
}
pub async fn load_check_if_client_with_id(
    client: reqwest::Client,
    username: String,
    id: u64,
    chunk_x: i32,
    chunk_y: i32,
) -> bool {
    let resp = client
        .get(format!(
            "http://localhost:8080/client_exists/{}/{}/{}/{}",
            username, id, chunk_x, chunk_y
        ))
        .send()
        .await
        .unwrap();
    let body = resp.text().await.unwrap();
    let decoded = body.parse().unwrap();
    return decoded;
}
pub async fn load_search_entity_clientid(
    client: reqwest::Client,
    username: String,
    id: u64,
) -> ClientId {
    let resp = client
        .get(format!("http://localhost:8080/search_entity/{}", username))
        .send()
        .await
        .unwrap();
    let body = resp.text().await.unwrap();
    let decoded = serde_json::from_str(&body).unwrap();
    return decoded;
}
pub async fn load_world_properties(
    client: reqwest::Client,
) -> WorldProperties{
    let resp = client
        .get(format!("http://localhost:8080/world_properties"))
        .send()
        .await
        .unwrap();
    let body = resp.text().await.unwrap();
    let decoded = serde_json::from_str(&body).unwrap();
    return decoded;
}
pub async fn post_to_queue(client: reqwest::Client, action: PostData) {
    let res = client
        .post("http://localhost:8080/queue")
        .json(&action)
        .send()
        .await;
}
