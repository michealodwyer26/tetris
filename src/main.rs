extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::video::{Window, WindowContext};
use sdl2::rect::Rect;
use sdl2::image::{LoadTexture, INIT_PNG, INIT_JPG};

use std::time::Duration;
use std::thread::sleep;

const FPS: u32 = 60;
const WAIT_TIME: u32 = 1_000_000_000u32 / FPS;
const TEXTURE_SIZE: u32 = 32;

#[derive(Clone, Copy)]
enum TextureColour {
    Green, 
    Blue
}

fn create_texture_rect<'a>(canvas: &mut Canvas<Window>, texture_creator: &'a TextureCreator<WindowContext>, 
    colour: TextureColour, size: u32) -> Option<Texture<'a>>{
    if let Ok(mut square_texture) = texture_creator.create_texture_target(None, size, size) {
        canvas.with_texture_canvas(&mut square_texture, |texture| {
            match colour {
                TextureColour::Green => texture.set_draw_color(Color::RGB(0, 255, 0)),
                TextureColour::Blue => texture.set_draw_color(Color::RGB(0, 0, 255))
            }
            texture.clear();
        }).expect("Failed to colour the texture.");
        Some(square_texture)
    } else  {
        None
    }
}

pub fn main() {
    let sdl_context = sdl2::init().expect("SDL initialization failed.");
    let video_subsystem = sdl_context.video().expect("SDL video subsystem failed.");

    let window = video_subsystem.window("Tetris.rs", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .expect("Failed to create window");

    let mut canvas = window.into_canvas().build().expect("Failed to convert window to canvas.");
    sdl2::image::init(INIT_PNG | INIT_JPG).expect("Failed to initialise image context.");
    
    let texture_creator: TextureCreator<_> = canvas.texture_creator();
    let image_texture = texture_creator.load_texture("assets/img.png").expect("Failed to load image.");

    
    canvas.present();

    let mut event_pump = sdl_context.event_pump().expect("Failed to get SDL event pump.");

    let mut blue_square = create_texture_rect(&mut canvas, &texture_creator, TextureColour::Blue, TEXTURE_SIZE)
        .expect("Failed to create texture.");
    let mut green_square = create_texture_rect(&mut canvas, &texture_creator, TextureColour::Green, TEXTURE_SIZE)
        .expect("Failed to create texture.");

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => { break 'running },
                _ => {}
            }
        }
        canvas.set_draw_color(Color::RGB(255, 0, 0));
        canvas.clear();
        canvas.copy(&image_texture, None, None).expect("Failed to render image.");
        canvas.present();

        sleep(Duration::new(0, WAIT_TIME));
    }
}