use actix_web::*;
use serde::{Deserialize, Serialize};
use crate::world::*;
use crate::queue::*;
use std::{thread,time};
use std::fs;
use bincode;
use once_cell::sync::Lazy;
use std::sync::{Mutex, Arc, RwLock};

#[derive(Serialize, Deserialize)]
struct ChunkGetData {
    x: i32,
    y: i32,
}

pub fn open_tiles(x: i32, y: i32) -> String {
    let path = format!("world/chunks/chunk_{}_{}/tiles.dat",x,y);
    let body = fs::read(path).unwrap();
    let decoded: Tiles = bincode::deserialize(&body).unwrap();
    let encoded = serde_json::to_string(&decoded).unwrap();
    return encoded; 
}
pub fn open_entities(x: i32, y: i32) -> String {
    let path = format!("world/chunks/chunk_{}_{}/entities.dat",x,y);
    let body = fs::read(path).unwrap();
    let decoded: Entities = bincode::deserialize(&body).unwrap();
    let encoded = serde_json::to_string(&decoded).unwrap();
    return encoded; 
}
pub fn open_entities_as_struct(x: i32, y: i32) -> Entities {
    let path = format!("world/chunks/chunk_{}_{}/entities.dat",x,y);
    let body = fs::read(path).unwrap();
    let decoded: Entities = bincode::deserialize(&body).unwrap();
    let encoded = serde_json::to_string(&decoded).unwrap();
    return decoded; 
}
pub fn open_world_properties() -> String {
    let path = "world/world_properties.dat";
    let body = fs::read(path).unwrap();
    let decoded: WorldProperties = bincode::deserialize(&body).unwrap();
    let encoded = serde_json::to_string(&decoded).unwrap();
    return encoded; 
}
pub fn open_world_properties_to_struct() -> WorldProperties {
    let body = open_world_properties();
    let decoded = serde_json::from_str(&body).unwrap(); 
    return decoded;
}
#[get("/tiles/{x}/{y}")]
async fn tiles(data: web::Path<ChunkGetData>) -> impl Responder {
    let contents = open_tiles(data.x,data.y);
    HttpResponse::Ok()
        .body(contents)
}
#[get("/entities/{x}/{y}")]
async fn entities(data: web::Path<ChunkGetData>) -> impl Responder {
    let contents = open_entities(data.x,data.y);
    HttpResponse::Ok()
        .body(contents)
}
#[get("/world_properties")]
async fn world_properties(_req: HttpRequest) -> impl Responder {
    let contents = open_world_properties();
    HttpResponse::Ok()
        .body(contents)
}
#[post("/queue")]
async fn post_queue(q: web::Data<Mutex<ActionQueue>>, post: web::Json<PostData>) -> impl Responder {
    add_to_queue(q,queue_to_object(post));
    HttpResponse::Ok()
}
#[post("/handle_queue")]
async fn handle_queue(q: web::Data<Mutex<ActionQueue>>) -> impl Responder {
   // execute_queue(q.lock().unwrap());
    thread::sleep(time::Duration::from_millis(50));
    HttpResponse::Ok()
}
#[actix_web::main] // or #[tokio::main]
pub async fn main() -> std::io::Result<()> {
    let action_queue = Arc::new(RwLock::new(web::Data::new(Mutex::new(ActionQueue {
        queue: Vec::new(),
    }))));
    let thread_queue = action_queue.clone();
    let server_queue = action_queue.clone();
    thread::spawn(move || {
        loop {
            execute_queue(thread_queue.read().unwrap().clone());
            thread::sleep(time::Duration::from_millis(50));
        }
    });
    HttpServer::new(move || {
        App::new()
            .app_data(server_queue.read().unwrap().clone())
            .wrap(middleware::Compress::default())
            .service(world_properties)
            .service(tiles)
            .service(entities)
            .service(post_queue)
            .service(handle_queue)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
