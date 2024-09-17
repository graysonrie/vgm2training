
// token suffixes
pub const NOTE_SUFFIX: &str = "NOTE";
pub const OCT_SUFFIX: &str = "OCT";
pub const INST_SUFFIX: &str = "INST";
pub const VOL_SUFFIX: &str = "VOL";
pub const FX_SUFFIX: &str = "FX";
pub const BLANK: &str = ".";

pub trait Tokenizer {
    fn encode(&self, item: &str) -> u32;
    fn decode(&self, index: u32) -> &str;
}