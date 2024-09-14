use crate::components::cell::Cell;
pub struct Channel {
    pub pattern_hex: String,
    pub cells: Vec<Cell>
}

impl Channel {
    pub fn new(pattern_hex:String) -> Self {
        return Self {
            pattern_hex,
            cells: vec![]
        };
    }
    pub fn parse_row(row: &str) -> Vec<Cell> {
        let mut res: Vec<Cell> = vec![];

        let mut row_hex = "";
        let cell_split: Vec<&str> = row.split(':').collect();

        if let Some(cell_hex) = cell_split.get(0) {
            let split_space: Vec<&str> = cell_hex.split_whitespace().collect();
            if let Some(hex) = split_space.get(1) {
                row_hex = hex;
            }
        }

        for c in &cell_split[1..] {
            let mut cell = Cell::new();
            cell.row_hex = row_hex.to_string();

            let split_space: Vec<&str> = c.split_whitespace().collect();
            if let Some(note) = split_space.get(0) {
                cell.note = String::from(&note[0..2]);
                if cell.note.ends_with("-") {
                    cell.note = cell.note[0..1].to_string();
                }
                if cell.note == ".." {
                    cell.note = String::from("");
                }
                cell.note_octave = note.chars().nth(2).unwrap().to_digit(10).unwrap_or(0);
            }
            if let Some(instr) = split_space.get(1) {
                if instr == &"." {
                    cell.instrument = "prev".to_string();
                } else {
                    cell.instrument = instr.to_string();
                }
            }
            if let Some(vol) = split_space.get(2) {
                if vol == &"." {
                    cell.note_volume = "prev".to_string();
                } else {
                    cell.note_volume = vol.to_string();
                }
            }
            for remainder in &cell_split[3..] {
                cell.fx.push(remainder.to_string());
            }
            res.push(cell);
        }
        res
    }
}
