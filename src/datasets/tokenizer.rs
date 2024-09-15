use crate::datasets::token::Token;
use crate::util::tokens_util::*;
use std::collections::BTreeSet;

trait Tokenizer {
    fn encode(&self, item: &str) -> u32;
    fn decode(&self, index: u32) -> &str;
}

pub struct StandardTokenizer {
    pub tokens: BTreeSet<Token>,
}

impl StandardTokenizer {
    pub fn new() -> Self {
        let mut tokens: Vec<String> = vec![
            "<StartOfSong>",
            "<EndOfSong>",
            "<StartOfMeasure>",
            "<EndOfMeasure>",
        ]
        .iter()
        .map(|x| x.to_string())
        .collect();
        tokens.extend(channel_tags());
        tokens.extend(music_notes());
        tokens.extend(hex_numbers());
        tokens.extend(fx_letters());
        let mut tree = BTreeSet::new();
        let ext: BTreeSet<Token> = tokens
            .iter()
            .enumerate()
            .map(|(i, x)| Token::new(i as u32, x.to_string()))
            .collect();
        tree.extend(ext);
        Self { tokens: tree }
    }
}

impl Tokenizer for StandardTokenizer {
    fn encode(&self, item: &str) -> u32 {
        match self.tokens.iter().find(|x| x.val == item) {
            Some(t) => t.key,
            None => 0,
        }
    }
    fn decode(&self, index: u32) -> &str {
        match self.tokens.iter().find(|x| x.key == index) {
            Some(t) => t.val.as_str(),
            None => "",
        }
    }
}
