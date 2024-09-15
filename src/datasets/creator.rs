use crate::components::{parser, song::Song};
use crate::datasets::tokenizer::StandardTokenizer;

/*
Assumes the songs to be made with the VRC6 chip.
Change this in the future
*/

pub fn standard_many(songs: &[Song]) {
    for song in songs {
        standard(song);
    }
}

/// Standard dataset follows this format:
/// [<StartOfSong><StartOfMeasure>(A)(0) (HEX0)(HEXA) (HEXF) (P)(HEX7)(HEXF) (S)(HEX0)(HEX4) (.)(HEX.)(HEX.)]
pub fn standard(song: &Song) {
    // 2a03 is implied
    let max_n_fx = 3; // max number of effects in a single cell
    let chip = "VRC6";
    let tokenizer = StandardTokenizer::new();

    for cell in song.channel_iter(0) {
        //println!("{:#?}",cell.note)
        println!("{:#?}",tokenizer.tokenize_cell(&cell))
    }
}
