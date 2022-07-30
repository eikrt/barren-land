use actix_web::*;
use serde::{Deserialize, Serialize};
use crate::world::*;
use std::fs;
use bincode;


#[derive(Serialize, Deserialize)]
struct PostData {
    command: String,
}
#[derive(Serialize, Deserialize)]
struct ChunkGetData {
    x: i32,
    y: i32,
}
fn queue_to_object(data: web::Json<PostData>) -> PostData {
    PostData{
        command: data.command.clone()
    }
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
pub fn open_world_properties() -> String {
    let path = "world/world_properties.dat";
    let body = fs::read(path).unwrap();
    let decoded: WorldProperties = bincode::deserialize(&body).unwrap();
    let encoded = serde_json::to_string(&decoded).unwrap();
    return encoded; 
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
async fn post_queue(post: web::Json<PostData>) -> impl Responder {
    let contents = queue_to_object(post);
    HttpResponse::Ok()
}
#[actix_web::main] // or #[tokio::main]
pub async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Compress::default())
            .service(world_properties)
            .service(tiles)
            .service(entities)
            .service(post_queue)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
