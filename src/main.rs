pub mod components;
pub mod util;
pub mod datasets;
use crate::components::parser;
use crate::datasets::creator;

fn main() {
    let songs = parser::parse_txt(r"input\ze backuo (2).txt");
    creator::export_standard(&songs, "output.json");
    //if let Some(song) = songs.get(0){
        //creator::standard(song);
    //}
    //for mut song in songs{
        //song.line_up_patterns();
    //}
}
