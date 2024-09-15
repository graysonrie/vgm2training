#[derive(Clone)]
pub struct Cell {
    pub row_n: u32,
    pub row_hex: String,
    pub note: Option<String>,
    pub note_octave: Option<char>,
    pub note_volume: Option<String>,
    pub instrument: Option<String>,
    pub fx: Vec<Option<String>>,
}

impl Cell {
    pub fn new() -> Self {
        return Self {
            row_n: 0,
            row_hex: String::from(""),
            note: None,
            note_octave: None,
            note_volume: None,
            instrument: None,
            fx: vec![],
        };
    }
}
