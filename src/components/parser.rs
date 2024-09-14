use crate::components::{channel::Channel, song::Song};
use std::fs::File;
use std::io::Read;
use std::str::Lines;
use regex::Regex;

fn open_txt(path: &str) -> String {
    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(e) => panic!("Failed to open file: {}", e),
    };

    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => {
            return contents;
        }
        Err(e) => panic!("Failed to read file: {}", e),
    }
}

fn split_string_adv(input: &str) -> Vec<String> {
    // Create a regular expression that matches words or quoted phrases
    let re = Regex::new(r#""[^"]*"|\S+"#).unwrap();
    
    re.find_iter(input)
        .map(|mat| mat.as_str().trim_matches('"').to_string()) // Remove quotes from quoted phrases
        .collect()
}

fn find_line_numbers(input: &str, target: &str) -> Vec<usize> {
    let mut res: Vec<usize> = vec![];
    for (i, line) in input.lines().enumerate() {
        if line.contains(target) {
            res.push(i);
        }
    }
    res
}

fn find_line_numbers_lines(input: &Vec<String>, target: &str) -> Vec<usize> {
    let mut res: Vec<usize> = vec![];
    for (i, line) in input.iter().enumerate() {
        if line.contains(target) {
            res.push(i);
        }
    }
    res
}

pub fn parse_txt(path: &str) -> Vec<Song> {
    let mut res: Vec<Song> = vec![];

    let txt = open_txt(path);
    let txt = txt.as_str();
    let txt_lines: Vec<String> = txt.lines().map(|x| x.to_string()).collect();

    let track_starts = find_line_numbers_lines(&txt_lines, "TRACK");
    // each track start indicates a song
    for track_start in track_starts {
        let mut song = Song::new();

        if let Some(line) = txt_lines.get(track_start) {
            let line_split: Vec<String> = split_string_adv(line);
            //println!("{:#?}",line_split);
            if let Some(rows) = line_split.get(1) {
                song.rows = rows.parse::<u32>().unwrap_or_default();
            }
            if let Some(speed) = line_split.get(2) {
                song.speed = speed.parse::<u32>().unwrap_or_default();
            }
            if let Some(tempo) = line_split.get(3) {
                song.tempo = tempo.parse::<u32>().unwrap_or_default();
            }
            if let Some(name) = line_split.get(4) {
                song.name = name.to_string();
            }
        }

        res.push(song);
    }
    res
}
