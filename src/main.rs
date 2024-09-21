pub mod components;
pub mod datasets;
pub mod util;
use components::song::Song;
use crate::components::parser;
use crate::datasets::exporters::{standard,compact};

/*
We can assume that songs with a Speed of 3 or less are in double time
*/

fn main() {
    println!("parsing songs and lining them up");
    let mut songs:Vec<Song> = parser::parse_txt(r"input\ze backuo (2).txt").iter().map(|x| x.line_up_patterns()).collect();

    println!("shrinking double timed songs");
    for song in songs.iter_mut(){
        if song.speed <= 3{
            song.shrink_all_channels(2, true);
        }
    }

    //for song in songs.iter(){
        //println!("avg dist: {}",song.notes_avg_dist());
    //}

    println!("exporting");
    standard::export(&songs, "output.json", "vocab.json");
    //compact::export(&songs, "output.json", "vocab.json");
}
