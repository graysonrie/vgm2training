#[derive(Clone)]
pub struct Cell {
    pub row_n: u32,
    pub row_hex: String,
    pub note: String,
    pub note_octave: u32,
    pub note_volume: String,
    pub instrument: String,
    pub fx: Vec<String>,
}

impl Cell {
    pub fn new() -> Self {
        return Self {
            row_n: 0,
            row_hex: String::from(""),
            note: String::from(""),
            note_octave: 0,
            note_volume: String::from(""),
            instrument: String::from(""),
            fx: vec![],
        };
    }
}
