use indexmap::IndexMap;
use crate::components::cell::Cell;
use crate::util::tokens_util::*;
use crate::util::vec_ext::VecExt;
use crate::datasets::tokenizer::*;

pub struct StandardTokenizer {
    pub tokens_decode: IndexMap<u32,String>,
    pub tokens_encode: IndexMap<String, u32>,
    max_num_fx: usize,
}

impl StandardTokenizer {
    pub fn new(max_num_fx: usize) -> Self {
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
        tokens.extend(misc_tokens());
        tokens.extend(music_notes().suffix(NOTE_SUFFIX));
        tokens.extend(oct_numbers().suffix(OCT_SUFFIX));
        tokens.extend(hex_numbers().suffix(VOL_SUFFIX));
        tokens.extend(hex_numbers().suffix(INST_SUFFIX));
        tokens.extend(fx_letters().suffix(FX_SUFFIX));
        let tokens_decode: IndexMap<u32,String> = tokens
            .iter()
            .enumerate()
            .map(|(i, x)| (i as u32, x.to_string()))
            .collect();
        let mut tokens_encode: IndexMap<String, u32> = IndexMap::new();
        for token in &tokens_decode{
            tokens_encode.insert(token.1.clone(),token.0.clone());
        }
        Self {
            tokens_decode,
            tokens_encode,
            max_num_fx,
        }
    }
    /// for example, passing in P7F would yield (PFX)(7HEX)(FHEX)
    pub fn tokenize_hex(&self, hex: &str) -> Vec<u32> {
        hex.chars().map(|x| self.encode(&x.to_string())).collect()
    }
    pub fn tokenize_cell(&self, cell: &Cell) -> Vec<u32> {
        let mut res: Vec<u32> = Vec::new();
        let blank_str = BLANK.to_string();
        // tokenize note
        res.push(
            self.encode(
                format!(
                    "{}{}",
                    cell.note.as_ref().unwrap_or(&blank_str),
                    NOTE_SUFFIX
                )
                .as_str(),
            ),
        );

        // tokenize instrument
        //println!("{:#?}",cell.instrument);
        let instrument_tokens: Vec<u32> = cell
            .instrument
            .as_deref()
            .unwrap_or("..")
            .chars()
            .map(|x| {
                let mut s = String::new();
                s.push(x);
                s.push_str(INST_SUFFIX);
                self.encode(&s)
            })
            .collect();

        res.extend(instrument_tokens);

        // tokenize octave
        res.push(
            self.encode(format!("{}{}", cell.note_octave.unwrap_or('.'), OCT_SUFFIX).as_str()),
        );

        // tokenize volume
        res.push(
            self.encode(
                format!(
                    "{}{}",
                    cell.note_volume.as_ref().unwrap_or(&blank_str),
                    VOL_SUFFIX
                )
                .as_str(),
            ),
        );

        // tokenize effects
        for fx in cell.fx.iter().take(self.max_num_fx) {
            //println!("{:#?}",fx);
            let fx_tokens: Vec<u32> = fx
                .as_deref()
                .unwrap_or("...")
                .chars()
                .map(|x| {
                    let mut s = String::new();
                    s.push(x);
                    s.push_str(FX_SUFFIX);
                    self.encode(&s)
                })
                .collect();

            res.extend(fx_tokens);
        }
        // fill the remaining context
        for _ in cell.fx.len()..self.max_num_fx {
            // 3 times since one empty effect is made of three '.' tokens
            let mut temp = String::new();
            temp.push('.');
            temp.push_str(FX_SUFFIX);
            res.extend(std::iter::repeat_with(|| self.encode(&temp)).take(3));
        }

        res
    }
}

impl Tokenizer for StandardTokenizer {
    fn encode(&self, item: &str) -> u32 {
        match self.tokens_encode.get(&item.to_string()) {
            Some(t) => t.clone(),
            None => panic!("token doesn't exist: '{}'", item),
        }
    }
    fn decode(&self, index: u32) -> &str {
        match self.tokens_decode.get(&index) {
            Some(t) => t.as_str(),
            None => "",
        }
    }
}
