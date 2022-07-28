use journey::world;
fn main() {
    let seed = 64;
    let width = 2;
    let height = 2;
    let chunk_size = 8;
    let sealevel = 256.0;
    let name = "Land".to_string();
   // let world = world::get_generated_world(seed, width, height, chunk_size, sealevel, name);
    world::generate_world(seed, chunk_size,width, height, sealevel, name);
}
