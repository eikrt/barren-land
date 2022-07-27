use journey::world;
fn main() {
    let seed = 64;
    let width = 8;
    let height = 8;
    let chunk_size = 12;
    let sealevel = 400.0;
    let name = "Land".to_string();
   // let world = world::get_generated_world(seed, width, height, chunk_size, sealevel, name);
    world::generate_world(seed, chunk_size,width, height, sealevel, name);
}
