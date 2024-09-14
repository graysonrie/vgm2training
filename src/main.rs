pub mod components;
pub mod util;
pub mod datasets;
use crate::components::parser;

fn main() {
    let songs = parser::parse_txt(r"input\ze backuo (2).txt");
    for mut song in songs{
        song.line_up_patterns();
    }
}
