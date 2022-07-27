
use crate::entities::{Player};
use crate::world::{World, get_generated_world};
use std::{thread, time};
use std::time::{SystemTime, UNIX_EPOCH};


pub fn run() {
    let mut running = true;
    let mut w = false;
    let mut a = false;
    let mut s = false;
    let mut d = false;
    let mut up = false;
    let mut down = false;
    let mut left = false;
    let mut right = false;

    let mut compare_time = SystemTime::now();

    while running {


        let delta = SystemTime::now().duration_since(compare_time).unwrap();
        let time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
        compare_time = SystemTime::now();

        let delta_as_millis = delta.as_millis();

       /* 
        for i in 0..world.chunks.len() {
            for j in 0..world.chunks.len() {
                for k in 0..world.chunks[i][j].tiles.len() {
                    for h in 0..world.chunks[i][j].tiles.len() {
                        let tile = &world.chunks[i][j].tiles[k][h];
                               render_sprite(&mut canvas, &sprites[&tile.current_sprite], tile.x, tile.y, 12, 12, (canvas.window_mut().size().0 / SCREEN_WIDTH) as f32, (canvas.window_mut().size().1 / SCREEN_HEIGHT) as f32);                            
                    }
                }
            }

        }
*/
        thread::sleep(time::Duration::from_millis(1));
        }
}
