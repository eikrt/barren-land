
use sdl2::image::{InitFlag, LoadSurface, LoadTexture};
use sdl2::keyboard::Keycode;
use sdl2::mixer::{InitFlag as AudioInitFlag, AUDIO_S16LSB, DEFAULT_CHANNELS};
use sdl2::mouse::MouseState;
use sdl2::mouse::MouseWheelDirection;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{BlendMode, Texture, TextureCreator, WindowCanvas};
use sdl2::surface::Surface;
use sdl2::ttf::Font;
use sdl2::video::FullscreenType;
use sdl2::Sdl;
use sdl2::event::Event;
use journey::entities::{Player};
use journey::world::{World, get_generated_world};
use journey::render::{render_sprite};
use std::{thread, time};
use std::time::{SystemTime, UNIX_EPOCH};

use std::collections::HashMap;
const SCREEN_WIDTH: u32 = 256;
const SCREEN_HEIGHT: u32 = 144;

fn main() {
    let mut sprites = HashMap::new();
    let mut running = true;
    let mut w = false;
    let mut a = false;
    let mut s = false;
    let mut d = false;
    let mut up = false;
    let mut down = false;
    let mut left = false;
    let mut right = false;

    let bg_color = Color::RGB(0, 0, 0);
    let sdl_context = sdl2::init().unwrap();

    let video_subsystem = sdl_context.video().unwrap();
    let mut window = video_subsystem
        .window("Journey", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .resizable()
        .build()
        .expect("could not initialize video subsystem");
    
    let mut canvas = window
        .into_canvas()
        .build()
        .expect("could not make a canvas");
    canvas.set_blend_mode(BlendMode::Blend);
    let texture_creator = canvas.texture_creator();
    let grass_texture = texture_creator.load_texture("res/grass_tile.png").unwrap();
    sprites.insert(
        "grass".to_string(),
        &grass_texture
    );
    let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG).unwrap();

    let mut compare_time = SystemTime::now();

    let mut event_pump = sdl_context.event_pump().unwrap();
    let world = get_generated_world();
    while running {


        let delta = SystemTime::now().duration_since(compare_time).unwrap();
        let time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
        compare_time = SystemTime::now();

        let delta_as_millis = delta.as_millis();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    running = false;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                }

                // WASD
                Event::KeyDown {
                    keycode: Some(Keycode::W),
                    ..
                } => {
                    w = true;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::A),
                    ..
                } => {
                    a = true;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::S),
                    ..
                } => {
                    s = true;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::D),
                    ..
                } => {
                    d = true;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => {
                    up = true;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => {
                    left = true;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => {
                    right = true;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    down = true;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Plus),
                    ..
                } => {
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Minus),
                    ..
                } => {
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Space),
                    ..
                } => {
                }
                Event::MouseWheel { x, y, .. } => {
                }
                Event::MouseMotion { .. } => {
                }
                Event::MouseButtonDown {
                    x, y, mouse_btn, ..
                } => {
                    if mouse_btn == sdl2::mouse::MouseButton::Middle {
                    }
                }
                // WASD
                Event::KeyUp {
                    keycode: Some(Keycode::W),
                    ..
                } => {
                    w = false;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::A),
                    ..
                } => {
                    a = false;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::S),
                    ..
                } => {
                    s = false;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::D),
                    ..
                } => {
                    d = false;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::Up),
                    ..
                } => {
                    up = false;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::Left),
                    ..
                } => {
                    left = false;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::Right),
                    ..
                } => {
                    right = false;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    down = false;
                }

                Event::KeyUp {
                    keycode: Some(Keycode::Plus),
                    ..
                } => {
                }
                Event::KeyUp {
                    keycode: Some(Keycode::Minus),
                    ..
                } => {
                }
                _ => {}
            }
        } 

        canvas.set_draw_color(bg_color);
        canvas.clear();
        
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

        canvas.present();
        thread::sleep(time::Duration::from_millis(1));
        }
}
