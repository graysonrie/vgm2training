pub mod components;
pub mod datasets;
pub mod util;
use crate::components::parser;
use crate::datasets::exporters::{standard,compact};

fn main() {
    let songs = parser::parse_txt(r"input\ze backuo (2).txt");
    compact::export(&songs, "output.json", "vocab.json");
    //standard::export(&songs, "output.json", "vocab.json");

    //if let Some(song) = songs.get(0){
    //creator::standard(song);
    //}
    //for mut song in songs{
    //song.line_up_patterns();
    //}
}
