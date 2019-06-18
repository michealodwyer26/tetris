use std::io::{self, Write, Read};
use std::fs::File;

const HIGHSCORE_FILE: &'static str = "scores.txt";
const NUM_HIGHSCORES: usize = 5;

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

pub fn load_highscores_and_lines() -> Option<(Vec<u32>, Vec<u32>)> {
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

pub fn save_highscores_and_lines(highscores: &[u32], num_of_lines: &[u32]) -> bool {
    let s_highscores = slice_to_string(highscores);
    let s_num_of_lines = slice_to_string(num_of_lines);
    write_to_file(format!("{}\n{}\n", s_highscores, s_num_of_lines).as_str(), HIGHSCORE_FILE).is_ok()
}

pub fn update_vec(v: &mut Vec<u32>, value: u32) -> bool {
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