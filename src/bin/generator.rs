use journey::world;
fn main() {
    let seed = 4;
    let width = 4;
    let height = 4;
    let chunk_size = 12;
    let sealevel = 102.0;
    let name = "The Barren Lands".to_string();
    // let world = world::get_generated_world(seed, width, height, chunk_size, sealevel, name);
    world::generate_world(seed, chunk_size, width, height, sealevel, name);
}
