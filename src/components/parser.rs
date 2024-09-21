use crate::components::{cell::Cell, channel::Channel, order::Order, song::Song};
use crate::util::hex as hexutil;
use regex::Regex;
use std::fs::File;
use std::io::Read;
use rayon::prelude::*;

use super::{
    channel,
    order::{self, OrderEntry},
    pattern::Pattern,
};

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

fn split_string(input: &str) -> Vec<String> {
    let mut parts = Vec::new();
    let mut in_quotes = false;
    let mut current = String::new();

    for c in input.chars() {
        if c == '"' {
            in_quotes = !in_quotes;
        } else if c.is_whitespace() && !in_quotes {
            if !current.is_empty() {
                parts.push(current.clone());
                current.clear();
            }
        } else {
            current.push(c);
        }
    }
    if !current.is_empty() {
        parts.push(current);
    }
    parts
}

fn next_line_with(
    input_lines: &[&str],
    target: &str,
    start_index: Option<usize>,
) -> Option<usize> {
    let start = start_index.unwrap_or(0);
    input_lines[start..]
        .iter()
        .position(|line| line.contains(target))
        .map(|pos| pos + start)
}

fn next_line_is_blank(input_lines: &[&str], start_index: usize) -> bool {
    input_lines[start_index + 1].trim().is_empty()
}

fn find_line_numbers_lines(input: &[&str], target: &str) -> Vec<usize> {
    let mut res: Vec<usize> = vec![];
    for (i, line) in input.iter().enumerate() {
        if line.contains(target) {
            res.push(i);
        }
    }
    res
}

pub fn parse_txt(path: &str) -> Vec<Song> {

    let txt = open_txt(path);
    let txt = txt.as_str();
    let txt_lines: Vec<&str> = txt.lines().collect();

    let track_starts = find_line_numbers_lines(&txt_lines, "TRACK");
    // each track start indicates a song
    let songs:Vec<Song> = track_starts.par_iter().map(|&track_start| {
        let mut song = Song::new();

        if let Some(line) = txt_lines.get(track_start) {
            let line_split: Vec<String> = split_string(line);
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
        // parse the ORDER section
        let mut order_entries: Vec<OrderEntry> = Vec::new();
        if let Some(mut order_ind) = next_line_with(&txt_lines, "ORDER", Some(track_start)) {
            let mut ind: u32 = 0;

            while let Some(line) = txt_lines.get(order_ind) {
                if !line.contains("ORDER") {
                    break;
                }

                let mut order_entry = OrderEntry::new();
                let sep: Vec<&str> = line.split(':').collect();

                if let Some(first_part) = sep.get(0) {
                    if let Some(hex) = first_part.split_whitespace().nth(1) {
                        order_entry.hex = hex.to_string();
                    }
                }

                if let Some(second_part) = sep.get(1) {
                    order_entry.patternhexes =
                        second_part.split_whitespace().map(String::from).collect();
                }

                order_entry.number = ind;
                order_entries.push(order_entry);

                ind += 1;
                order_ind += 1;
            }
            song.order = Order::new(order_entries);

            // parse each PATTERN section
            let mut last_hex:u32 = 0;
            while let Some(mut pattern_ind) = next_line_with(&txt_lines, "PATTERN", Some(order_ind))
            {
                let mut pattern = Pattern::new();
                let mut channels: Vec<Channel> = Vec::new();
                if let Some(line) = txt_lines.get(pattern_ind) {
                    if let Some(hex) = line.split_whitespace().nth(1) {
                        pattern.hex = hex.to_string();
                        let hex_u32 = hexutil::to_u32(&pattern.hex.to_string());
                        if last_hex > hex_u32{
                            break;
                        }
                        last_hex = hex_u32;
                    }else{
                        panic!("Pattern should have associated HEX value, but there is none: '{}'", line);
                    }
                }
                while !next_line_is_blank(&txt_lines, pattern_ind) {
                    pattern_ind += 1;

                    if let Some(line) = txt_lines.get(pattern_ind) {
                        let mut row: Vec<Option<Cell>> = Channel::parse_row(line)
                            .into_iter()
                            .map(Some)
                            .collect();
                        // ensure list is populated
                        if channels.len() < 1 {
                            if song.num_channels == 0 {
                                song.num_channels = row.len();
                            }
                            for _ in 0..row.len() {
                                channels.push(Channel::new(0, pattern.hex.to_string(),vec![]));
                            }
                        }

                        for i in 0..channels.len() {
      
                            if let Some(cell) = row.get_mut(i).unwrap().take() {
                                match channels.get_mut(i) {
                                    Some(channel) => {
                                        channel.cells.push(cell);
                                    }
                                    None => { panic!("Channel should have {} channels but channel {} does not exist",song.num_channels,i)}
                                }
                            }
                        }
                    }
                }
                pattern.channels = channels;
                song.patterns.push(pattern);
                order_ind = pattern_ind;
                
            }  
        }
        song
    }).collect();
    songs
}
