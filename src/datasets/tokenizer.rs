use crate::datasets::token::Token;
use std::collections::BTreeSet;

pub fn channel_names() -> Vec<String> {
    vec![
        "Pulse1",
        "Pulse2",
        "Triangle",
        "Noise",
        "VRC6Pulse1",
        "VRC6Pulse2",
        "Sawtooth",
    ]
    .iter()
    .map(|x| x.to_string())
    .collect()
}

pub fn channel_tags() -> Vec<String> {
    let mut res: Vec<String> = Vec::new();
    for channel in channel_names() {
        res.push(format!("<{}Start>", channel));
        res.push(format!("<{}End>", channel));
    }
    res
}

pub fn music_notes() -> Vec<String> {
    vec![
        "A", "A#", "B", "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#",
    ]
    .iter()
    .map(|x| x.to_string())
    .collect()
}

trait Tokenizer {
    fn tokenize(item: &str) -> u32;
}

struct StandardTokenizer {
    pub tokens: BTreeSet<Token>,
}

impl StandardTokenizer {
    pub fn new() -> Self {
        let mut tokens: Vec<String> = vec!["<StartOfMeasure>", "<EndOfMeasure>"]
            .iter()
            .map(|x| x.to_string())
            .collect();
        tokens.extend(channel_tags());
        tokens.extend(music_notes());
        Self {
            tokens: BTreeSet::new(),
        }
    }
}
