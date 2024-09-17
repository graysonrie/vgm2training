use crate::components::cell::Cell;
use crate::datasets::tokenizer::*;
use crate::util::tokens_util::{self, *};
use crate::util::vec_ext::VecExt;
use indexmap::IndexMap;

pub struct CompactTokenizer {
    pub tokens_decode: IndexMap<u32, String>,
    pub tokens_encode: IndexMap<String, u32>,
    max_num_fx: usize,
    limit_octaves: bool,
}

impl CompactTokenizer {
    pub fn new(max_num_fx: usize, limit_octaves: bool) -> Self {
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
        tokens.extend(CompactTokenizer::gen_all_possible(
            max_num_fx,
            limit_octaves,
        ));
        let tokens_decode: IndexMap<u32, String> = tokens
            .iter()
            .enumerate()
            .map(|(i, x)| (i as u32, x.to_string()))
            .collect();
        let mut tokens_encode: IndexMap<String, u32> = IndexMap::new();
        for token in &tokens_decode {
            tokens_encode.insert(token.1.clone(), token.0.clone());
        }
        Self {
            tokens_decode,
            tokens_encode,
            max_num_fx,
            limit_octaves,
        }
    }
    /// generate all possible cell combinations. TODO: add support for fx category
    fn gen_all_possible(max_num_fx: usize, limit_octaves: bool) -> Vec<String> {
        let mut res: Vec<String> = Vec::new();
        let mut cell = Cell::new();
        for note in tokens_util::music_notes() {
            for octave in tokens_util::oct_numbers_char() {
                for vol_hex in tokens_util::hex_numbers_char() {
                    for instr_hex_1 in tokens_util::starting_inst_numbers() {
                        for instr_hex_2 in tokens_util::hex_numbers_char() {
                            let is_valid_octave =
                                !(octave == '0' || octave == '7' || octave == '6' && limit_octaves);
                            let is_valid_instr = (instr_hex_1 == '.' && instr_hex_2 == '.')
                                || (instr_hex_1 != '.' && instr_hex_2 != '.');
                            let is_valid_note = !(CompactTokenizer::is_u32(&note) && octave != '#');
                            if is_valid_instr && is_valid_octave && is_valid_note {
                                cell.note = Some(note.clone());
                                cell.note_octave = Some(octave);
                                cell.note_volume = Some(vol_hex.to_string());
                                cell.instrument = Some(format!("{}{}", instr_hex_1, instr_hex_2));
                                res.push(CompactTokenizer::cell_as_tag_static(
                                    &cell,
                                    max_num_fx,
                                    limit_octaves,
                                ))
                            }
                        }
                    }
                }
            }
        }
        res
    }
    pub fn cell_as_tag(&self, cell: &Cell) -> String {
        CompactTokenizer::cell_as_tag_static(cell, self.max_num_fx, self.limit_octaves)
    }
    fn cell_as_tag_static(cell: &Cell, max_num_fx: usize, limit_octaves: bool) -> String {
        let mut sb: String = String::new();
        let blank_char = '.';
        let blank = String::from(".");
        let double_blank = String::from("..");
        sb.push('<');
        sb.push_str(cell.note.as_ref().unwrap_or(&blank).as_str());

        if let Some(octave) = cell.note_octave {
            if limit_octaves {
                sb.push(CompactTokenizer::clamp_octave(octave));
            } else {
                sb.push(octave);
            }
        } else {
            sb.push(blank_char);
        }

        sb.push_str(cell.instrument.as_ref().unwrap_or(&double_blank).as_str());
        sb.push_str(cell.note_volume.as_ref().unwrap_or(&blank).as_str());
        for fx in cell.fx.iter().take(max_num_fx) {
            sb.push_str(fx.as_ref().unwrap_or(&blank).as_str());
        }
        sb.push('>');
        sb
    }
    fn clamp_octave(octave: char) -> char {
        match octave {
            '0' => '1',
            '6' | '7' => '5',
            _ => octave,
        }
    }
    fn is_u32(string: &String) -> bool {
        match string.parse::<u32>() {
            Ok(_) => true,
            Err(_) => false,
        }
    }
}

impl Tokenizer for CompactTokenizer {
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
