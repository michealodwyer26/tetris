extern crate sdl2;

use tetris::Tetris;
use sdl2::pixels::Color;
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::video::{Window, WindowContext};
use sdl2::rect::Rect;
use sdl2::image::LoadTexture;

pub fn load_asset<'a>(texture_creator: &'a TextureCreator<WindowContext>, file: &str) -> Texture<'a> {
    match texture_creator.load_texture(file) {
        Ok(texture) => texture, 
        Err(_) => panic!("Failed to load asset")
    }
}

pub fn create_texture_rect<'a>(canvas: &mut Canvas<Window>, texture_creator: &'a TextureCreator<WindowContext>, 
    r: u8, g: u8, b:u8, width: u32, height: u32) -> Option<Texture<'a>> {
    if let Ok(mut square_texture) = texture_creator.create_texture_target(None, width, height) {
        canvas.with_texture_canvas(&mut square_texture, |texture| {
            texture.set_draw_color(Color::RGB(r, g, b));
            texture.clear();
        }).expect("Failed to colour the texture.");
        Some(square_texture)
    } else  {
        None
    }
}

fn create_texture_from_text<'a>(texture_creator: &'a TextureCreator<WindowContext>, 
    font: &sdl2::ttf::Font, text: &str, r: u8, g:u8, b: u8) -> Option<Texture<'a>> {
        if let Ok(surface) = font.render(text).blended(Color::RGB(r, g, b)) {
            texture_creator.create_texture_from_surface(&surface).ok()
        } else {
            None
        }
}

fn get_rect_from_text(text: &str, x: i32, y:i32) -> Option<Rect> {
    Some(Rect::new(x, y, text.len() as u32 * 10, 30))
}

pub fn display_score<'a>(tetris: &Tetris, canvas: &mut Canvas<Window>, texture_creator: &'a TextureCreator<WindowContext>, 
    font: &sdl2::ttf::Font, x: i32) {
    let score_text = format!("Score: {}", tetris.score);
    let score = create_texture_from_text(&texture_creator, &font, &score_text, 255, 255, 255).expect("Cannot render text.");
    canvas.copy(&score, None, get_rect_from_text(&score_text, x, 0)).expect("Couldn't render text.");
}