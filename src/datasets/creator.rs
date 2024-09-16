use super::tokenizer::Tokenizer;
use crate::components::song::Song;
use crate::datasets::{
    chipview::{self, TagOps},
    tokenizer::StandardTokenizer,
};
use crate::util::tokens_util::*;
use rayon::prelude::*;
use serde_json;
use std::fs::File;
use std::io::Write;

/*
Assumes the songs to be made with the VRC6 chip.
Change this in the future
*/

pub fn export_standard(songs: &[Song], file_path: &str) {
    let tokens: Vec<Vec<u32>> = songs
        .par_iter()
        .map(|x| {
            let t = standard(x);
            t
        })
        .collect();

    let json_data = serde_json::to_string(&tokens).unwrap();

    let mut file = File::create(file_path).expect("Unable to create file");
    file.write_all(json_data.as_bytes())
        .expect("Unable to write data");

    println!("Tokenized songs successfully saved to '{}'", file_path);
}

/// Standard dataset follows this format:
/// [<StartOfSong><StartOfMeasure>(A)(0) (HEX0)(HEXA) (HEXF) (P)(HEX7)(HEXF) (S)(HEX0)(HEX4) (.)(HEX.)(HEX.)]
pub fn standard(song: &Song) -> Vec<u32> {
    // 2a03 is implied
    let soundchip = "VRC6";
    let tokenizer = StandardTokenizer::new(3);

    let mut tokens: Vec<u32> = Vec::new();
    tokens.push(tokenizer.encode(SONG_START));
    for pattern in song.patterns.iter() {
        tokens.push(tokenizer.encode(PAT_START));

        for (i, channel) in pattern.channels.iter().enumerate() {
            let channel_name = chipview::channel_name_from_number(soundchip, i, false).unwrap();

            tokens.push(tokenizer.encode(channel_name.into_tag(true).as_str()));
            for cell in channel.cells.iter() {
                tokens.extend(tokenizer.tokenize_cell(cell));
            }
            tokens.push(tokenizer.encode(channel_name.into_tag(false).as_str()));
        }
        tokens.push(tokenizer.encode(PAT_END));
    }
    tokens.push(tokenizer.encode(SONG_END));
    tokens
}
