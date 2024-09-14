pub mod components;
use crate::components::parser;
use std::fs::File;
use std::io::Read;

fn main() {
    let songs = parser::parse_txt(r"input\ze backuo (2).txt");
    for song in songs{
        println!("{}",song.name);
    }
}
