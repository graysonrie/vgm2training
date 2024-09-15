use crate::components::order::Order;

use super::{cell::Cell, pattern::Pattern};

pub struct Song {
    pub name: String,
    pub rows: u32,
    pub speed: u32,
    pub tempo: u32,
    pub order: Order,
    pub patterns: Vec<Pattern>,
    pub num_channels: usize,
}

impl Song {
    pub fn new() -> Self {
        Self {
            name: "".to_string(),
            rows: 0,
            speed: 0,
            tempo: 0,
            order: Order::new(vec![]),
            patterns: vec![],
            num_channels: 0,
        }
    }
    /// adjusts the channels in the patterns based off the orders
    pub fn line_up_patterns(&mut self) {
        println!("lining up patterns");
    }
    pub fn total_rows(&self) -> usize {
        let mut i: usize = 0;
        for pattern in self.patterns.iter() {
            i += pattern.get_len()
        }
        i
    }
    /// iterate over an entire channel, combining all patterns in the song
    pub fn channel_iter(&self, channel_n: usize) -> ChannelIter {
        ChannelIter {
            song: self,
            channel_n,
            current_pattern: 0,
            current_row: 0,
            max_rows: self.total_rows(),
        }
    }
}

pub struct ChannelIter<'a> {
    song: &'a Song,
    channel_n: usize,
    current_pattern: usize,
    current_row: usize,
    max_rows: usize,
}

impl<'a> Iterator for ChannelIter<'a> {
    type Item = Cell;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_row < self.max_rows {
            if let Some(pattern) = self.song.patterns.get(self.current_pattern) {
                if let Some(channel) = pattern.channels.get(self.channel_n) {
                    if let Some(cell) = channel.cells.get(self.current_row) {
                        self.current_row += 1;
                        if pattern.is_outside_bounds(self.current_row) {
                            self.current_pattern += 1;
                            self.current_row = 0;
                        }
                        return Some(cell.clone());
                    }
                }
            }
            None
        } else {
            None
        }
    }
}
