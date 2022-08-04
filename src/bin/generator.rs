use journey::world;
fn main() {
    let seed = 4;
    let width = 16;
    let height = 16;
    let chunk_size = 4;
    let sealevel = 256.0;
    let name = "Land".to_string();
   // let world = world::get_generated_world(seed, width, height, chunk_size, sealevel, name);
    world::generate_world(seed, chunk_size,width, height, sealevel, name);
}
