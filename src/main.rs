extern crate sdl2;
extern crate rand;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::video::{Window, WindowContext};
use sdl2::rect::Rect;
use sdl2::image::{LoadTexture, INIT_PNG};

use std::time::{Duration, SystemTime};
use std::thread::sleep;
use std::fs::File;
use std::io::{self, Write, Read};

const FPS: u32 = 60;
const WAIT_TIME: u32 = 1_000_000_000u32 / FPS;
// const TEXTURE_SIZE: u32 = 32;
const LEVEL_TIMES: [u32; 10] = [1000, 850, 700, 600, 500, 400, 300, 250, 221, 190];
const LEVEL_LINES: [u32; 10] = [20,   40,  60,  80,  100, 120, 140, 160, 180, 200];
const NUM_HIGHSCORES: usize = 5;
const HIGHSCORE_FILE: &'static str = "scores.txt";

const TETRIS_HEIGHT: usize = 40;
const ORIGIN_X: i32 = 0;
const ORIGIN_Y: i32 = 0;
const GRID_ORIGIN_X: i32 = 10;
const GRID_ORIGIN_Y : i32 = 10;
const GRID_WIDTH: u32 = TETRIS_HEIGHT as u32 * 10;
const GRID_HEIGHT: u32 = TETRIS_HEIGHT as u32 * 16;
const BORDER_SIZE: u32 = 10;
const BORDER_WIDTH: u32 = GRID_WIDTH + (BORDER_SIZE * 2);
const BORDER_HEIGHT: u32 = GRID_HEIGHT + (BORDER_SIZE * 2);
const WINDOW_WIDTH: u32 = BORDER_WIDTH + (BORDER_WIDTH / 2);
const WINDOW_HEIGHT: u32 = BORDER_HEIGHT;

type Piece = Vec<Vec<u8>>;
type States = Vec<Piece>;

struct Tetris {
    game_map: Vec<Vec<u8>>,
    current_level: u32,
    score: u32,
    num_lines: u32,
    current_piece: Option<Tetrimino>
}

impl Tetris {
    fn new() -> Tetris {
        let mut game_map = Vec::new();
        for _ in 0..16 {
            game_map.push(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        }
        Tetris {
            game_map: game_map,
            current_level: 1,
            score: 0,
            num_lines: 0,
            current_piece: None
        }
    }

    fn create_new_tetrimino(&self) -> Tetrimino {
        static mut PREV: u8 = 7;
        let mut rand_num = rand::random::<u8>() % 7;
        if unsafe {PREV} == rand_num {
            rand_num = rand::random::<u8>() % 7;
        }
        unsafe { PREV = rand_num; }
        match rand_num {
            0 => TetriminoI::new(),
            1 => TetriminoJ::new(),
            2 => TetriminoL::new(),
            3 => TetriminoO::new(),
            4 => TetriminoS::new(),
            5 => TetriminoZ::new(),
            6 => TetriminoT::new(),
            _ => unreachable!()
        }
    }

    fn check_lines(&mut self) {
        let mut y = 0;
        let mut score_add = 0;

        while y < self.game_map.len() {
            let mut complete = true;

            for x in &self.game_map[y] {
                if *x == 0 {
                    complete = false;
                    break
                }
            }
            if complete {
                score_add += self.current_level;
                self.game_map.remove(y);
                y -= 1;
            }
            y += 1;
        }
        if self.game_map.len() == 0 {
            score_add += 1000;
        }
        self.update_score(score_add);
        while self.game_map.len() < 16 {
            self.increase_line();
            self.game_map.insert(0, vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        }
    }

    fn make_permanent(&mut self) {
        let mut to_add = 0;
        if let Some(ref mut piece) = self.current_piece {
            let mut shift_y = 0;

            while shift_y < piece.states[piece.current_state as usize].len() && piece.y + shift_y < self.game_map.len() {
                let mut shift_x = 0;

                while shift_x < piece.states[piece.current_state as usize][shift_y].len() && (piece.x + shift_x as isize) < 
                    self.game_map[piece.y + shift_y].len() as isize {
                    if piece.states[piece.current_state as usize][shift_y][shift_x] != 0 {
                        let x = piece.x + shift_x as isize;
                        self.game_map[piece.y + shift_y][x as usize] = piece.states[piece.current_state as usize][shift_y][shift_x];
                    }
                    shift_x += 1;
                }
                shift_y += 1;
            }
            to_add += self.current_level;
        }
        self.update_score(to_add);
        self.check_lines();
        self.current_piece = None;
    }

    fn update_score(&mut self, to_add: u32) {
        self.score += to_add;
    }

    fn increase_line(&mut self) {
        self.num_lines += 1;
        if self.num_lines > LEVEL_LINES[self.current_level as usize - 1] {
            self.current_level += 1;
        }
    }
}

struct Tetrimino {
    states: States,
    x: isize,
    y: usize,
    current_state: u8
}

impl Tetrimino {
    fn rotate(&mut self, game_map: &[Vec<u8>]) {
        let mut tmp_state = self.current_state + 1;
        if tmp_state as usize >= self.states.len() {
            tmp_state = 0;
        }

        // tests if piece will fit if translated along the x-axis by up to 3 blocks in either direction
        let x_pos = [0, -1, 1, -2, 2, -3];
        for x in x_pos.iter() {
            if self.test_position(game_map, tmp_state as usize, self.x + x, self.y) {
                self.current_state = tmp_state;
                self.x += *x;
                break;
            }
        }
    }

    fn change_position(&mut self, game_map: &[Vec<u8>], new_x: isize, new_y: usize) -> bool {
        if self.test_position(game_map, self.current_state as usize, new_x, new_y) {
            self.x = new_x as isize;
            self.y = new_y;
            true
        } else {
            false
        }
    }

    fn test_position(&self, game_map: &[Vec<u8>], tmp_state: usize, x:isize, y:usize) -> bool {
        for decal_y in 0..4 {
            for decal_x in 0..4 {
                let x = x + decal_x;
                if self.states[tmp_state][decal_y][decal_x as usize] != 0 
                    && (y + decal_y >= game_map.len() || x < 0 ||  x as usize >= game_map[y + decal_y].len() ||
                        game_map[y + decal_y][x as usize] != 0) {
                    return false;
                }
            }
        }
        return true;
    }

    fn test_current_position(&self, game_map: &[Vec<u8>]) -> bool {
        self.test_position(game_map, self.current_state as usize, self.x, self.y)
    }
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
    if let Ok(content) = read_from_file(HIGHSCORE_FILE) {
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
    write_to_file(format!("{}\n{}\n", s_highscores, s_num_of_lines).as_str(), HIGHSCORE_FILE).is_ok()
}

fn update_vec(v: &mut Vec<u32>, value: u32) -> bool {
    if v.len() < NUM_HIGHSCORES {
        v.push(value);
        v.sort();
        true 
    } else {
        for entry in v.iter_mut() {
            if value > *entry {
                *entry = value;
                return true
            }
        }
        false
    }
}

fn create_texture_rect<'a>(canvas: &mut Canvas<Window>, texture_creator: &'a TextureCreator<WindowContext>, 
    r: u8, g: u8, b:u8, width: u32, height: u32) -> Option<Texture<'a>>{
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

fn handle_events(tetris: &mut Tetris, quit: &mut bool, timer: &mut SystemTime, event_pump: &mut sdl2::EventPump) -> bool {
    let mut make_permanent = false;
    if let Some(ref mut piece) = tetris.current_piece {
        let mut tmp_x = piece.x;
        let mut tmp_y = piece.y;

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown {keycode: Some(Keycode::Escape), ..} => {*quit = true; break},
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

    let border = create_texture_rect(&mut canvas, &texture_creator, 255, 255, 255, BORDER_WIDTH, BORDER_WIDTH)
        .expect("Failed to create texture.");

    // macro_rules! texture {
    //     ($r: expr, $g: expr, $b: expr) => (
    //         create_texture_rect(&mut canvas, &texture_creator, $r, $g, $b, TETRIS_HEIGHT as u32, TETRIS_HEIGHT as u32).unwrap()
    //     )
    // }

    // let textures = [texture!(255, 69, 69), texture!(255, 220, 69), texture!(237, 150, 37), texture!(171, 99, 237), texture!(77, 149, 239),
    //     texture!(39, 218, 225), texture!(45, 216, 47)];

    let textures = [texture_creator.load_texture("assets/I.png").expect("Couldn't load image."), 
        texture_creator.load_texture("assets/L.png").expect("Couldn't load image."), 
        texture_creator.load_texture("assets/R.png").expect("Couldn't load image."), 
        texture_creator.load_texture("assets/S.png").expect("Couldn't load image."), 
        texture_creator.load_texture("assets/T.png").expect("Couldn't load image."),
        texture_creator.load_texture("assets/S.png").expect("Couldn't load image."), 
        texture_creator.load_texture("assets/L.png").expect("Couldn't load image.")];
    
    loop {
        if is_time_over(&tetris, &timer) {
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
            canvas.copy(&border, 
                None, 
                Rect::new(ORIGIN_X, ORIGIN_Y, BORDER_WIDTH, BORDER_HEIGHT))
                    .expect("Failed to copy texture to window.");
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
        
        if !handle_events(&mut tetris, &mut quit, &mut timer, &mut event_pump) {
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

        canvas.present();

        if quit {
            print_game_info(&tetris);
            break
        }

        sleep(Duration::new(0, WAIT_TIME));
    }
}