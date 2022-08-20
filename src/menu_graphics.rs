use sdl2::image::{InitFlag, LoadTexture};
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{BlendMode, Texture, TextureCreator, WindowCanvas};
use sdl2::surface::Surface;
use sdl2::ttf::Font;
#[derive(PartialEq, Clone, Debug)]
pub enum ButtonStatus {
    Neutral,
    Hovered,
    Pressed,
    Released,
}
pub struct Button {
    pub status: ButtonStatus,
    pub previous_status: ButtonStatus,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}
impl Button {
    pub fn hover(&mut self) {
        self.status = ButtonStatus::Hovered;
    }
    pub fn press(&mut self) {
        self.status = ButtonStatus::Pressed;
    }
    pub fn release(&mut self) {
        self.status = ButtonStatus::Released;
    }
    pub fn neutralize(&mut self) {
        self.status = ButtonStatus::Neutral;
    }
    pub fn check_if_hovered(&mut self, m_x: f32, m_y: f32, ratio_x: f32, ratio_y: f32) {
        let m_x2 = m_x;
        let m_y2 = m_y;
        if m_x2 > self.x as f32
            && m_x2 < self.x + self.width
            && m_y2 > self.y
            && m_y2 < self.y + self.height
        {
            self.hover();
        } else {
            self.neutralize();
        }
    }
    pub fn check_if_pressed(&mut self, _m_x: f32, _m_y: f32, pressed: bool) {
        if pressed && self.status == ButtonStatus::Hovered {
            self.press();
        } else if !pressed && self.previous_status == ButtonStatus::Pressed {
            self.release();
        }
        self.previous_status = self.status.clone();
    }
}
pub struct Text<'a> {
    pub text_surface: Surface<'a>,
    pub text_texture: Texture<'a>,
    pub text_sprite: Rect,
}
pub fn render(
    canvas: &mut WindowCanvas,
    texture: &Texture,
    position: Point,
    sprite: Rect,
    zoom: f32,
    ratio_x: f32,
    ratio_y: f32,
) -> Result<(), String> {
    let (width, height) = canvas.output_size()?;
    let screen_rect = Rect::new(
        (position.x as f32 / ratio_x) as i32,
        (position.y as f32 / ratio_y) as i32,
        (sprite.width() as f32 * zoom / ratio_x) as u32,
        (sprite.height() as f32 * zoom / ratio_y) as u32,
    );
    canvas.copy(texture, sprite, screen_rect)?;
    Ok(())
}
pub fn render_text(
    canvas: &mut WindowCanvas,
    texture: &Texture,
    position: Point,
    sprite: Rect,
    ratio_x: f32,
    ratio_y: f32,
) {
    let screen_rect = Rect::new(
        (position.x as f32 / ratio_x) as i32,
        (position.y as f32 / ratio_y) as i32,
        (sprite.width() as f32 / ratio_x) as u32,
        (sprite.height() as f32 / ratio_y) as u32,
    );
    canvas.copy(texture, None, screen_rect);
}
pub fn get_text<'a, T>(
    msg: String,
    color: Color,
    font_size: u16,
    font: &Font,
    texture_creator: &'a TextureCreator<T>,
) -> Option<Text<'a>> {
    let text_surface = font
        .render(&msg)
        .blended(color)
        .map_err(|e| e.to_string())
        .ok()?;
    let text_texture = texture_creator
        .create_texture_from_surface(&text_surface)
        .map_err(|e| e.to_string())
        .ok()?;
    let text_sprite = Rect::new(
        0,
        0,
        (font_size as f32 / 2.0) as u32 * msg.len() as u32,
        (font_size as f32) as u32,
    );

    let text = Text {
        text_surface: text_surface,
        text_texture: text_texture,
        text_sprite: text_sprite,
    };
    return Some(text);
}
