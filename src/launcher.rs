use crate::menu_graphics::*;
use crate::*;
use sdl2::event::Event;
use sdl2::image::{InitFlag, LoadSurface, LoadTexture};
use sdl2::keyboard::Keycode;
use sdl2::mixer::{InitFlag as AudioInitFlag, AUDIO_S16LSB, DEFAULT_CHANNELS};
use sdl2::mouse::MouseState;
use sdl2::mouse::MouseWheelDirection;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{BlendMode, Texture, TextureCreator, WindowCanvas};
use sdl2::surface::Surface;
use std::env;
use std::process::Command;
use std::{thread, time};

const SCREEN_WIDTH: u32 = 500;
const SCREEN_HEIGHT: u32 = 200;
const REFRESH_TIME: u64 = 10;
pub fn main() -> Result<(), String> {
    let mut running = true;
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let mut window = video_subsystem
        .window("Tales of Terrant", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .resizable()
        .build()
        .expect("could not initialize video subsystem");
    let mut canvas = window
        .into_canvas()
        .build()
        .expect("could not make a canvas");
    canvas.set_blend_mode(BlendMode::Blend);
    let sprite_128x32 = Rect::new(0, 0, (128.0) as u32, (32.0) as u32);
    let texture_creator = canvas.texture_creator();
    let mut menu_button_texture = texture_creator.load_texture("res/menu/menu_button.png")?;
    let mut menu_button_hovered_texture =
        texture_creator.load_texture("res/menu/menu_button_hovered.png")?;
    let mut menu_button_pressed_texture =
        texture_creator.load_texture("res/menu/menu_button_pressed.png")?;
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
    let bg_color = Color::RGB(0, 0, 0);
    let mut event_pump = sdl_context.event_pump()?;
    let main_title_font_size = 17;
    let button_font_size = 16;
    let mut main_title_font =
        ttf_context.load_font("fonts/PixelOperator.ttf", main_title_font_size)?;
    let mut button_font = ttf_context.load_font("fonts/PixelOperator.ttf", button_font_size)?;

    let mut menu_buttons: Vec<Button> = vec![
        // menu buttons
        Button {
            status: ButtonStatus::Hovered, // play button
            previous_status: ButtonStatus::Hovered,
            x: 35 as f32,
            y: (SCREEN_HEIGHT / 2 - 16) as f32,
            width: 128.0,
            height: 32.0,
        },
        Button {
            status: ButtonStatus::Hovered, // settings button
            previous_status: ButtonStatus::Hovered,
            x: 175 as f32,
            y: (SCREEN_HEIGHT / 2 - 16) as f32,
            width: 128.0,
            height: 32.0,
        },
        Button {
            status: ButtonStatus::Hovered, //  manual button
            previous_status: ButtonStatus::Hovered,
            x: 315 as f32,
            y: (SCREEN_HEIGHT / 2 - 16) as f32,
            width: 128.0,
            height: 32.0,
        },
    ];
    let mut mouse_state = MouseState::new(&event_pump);
    let mut mx = 0;
    let mut my = 0;
    let mut mouse_left = false;
    while running {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    running = false;
                }
                Event::MouseMotion { x, y, .. } => {
                    mx = x;
                    my = y;
                }
                Event::MouseButtonDown {
                    x, y, mouse_btn, ..
                } => {
                    if mouse_btn == sdl2::mouse::MouseButton::Left {
                        mouse_left = true;
                    }
                }
                Event::MouseButtonUp {
                    x, y, mouse_btn, ..
                } => {
                    if mouse_btn == sdl2::mouse::MouseButton::Left {
                        mouse_left = false;
                    }
                }
                _ => {}
            }
        }
        canvas.set_draw_color(bg_color);
        canvas.clear();
        for button in menu_buttons.iter_mut() {
            let position = Point::new(button.x as i32, button.y as i32);
            button.check_if_hovered(mx as f32, my as f32, 1.0, 1.0);
            button.check_if_pressed(mx as f32, my as f32, mouse_left);
        }

        if menu_buttons[0].status == ButtonStatus::Pressed {
            if env::var("MODE").unwrap() == "DEV" {
                thread::spawn(|| {
                    let output = Command::new("cargo")
                        .args(["run", "--bin", "server"])
                        .output()
                        .expect("failed to execute process");
                });
                menu_buttons[0].status = ButtonStatus::Released;
            } else {
            }
        }
        if menu_buttons[1].status == ButtonStatus::Pressed {
            if env::var("MODE").unwrap() == "DEV" {
                thread::spawn(|| {
                    let output = Command::new("cargo")
                        .args(["run", "--bin", "server"])
                        .output()
                        .expect("failed to execute process");
                });
                menu_buttons[0].status = ButtonStatus::Released;
            } else {
            }
        }
        if menu_buttons[2].status == ButtonStatus::Pressed {
            std::process::exit(0);
        }
        for button in menu_buttons.iter_mut() {
            let position = Point::new(button.x as i32, button.y as i32);
            if button.status == ButtonStatus::Hovered {
                render(
                    &mut canvas,
                    &menu_button_hovered_texture,
                    position,
                    sprite_128x32,
                    1.0,
                    1.0,
                    1.0,
                );
            } else if button.status == ButtonStatus::Pressed {
                render(
                    &mut canvas,
                    &menu_button_pressed_texture,
                    position,
                    sprite_128x32,
                    1.0,
                    1.0,
                    1.0,
                );
            } else {
                render(
                    &mut canvas,
                    &menu_button_texture,
                    position,
                    sprite_128x32,
                    1.0,
                    1.0,
                    1.0,
                );
            }
        }
        let title_text = get_text(
            "Barren Lands Launcher".to_string(),
            Color::RGBA(255, 255, 255, 255),
            main_title_font_size,
            &main_title_font,
            &texture_creator,
        )
        .unwrap();
        let position = Point::new(0, 0);
        let text_margin = 4;
        render_text(
            &mut canvas,
            &title_text.text_texture,
            position,
            title_text.text_sprite,
            1.0,
            1.0,
        );
        let server_text = get_text(
            "Start Server".to_string(),
            Color::RGBA(255, 255, 255, 255),
            button_font_size,
            &button_font,
            &texture_creator,
        )
        .unwrap();
        let position = Point::new(menu_buttons[0].x as i32 + 4, menu_buttons[0].y as i32 + 4);
        let text_margin = 4;
        render_text(
            &mut canvas,
            &server_text.text_texture,
            position,
            server_text.text_sprite,
            1.0,
            1.0,
        );
        let client_text = get_text(
            "Start Client".to_string(),
            Color::RGBA(255, 255, 255, 255),
            button_font_size,
            &button_font,
            &texture_creator,
        )
        .unwrap();
        let position = Point::new(menu_buttons[1].x as i32 + 4, menu_buttons[1].y as i32 + 4);

        render_text(
            &mut canvas,
            &client_text.text_texture,
            position,
            client_text.text_sprite,
            1.0,
            1.0,
        );
        let position = Point::new(menu_buttons[2].x as i32 + 4, menu_buttons[2].y as i32 + 4);
        let exit_text = get_text(
            "Exit".to_string(),
            Color::RGBA(255, 255, 255, 255),
            button_font_size,
            &button_font,
            &texture_creator,
        )
        .unwrap();
        render_text(
            &mut canvas,
            &exit_text.text_texture,
            position,
            exit_text.text_sprite,
            1.0,
            1.0,
        );

        canvas.present();
        thread::sleep(time::Duration::from_millis(REFRESH_TIME));
    }
    Ok(())
}
