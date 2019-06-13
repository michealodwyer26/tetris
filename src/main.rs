extern crate sdl2;
extern crate rand;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::video::{Window, WindowContext};
use sdl2::rect::Rect;
use sdl2::image::{LoadTexture, INIT_PNG, INIT_JPG};

use std::time::Duration;
use std::thread::sleep;
use std::fs::File;
use std::io::{self, Write, Read};

const FPS: u32 = 60;
const WAIT_TIME: u32 = 1_000_000_000u32 / FPS;
const TEXTURE_SIZE: u32 = 32;

type Piece = Vec<Vec<u8>>;
type States = Vec<Piece>;

#[derive(Clone, Copy)]
enum TextureColour {
    Green, 
    Blue
}

struct Tetrimino {
    states: States,
    x: isize,
    y: isize,
    current_state: u8
}

trait TetriminoGenerator {
    fn new() -> Tetrimino;
}

struct TetriminoI;

impl TetriminoGenerator for TetriminoI {
    fn new() -> Tetrimino {
        Tetrimino {
            states: vec![vec![vec![1, 1, 1, 1],
                        vec![0, 0, 0, 0],
                        vec![0, 0, 0, 0],
                        vec![0, 0, 0, 0]],
                        vec![vec![0, 1, 0, 0],
                        vec![0, 1, 0, 0],
                        vec![0, 1, 0, 0],
                        vec![0, 1, 0, 0]]],
            x: 4,
            y: 0,
            current_state: 0
        }
    }
}

struct TetriminoJ;

impl TetriminoGenerator for TetriminoJ {
    fn new() -> Tetrimino {
        Tetrimino {
            states: vec![vec![vec![2, 2, 2, 0],
                            vec![2, 0, 0, 0],
                            vec![0, 0, 0, 0],
                            vec![0, 0, 0, 0]],
                    vec![vec![2, 2, 0, 0],
                        vec![0, 2, 0, 0],
                        vec![0, 2, 0, 0],
                        vec![0, 0, 0, 0]],
                    vec![vec![0, 0, 2, 0],
                        vec![2, 2, 2, 0],
                        vec![0, 0, 0, 0],
                        vec![0, 0, 0, 0]],
                    vec![vec![2, 0, 0, 0],
                        vec![2, 0, 0, 0],
                        vec![2, 2, 0, 0],
                        vec![0, 0, 0, 0]]],
            x: 4,
            y: 0,
            current_state: 0,
        }
    }
}

struct TetriminoL;

impl TetriminoGenerator for TetriminoL {
    fn new() -> Tetrimino {
        Tetrimino {
            states: vec![vec![vec![3, 3, 3, 0],
                            vec![0, 0, 3, 0],
                            vec![0, 0, 0, 0],
                            vec![0, 0, 0, 0]],
                        vec![vec![0, 3, 0, 0],
                            vec![0, 3, 0, 0],
                            vec![3, 3, 0, 0],
                            vec![0, 0, 0, 0]],
                        vec![vec![3, 0, 0, 0],
                            vec![3, 3, 3, 0],
                            vec![0, 0, 0, 0],
                            vec![0, 0, 0, 0]],
                        vec![vec![3, 3, 0, 0],
                            vec![3, 0, 0, 0],
                            vec![3, 0, 0, 0],
                            vec![0, 0, 0, 0]]],
            x: 4,
            y: 0,
            current_state: 0,
        }
    }
}
struct TetriminoO;

impl TetriminoGenerator for TetriminoO {

    fn new() -> Tetrimino {
        Tetrimino {
            states: vec![vec![vec![4, 4, 0, 0],
                            vec![4, 4, 0, 0],
                            vec![0, 0, 0, 0],
                            vec![0, 0, 0, 0]]],
            x: 5,
            y: 0,
            current_state: 0,
        }
    }
}

struct TetriminoS;

impl TetriminoGenerator for TetriminoS {
    fn new() -> Tetrimino {
        Tetrimino {
            states: vec![vec![vec![0, 5, 5, 0],
                            vec![5, 5, 0, 0],
                            vec![0, 0, 0, 0],
                            vec![0, 0, 0, 0]],
                        vec![vec![0, 5, 0, 0],
                            vec![0, 5, 5, 0],
                            vec![0, 0, 5, 0],
                            vec![0, 0, 0, 0]]],
            x: 4,
            y: 0,
            current_state: 0,
        }
    }
}

struct TetriminoZ;

impl TetriminoGenerator for TetriminoZ {
    fn new() -> Tetrimino {
        Tetrimino {
            states: vec![vec![vec![6, 6, 0, 0],
                            vec![0, 6, 6, 0],
                            vec![0, 0, 0, 0],
                            vec![0, 0, 0, 0]],
                        vec![vec![0, 0, 6, 0],
                            vec![0, 6, 6, 0],
                            vec![0, 6, 0, 0],
                            vec![0, 0, 0, 0]]],
            x: 4,
            y: 0,
            current_state: 0,
        }
    }
}

struct TetriminoT;

impl TetriminoGenerator for TetriminoT {
    fn new() -> Tetrimino {
        Tetrimino {
            states: vec![vec![vec![7, 7, 7, 0],
                            vec![0, 7, 0, 0],
                            vec![0, 0, 0, 0],
                            vec![0, 0, 0, 0]],
                        vec![vec![0, 7, 0, 0],
                            vec![7, 7, 0, 0],
                            vec![0, 7, 0, 0],
                            vec![0, 0, 0, 0]],
                        vec![vec![0, 7, 0, 0],
                            vec![7, 7, 7, 0],
                            vec![0, 0, 0, 0],
                            vec![0, 0, 0, 0]],
                        vec![vec![0, 7, 0, 0],
                            vec![0, 7, 7, 0],
                            vec![0, 7, 0, 0],
                            vec![0, 0, 0, 0]]],
            x: 4,
            y: 0,
            current_state: 0,
        }
    }
}

fn create_new_tetrimino() -> Tetrimino {
    let rand_num = 
}

fn write_to_file(content: &str, file_name: &str) -> io::Result<()> {
    let mut f = File::create(file_name)?;
    f.write_all(content.as_bytes())
}

fn read_from_file(file_name: &str) -> io::Result<String> {
    let mut f = File::open(file_name)?;
    let mut content = String::new();
    f.read_to_string(&mut content)?;
    Ok(content)
}

fn slice_to_string(slice: &[u32]) -> String {
    slice.iter().map(|highscore| highscore.to_string()).collect::<Vec<String>>().join(" ")
}

fn string_to_slice(line: &str) -> Vec<u32> {
    line.split(" ").filter_map(|num| num.parse::<u32>().ok()).collect()
}

fn load_highscores_and_lines() -> Option<(Vec<u32>, Vec<u32>)> {
    if let Ok(content) = read_from_file("scores.txt") {
        let mut lines = content.splitn(2, "\n").map(|line| string_to_slice(line)).collect::<Vec<_>>();
        if lines.len() == 2 {
            let (number_lines, highscores) = (lines.pop().unwrap(), lines.pop().unwrap());
            Some((highscores, number_lines))
        } else {
            None
        }
    } else {
        None
    }
}

fn save_highscores_and_lines(highscores: &[u32], num_of_lines: &[u32]) -> bool {
    let s_highscores = slice_to_string(highscores);
    let s_num_of_lines = slice_to_string(num_of_lines);
    write_to_file(format!("{}\n{}\n", s_highscores, s_num_of_lines).as_str(), "scores.txt").is_ok()
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