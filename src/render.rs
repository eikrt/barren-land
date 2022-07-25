use sdl2::rect::{Rect};
use sdl2::render::Texture;
use sdl2::render::WindowCanvas;

pub fn render_sprite (
    canvas: &mut WindowCanvas,
    texture: &Texture,
    x: f32,
    y: f32,
    w: u32,
    h: u32,
    ratio_x: f32,
    ratio_y: f32,
) -> Result<(), String> {
    let (width, height) = canvas.output_size()?;
    let sprite = Rect::new(0,0,w,h);
    let zoom = 1.0;
    let screen_rect = Rect::new(
        (x / ratio_x) as i32,
        (y / ratio_y) as i32,
        (sprite.width() as f32 * ratio_x) as u32,
        (sprite.height() as f32 * ratio_y) as u32,
    );
    canvas.copy(texture, sprite, screen_rect)?;
    Ok(())
}
