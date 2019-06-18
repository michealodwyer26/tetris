extern crate sdl2;

pub mod tetrimino;
pub mod tetris;
pub mod highscore;
pub mod graphics;

use highscore::{load_highscores_and_lines, save_highscores_and_lines, update_vec};
use tetris::Tetris;
use graphics::{create_texture_rect, display_score, load_asset};

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::TextureCreator;
use sdl2::rect::Rect;
use sdl2::image::INIT_PNG;

use std::time::{Duration, SystemTime};
use std::thread::sleep;

const FPS: u32 = 60;
const WAIT_TIME: u32 = 1_000_000_000u32 / FPS;
const LEVEL_TIMES: [u32; 10] = [1000, 850, 700, 600, 500, 400, 300, 250, 221, 190];

const TETRIS_HEIGHT: usize = 40;
const GRID_ORIGIN_X: i32 = 0;
const GRID_ORIGIN_Y : i32 = 0;
const GRID_WIDTH: u32 = TETRIS_HEIGHT as u32 * 10;
const GRID_HEIGHT: u32 = TETRIS_HEIGHT as u32 * 16;
const WINDOW_WIDTH: u32 = GRID_WIDTH;
const WINDOW_HEIGHT: u32 = GRID_HEIGHT;

fn handle_events(tetris: &mut Tetris, quit: &mut bool, timer: &mut SystemTime, event_pump: &mut sdl2::EventPump, 
    paused: &mut bool) -> bool {
    let mut make_permanent = false;
    
    if !*paused {
        if let Some(ref mut piece) = tetris.current_piece {
            let mut tmp_x = piece.x;
            let mut tmp_y = piece.y;

            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit {..} | Event::KeyDown {keycode: Some(Keycode::Escape), ..} => {*quit = true; break},
                    Event::KeyDown{keycode: Some(Keycode::P), ..} => {*paused = !*paused; break},
                    Event::KeyDown{keycode: Some(Keycode::Down), ..} => {*timer = SystemTime::now(); tmp_y += 1},
                    Event::KeyDown{keycode: Some(Keycode::Right), ..} => tmp_x += 1,
                    Event::KeyDown{keycode: Some(Keycode::Left), ..} => tmp_x -= 1,
                    Event::KeyDown{keycode: Some(Keycode::Up), ..} => piece.rotate(&tetris.game_map),
                    Event::KeyDown{keycode: Some(Keycode::Space), ..} => {
                        let x = piece.x; 
                        let mut y = piece.y;
                        while piece.change_position(&tetris.game_map, x, y +1) {
                            y += 1;
                        }
                        make_permanent = true
                    },
                    _ => {}
                }
            }
            if !make_permanent {
                if !piece.change_position(&tetris.game_map, tmp_x, tmp_y) && tmp_y != piece.y {
                    make_permanent = true;
                }
            }
        }
        if make_permanent {
            tetris.make_permanent();
            *timer = SystemTime::now();
        }
        make_permanent
    }
    else {
        for event in event_pump.poll_iter() {
            match event {
                Event::KeyDown{keycode: Some(Keycode::P), ..} => {*paused = !*paused; break}
                _ => {}
            }
        }
        make_permanent
    }
}

fn print_game_info(tetris: &Tetris) {
    let mut new_highest_highscore = true;
    let mut new_highest_lines_sent = true;
    if let Some((mut highscores, mut lines_sent)) = load_highscores_and_lines() {
        new_highest_highscore = update_vec(&mut highscores, tetris.score);
        new_highest_lines_sent = update_vec(&mut lines_sent, tetris.num_lines);
        if new_highest_highscore || new_highest_lines_sent {
            save_highscores_and_lines(&highscores, &lines_sent);
        }
    } else {
        save_highscores_and_lines(&[tetris.score], &[tetris.num_lines]);
    }
    println!("Game over...");
    println!("Score:            {}{}", tetris.score, if new_highest_highscore {"[NEW HIGHSCORE]"} else {""});
    println!("Number of lines:  {}{}", tetris.num_lines, if new_highest_lines_sent {"[NEW HIGHSCORE]"} else {""});
    println!("Current level:    {}", tetris.current_level);
}

fn is_time_over(tetris: &Tetris, timer: &SystemTime) -> bool {
    match timer.elapsed() {
        Ok(elapsed) => {
            let millis = elapsed.as_secs() as u32 * 1000 + elapsed.subsec_nanos() / 1_000_000;
            millis > LEVEL_TIMES[tetris.current_level as usize - 1]
        }
        Err(_) => false
    }
}

pub fn main() {
    let sdl_context = sdl2::init().expect("SDL initialization failed.");
    let video_subsystem = sdl_context.video().expect("SDL video subsystem initialisation failed.");
    sdl2::image::init(INIT_PNG).expect("Failed to initialise the image context.");
    let ttf_context = sdl2::ttf::init().expect("Failed to initialise SDL ttf.");

    let mut tetris = Tetris::new();
    let mut timer = SystemTime::now();
    let mut event_pump = sdl_context.event_pump().expect("Failed to get SDL event pump");

    let window = video_subsystem.window("Tetris.rs", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered() 
        .build() 
        .expect("Failed to create window.");

    let mut canvas = window.into_canvas() 
        .target_texture() 
        .present_vsync() 
        .build() 
        .expect("Failed to create canvas.");

    let texture_creator: TextureCreator<_> = canvas.texture_creator();

    let grid = create_texture_rect(&mut canvas, &texture_creator, 0, 0, 0, GRID_WIDTH, GRID_HEIGHT)
        .expect("Failed to create texture.");

    let textures = [load_asset(&texture_creator, "assets/1.png"), load_asset(&texture_creator, "assets/2.png"), 
    load_asset(&texture_creator, "assets/3.png"), load_asset(&texture_creator, "assets/4.png"), 
    load_asset(&texture_creator, "assets/5.png"), load_asset(&texture_creator, "assets/6.png"), 
    load_asset(&texture_creator, "assets/7.png")];

    let mut font = ttf_context.load_font("assets/Inconsolata-Regular.ttf", 128).expect("Failed to load image.");
    font.set_style(sdl2::ttf::STYLE_BOLD);

    let mut paused = false;
    
    loop {
        if is_time_over(&tetris, &timer) && !paused {
            let mut make_permanent = false;
            if let Some(ref mut piece) = tetris.current_piece {
                let x = piece.x;
                let y = piece.y + 1;
                make_permanent = !piece.change_position(&tetris.game_map, x, y);
            }
            if make_permanent {
                tetris.make_permanent();
            }
            timer = SystemTime::now();
        }

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.copy(&grid, 
                None, 
                Rect::new(GRID_ORIGIN_X, GRID_ORIGIN_Y, GRID_WIDTH, GRID_HEIGHT))
                    .expect("Failed to copy texture to window.");

        if tetris.current_piece.is_none() {
            let current_piece = tetris.create_new_tetrimino();
            if !current_piece.test_current_position(&tetris.game_map) {
                print_game_info(&tetris);
                break
            }
            tetris.current_piece = Some(current_piece);
        }

        let mut quit = false;
        
        if !handle_events(&mut tetris, &mut quit, &mut timer, &mut event_pump, &mut paused) {
            if let Some(ref mut piece) = tetris.current_piece {
                for (line_num, line) in piece.states[piece.current_state as usize].iter().enumerate() {
                    for (case_num, case) in line.iter().enumerate() {
                        if *case == 0 {
                            continue
                        }
                    canvas.copy(&textures[*case as usize - 1],
                                None, 
                                Rect::new(GRID_ORIGIN_X + (piece.x + case_num as isize) as i32 * TETRIS_HEIGHT as i32, 
                                        GRID_ORIGIN_Y + (piece.y + line_num) as i32 * TETRIS_HEIGHT as i32, 
                                        TETRIS_HEIGHT as u32, TETRIS_HEIGHT as u32))
                        .expect("Failed to copy texture to window");
                    }
                }
            }
        }

        for (line_num, line) in tetris.game_map.iter().enumerate() {
            for (case_num, case) in line.iter().enumerate() {
                if *case == 0 {
                    continue 
                }
                canvas.copy(&textures[*case as usize - 1], 
                            None, 
                            Rect::new(GRID_ORIGIN_X + case_num as i32 * TETRIS_HEIGHT as i32, GRID_ORIGIN_Y + line_num as i32 * TETRIS_HEIGHT as i32,
                                    TETRIS_HEIGHT as u32, TETRIS_HEIGHT as u32))
                    .expect("Failed to copy texture to window.");
            }
        }

        display_score(&tetris, &mut canvas, &texture_creator, &font, WINDOW_WIDTH as i32 - 110);

        canvas.present();

        if quit {
            print_game_info(&tetris);
            break
        }

        sleep(Duration::new(0, WAIT_TIME));
    }
}