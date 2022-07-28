use actix_web::*;
use serde::{Deserialize, Serialize};
use crate::world::*;
use std::fs;
use bincode;


#[derive(Serialize, Deserialize)]
struct PostData {
    name: String,
}
#[derive(Serialize, Deserialize)]
struct ChunkGetData {
    x: i32,
    y: i32,
}
pub fn open_chunk(x: i32, y: i32) -> String {
    let path = format!("world/chunks/chunk_{}_{}",x,y);
    let body = fs::read(path).unwrap();
    let decoded: Chunk = bincode::deserialize(&body).unwrap();
    let encoded = serde_json::to_string(&decoded).unwrap();
    return encoded; 
}
#[get("/chunks/{x}/{y}")]
async fn chunks(data: web::Path<ChunkGetData>) -> impl Responder {
    let contents = open_chunk(data.x,data.y);
    HttpResponse::Ok()
        .body(contents)
}
#[get("/index")]
async fn index(_req: HttpRequest) -> impl Responder {
    web::Bytes::from_static(b"Hello world!")
}
#[actix_web::main] // or #[tokio::main]
pub async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Compress::default())
            .service(chunks)
            .service(index)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
