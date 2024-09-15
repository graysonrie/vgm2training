use crate::components::cell::Cell;
pub struct Channel {
    pub pattern_hex: String,
    pub cells: Vec<Cell>,
}

impl Channel {
    pub fn new(pattern_hex: String) -> Self {
        return Self {
            pattern_hex,
            cells: vec![],
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
                let cellnote = String::from(&note[0..2]);
                // ensure that it also isn't a note break
                if cellnote.ends_with("-") && cellnote != "--" {
                    cell.note = Some(cellnote[0..1].to_string());
                } else if cellnote == ".." {
                    cell.note = None;
                } else {
                    cell.note = Some(cellnote);
                }
                let oct_val = note.chars().nth(2);
                if let Some(oct) = oct_val {
                    if oct == '.' || oct == '-' || oct == '=' {
                        // don't count note breaks, note releases, or lacks of notes
                    } else {
                        cell.note_octave = oct_val;
                    }
                }
                //println!("{:#?}", note);
            }
            if let Some(instr) = split_space.get(1) {
                if instr == &"." {
                    cell.instrument = None;
                } else {
                    cell.instrument = Some(instr.to_string());
                }
            }
            if let Some(vol) = split_space.get(2) {
                if vol == &"." {
                    cell.note_volume = None;
                } else {
                    cell.note_volume = Some(vol.to_string());
                }
            }
            for remainder in &split_space[3..] {
                //println!("{}",remainder);
                if remainder == &"..." {
                    cell.fx.push(None);
                } else {
                    cell.fx.push(Some(remainder.to_string()));
                }
            }
            res.push(cell);
        }
        res
    }
}
