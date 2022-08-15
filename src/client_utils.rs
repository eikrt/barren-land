use crate::queue::PostData;
use crate::server::ClientId;
use crate::world::*;
use crate::entities::*;
use crate::tiles::*;
use std::error::Error;
use bincode;
use std::collections::hash_map::DefaultHasher;
use std::fs;
use std::hash::{Hash, Hasher};
use pancurses::*;
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
        .get(format!("http://localhost:8081/tiles/{}/{}", x, y))
        .send()
        .await
        .unwrap();
    let body = resp.text().await.unwrap();

    let decoded = serde_json::from_str(&body).unwrap();
    return decoded;
}
pub async fn load_world_map_tile(client: reqwest::Client, x: i32, y: i32) -> WorldMapTile {
    let resp = client
        .get(format!("http://localhost:8081/world_map/{}/{}", x, y))
        .send()
        .await
        .unwrap();
    let body = resp.text().await.unwrap();

    let decoded = serde_json::from_str(&body).unwrap();
    return decoded;
}
pub async fn load_chunk_tile(client: reqwest::Client, x: i32, y: i32) -> WorldMapTile {
    let resp = client
        .get(format!("http://localhost:8081/world_map/{}/{}", x, y))
        .send()
        .await
        .unwrap();
    let body = resp.text().await.unwrap();

    let decoded = serde_json::from_str(&body).unwrap();
    return decoded;
}
pub async fn load_entities(client: reqwest::Client, x: i32, y: i32) -> Entities {
    let resp = client
        .get(format!("http://localhost:8081/entities/{}/{}", x, y))
        .send()
        .await;
    let resp = match resp {
        Ok(r) => r,
        Err(_e) => {
            endwin();
            panic!();
        }
    };
    let body = resp.text().await.unwrap();
    let decoded = serde_json::from_str(&body).unwrap();
    return decoded;
}
pub async fn load_player(client: reqwest::Client, _x: i32, _y: i32, id: u64) -> Entity {
    let resp = client
        .get("http://localhost:8081/entities/{}/{}")
        .send()
        .await
        .unwrap();
    let body = resp.text().await.unwrap();
    let decoded: Entities = serde_json::from_str(&body).unwrap();
    return decoded.entities.get(&id).unwrap().clone();
}
pub async fn load_properties(client: reqwest::Client) -> WorldProperties {
    let resp = client
        .get("http://localhost:8081/world_properties")
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
            "http://localhost:8081/client_exists/{}/{}/{}/{}",
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
    _id: u64,
) -> ClientId {
    let resp = client
        .get(format!("http://localhost:8081/search_entity/{}", username))
        .send()
        .await
        .unwrap();
    let body = resp.text().await.unwrap();
    let decoded = serde_json::from_str(&body).unwrap();
    return decoded;
}
pub async fn load_world_properties(
    client: reqwest::Client,
) -> Result<WorldProperties, Box<dyn Error>>{
    let resp = client
        .get(format!("http://localhost:8081/world_properties"))
        .send()
        .await?;
    let body = resp.text().await.unwrap();
    let decoded = serde_json::from_str(&body).unwrap();
    Ok(decoded)
}
pub async fn post_to_queue(client: reqwest::Client, action: PostData) {
    let _res = client
        .post("http://localhost:8081/queue")
        .json(&action)
        .send()
        .await;
}
