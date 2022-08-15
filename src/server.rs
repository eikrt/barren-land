use crate::queue::*;
use crate::world::*;
use crate::entities::*;
use crate::tiles::*;
use actix_web::*;
use bincode;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::sync::{Arc, Mutex, RwLock};
use std::{thread, time};

#[derive(Serialize, Deserialize)]
struct ChunkGetData {
    x: i32,
    y: i32,
}
#[derive(Serialize, Deserialize)]
pub struct IdQueryData {
    id: u64,
    username: String,
    chunk_x: i32,
    chunk_y: i32,
}
#[derive(Serialize, Deserialize)]
pub struct EntityListQueryData {
    username: String,
}
#[derive(Serialize, Deserialize, Clone)]
pub struct ClientId {
    pub id: u64,
    pub username: String,
    pub chunk_x: i32,
    pub chunk_y: i32,
}
impl Default for ClientId {
    fn default() -> ClientId {
        ClientId {
            id: 0,
            username: "Default username".to_string(),
            chunk_x: 0,
            chunk_y: 0,
        }
    }
}
#[derive(Serialize, Deserialize, Clone)]
pub struct ClientIds {
    pub ids: HashMap<String, ClientId>,
}
impl Default for ClientIds {
    fn default() -> ClientIds {
        ClientIds {
            ids: HashMap::new(),
        }
    }
}
pub fn open_tiles(x: i32, y: i32) -> String {
    let path = format!("world/chunks/chunk_{}_{}/tiles.dat", x, y);
    let body = fs::read(path).unwrap();
    let decoded: Tiles = bincode::deserialize(&body).unwrap_or(Tiles::default());
    let encoded = serde_json::to_string(&decoded).unwrap();
    return encoded;
}
pub fn open_tiles_as_struct(x: i32, y: i32) -> Result<Tiles, std::io::Error> {
    let path = format!("world/chunks/chunk_{}_{}/tiles.dat", x, y);
    let body = fs::read(path)?;
    Ok(bincode::deserialize(&body).unwrap())
}
pub fn open_entities(x: i32, y: i32) -> String {
    let path = format!("world/chunks/chunk_{}_{}/entities.dat", x, y);
    let body = fs::read(path).unwrap();
    let decoded: Entities = bincode::deserialize(&body).unwrap_or(Entities::default());
    let encoded = serde_json::to_string(&decoded).unwrap();
    return encoded;
}
pub fn open_entities_as_struct(x: i32, y: i32) -> Result<Entities, Error> {
    let path = format!("world/chunks/chunk_{}_{}/entities.dat", x, y);
    let body = fs::read(path)?;
    Ok(bincode::deserialize(&body).unwrap())
}
pub fn open_world_properties() -> String {
    let path = "world/world_properties.dat";
    let body = fs::read(path).unwrap();
    let decoded: WorldProperties = bincode::deserialize(&body).unwrap();
    let encoded = serde_json::to_string(&decoded).unwrap();
    return encoded;
}
pub fn open_map_tile_for_chunks(x: i32, y: i32) -> String {
    let path = format!("world/chunks/chunk_{}_{}/world_map.dat", x, y);
    let body = fs::read(path).unwrap();
    let decoded: WorldMapTile = bincode::deserialize(&body).unwrap();
    let encoded = serde_json::to_string(&decoded).unwrap();
    return encoded;
}
pub fn open_map_tile_for_chunks_as_struct(x: i32, y: i32) -> WorldMapTile {
    let path = format!("world/chunks/chunk_{}_{}/world_map.dat", x, y);
    let body = fs::read(path).unwrap();
    let decoded: WorldMapTile = bincode::deserialize(&body).unwrap();
    return decoded;
}
pub fn open_client_ids_to_struct() -> ClientIds {
    let path = "world/client_ids.dat";
    let body = fs::read(path).unwrap_or(Vec::new());
    let decoded: ClientIds = bincode::deserialize(&body).unwrap_or(ClientIds::default());
    return decoded;
}
pub fn open_client_ids() -> String {
    let path = "world/client_ids.dat";
    let body = fs::read(path).unwrap();
    let decoded: ClientIds = bincode::deserialize(&body).unwrap();
    let encoded = serde_json::to_string(&decoded).unwrap();
    return encoded;
}
pub fn add_client_id(username: String, id: u64, chunk_x: i32, chunk_y: i32) {
    let mut client_ids = open_client_ids_to_struct();
    client_ids.ids.insert(
        username.clone(),
        ClientId {
            username: username,
            id: id,
            chunk_x: chunk_x,
            chunk_y: chunk_y,
        },
    );
    write_client_ids_to_file(client_ids);
}
pub fn open_world_properties_to_struct() -> WorldProperties {
    let body = open_world_properties();
    let decoded = serde_json::from_str(&body).unwrap();
    return decoded;
}
pub fn search_entity_by_username_as_string(username: String) -> String {
    let client_ids = open_client_ids_to_struct();
    let default_clientid = ClientId::default();
    let body = client_ids.ids.get(&username).unwrap_or(&default_clientid);
    let encoded = serde_json::to_string(&body).unwrap();
    println!("{}", encoded);
    return encoded;
}

pub fn write_client_ids_to_file(client_ids: ClientIds) {
    let mut ids_file = fs::File::create(format!("world/client_ids.dat")).unwrap();
    let encoded: Vec<u8> = bincode::serialize(&client_ids).unwrap();

    ids_file.write_all(&encoded);
}
pub fn check_if_client_exists(username: String, _id: u64) -> bool {
    let mut exists = false;
    let ids = open_client_ids_to_struct();
    if ids.ids.contains_key(&username) {
        exists = true;
    }
    return exists;
}
#[get("/tiles/{x}/{y}")]
async fn tiles(data: web::Path<ChunkGetData>) -> impl Responder {
    let contents = open_tiles(data.x, data.y);
    HttpResponse::Ok().body(contents)
}
#[get("/entities/{x}/{y}")]
async fn entities(data: web::Path<ChunkGetData>) -> impl Responder {
    let contents = open_entities(data.x, data.y);
    HttpResponse::Ok().body(contents)
}
#[get("/world_properties")]
async fn world_properties(_req: HttpRequest) -> impl Responder {
    let contents = open_world_properties();
    HttpResponse::Ok().body(contents)
}
#[get("/world_map/{x}/{y}")]
async fn world_map(data: web::Path<ChunkGetData>) -> impl Responder {
    let contents = open_map_tile_for_chunks(data.x, data.y);
    HttpResponse::Ok().body(contents)
}
#[get("/client_exists/{username}/{id}/{chunk_x}/{chunk_y}")]
async fn client_exists(data: web::Path<IdQueryData>) -> impl Responder {
    let exists = check_if_client_exists(data.username.clone(), data.id);
    let contents = format!("{}", exists);
    if !exists {
        add_client_id(data.username.clone(), data.id, data.chunk_x, data.chunk_y);
    }
    HttpResponse::Ok().body(contents)
}
#[get("/search_entity/{username}")]
async fn search_entity(data: web::Path<EntityListQueryData>) -> impl Responder {
    let contents = search_entity_by_username_as_string(data.username.clone());
    HttpResponse::Ok().body(contents)
}
#[post("/queue")]
async fn post_queue(q: web::Data<Mutex<ActionQueue>>, post: web::Json<PostData>) -> impl Responder {
    add_to_queue(q, queue_to_object(post));
    HttpResponse::Ok()
}
#[actix_web::main] // or #[tokio::main]
pub async fn main() -> std::io::Result<()> {
    let action_queue = Arc::new(RwLock::new(web::Data::new(Mutex::new(ActionQueue {
        queue: Vec::new(),
    }))));
    let process_queue = action_queue.clone();
    let exe_queue = action_queue.clone();
    let server_queue = action_queue.clone();
    thread::spawn(move || loop {
        process_entities(process_queue.read().unwrap().clone());
        thread::sleep(time::Duration::from_millis(1000));
    });
    thread::spawn(move || loop {
        execute_queue(exe_queue.read().unwrap().clone());
    });
    HttpServer::new(move || {
        App::new()
            .app_data(server_queue.read().unwrap().clone())
            .wrap(middleware::Compress::default())
            .service(world_properties)
            .service(client_exists)
            .service(tiles)
            .service(entities)
            .service(post_queue)
            .service(world_map)
            .service(search_entity)
    })
    .bind(("127.0.0.1", 8081))?
    .run()
    .await
}
