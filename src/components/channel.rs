use crate::components::cell::Cell;

#[derive(Clone)]
pub struct Channel {
    pub index: usize,
    pub pattern_hex: String,
    pub cells: Vec<Cell>,
}

impl Channel {
    pub fn new(index: usize, pattern_hex: String, cells: Vec<Cell>) -> Self {
        return Self {
            index,
            pattern_hex,
            cells,
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
    ///good for putting a song in double time back to normal time. Irreverible action
    ///
    /// Example:
    /// passing in `2` for `step_size` has the same effect as shrinking the pattern in tracker software
    pub fn shrink(&mut self, step_size: usize) {
        let mut count = 0;
        self.cells.retain(|_| {
            let keep = count % step_size == 0;
            count += 1;
            keep
        });
    }
    /// step_size must be a number greater than 1. Returns true if successful
    pub fn expand(&mut self, step_size: usize) -> bool {
        if step_size < 2 {
            return false;
        }
        for i in 0..self.cells.len() {
            if i % 2 == 1 {
                for _ in 0..step_size - 1 {
                    self.cells.insert(i, Cell::new());
                }
            }
        }
        true
    }
    /// adds the given channel onto the end of this one
    pub fn append(&mut self, channel: &mut Channel) {
        self.cells.append(&mut channel.cells);
    }
    /// splits this channel in two and returns the extra channel
    pub fn halve(&mut self) -> Channel {
        let mid = self.cells.len() / 2;
        let cells = self.cells.split_off(mid);
        Channel::new(self.index + 1, self.pattern_hex.clone(), cells)
    }
    /// returns the average distance between notes for this channel
    pub fn notes_avg_dist(&self) -> u32 {
        let mut dists: Vec<u32> = Vec::new();
        let mut d: u32 = 0;
        for cell in self.cells.iter() {
            if (cell.note_octave.is_some()){
                dists.push(d);
                d = 0;
            }
            d += 1;
        }
        if dists.len() == 0{ // avoid division by 0
            return self.cells.len() as u32;
        }
        dists.iter().sum::<u32>()/(dists.len() as u32)
    }
}
