use crate::components::{parser, song::Song};
use std::collections::BTreeSet;

/*
Assumes the songs to be made with the VRC6 chip.
Change this in the future
*/

pub fn standard_many(songs: &[Song]) {
    for song in songs {
        standard(song);
    }
}

pub fn standard(song: &Song) {
    // 2a03 is implied
    let chip = "VRC6";
    let mut tokens: BTreeSet<&str> = BTreeSet::new();

    for cell in song.channel_iter(0) {
        println!("{}", cell.note);
    }
}
