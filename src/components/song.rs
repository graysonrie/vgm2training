use crate::{components::order::Order, util::hex};

use super::{cell::Cell, channel::Channel, pattern::Pattern};

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
    ///
    /// Needs to be called **BEFORE** you split the channels or do anything else since it constructs an entirely new song
    pub fn line_up_patterns(&self) -> Song {
        let mut song = Song::new();

        for (i, entry) in self.order.orders.iter().enumerate() {
            let hex_i = hex::to_hex(i as u32);

            let mut build = Pattern::new();
            build.hex = hex_i.clone();
            for (channel_n, hex) in entry.patternhexes.iter().enumerate() {
                let mut build_channel: Option<Channel> = None;
                if let Some(correct_channel) = self.channel_where_hex(channel_n, &hex) {
                    build_channel = Some(correct_channel.clone());
                } else {
                    println!(
                        "Warning: tried to find channel #{} with hex '{}' but it doesn't exist. Filling it in with a blank channel",
                        channel_n, &hex
                    );
                    build_channel = Some(Channel::new(
                        0,
                        "".to_string(),
                        vec![Cell::new(); self.rows as usize],
                    ));
                }
                if let Some(mut build_channel) = build_channel {
                    build_channel.pattern_hex = hex.clone();
                    build.channels.push(build_channel);
                } else {
                    panic!("where the heck did the channel go?")
                }
            }
            song.patterns.push(build);
        }
        song.tempo = self.tempo;
        song.rows = self.rows;
        song.speed = self.speed;
        // might have to refactor this order part to make sure it has the correct hex labels
        song.order = self.order.clone();
        song.num_channels = self.num_channels;
        println!(
            "new song length: {} original song length: {}",
            song.patterns.len(),
            self.patterns.len()
        );
        song
    }
    fn channel_where_hex(&self, channel_num: usize, hex: &str) -> Option<&Channel> {
        for pattern in self.patterns.iter() {
            for (i, channel) in pattern.channels.iter().enumerate() {
                if i == channel_num && channel.pattern_hex == hex {
                    return Some(channel);
                }
            }
        }
        None
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
    /// ### Example:
    ///
    /// If each channel has a length of 64 and you shrink each channel to 32, you can set `min_len` to 64 and it will
    /// combine channels to put them back at a length of 64
    pub fn concat_channels(&mut self, min_len: usize) {
        let mut i = 0;
        while i < self.patterns.len() {
            let p1_len = {
                if let Some(p1) = self.patterns.get_mut(i) {
                    p1.get_true_len()
                } else {
                    break;
                }
            };

            if p1_len < min_len {
                let mut next_pattern = i + 1;
                while next_pattern < self.patterns.len() {
                    // Create a mutable slice to split the borrow and get distinct mutable references
                    let (p1_slice, p2_slice) = self.patterns.split_at_mut(next_pattern);

                    let p1 = &mut p1_slice[i]; // This is safe since i < next_pattern
                    let p2 = &mut p2_slice[0]; // This is the first element after the split

                    p1.append_pattern(p2);
                    next_pattern += 1;
                    i += 1;

                    if p1.get_true_len() >= min_len {
                        break;
                    }
                }
            }
            i += 1;
        }
    }
    pub fn notes_avg_dist(&self) -> u32 {
        let mut dists: Vec<u32> = Vec::new();
        for pattern in self.patterns.iter() {
            for channel in pattern.channels.iter() {
                dists.push(channel.notes_avg_dist())
            }
        }
        dists.iter().sum::<u32>() / (dists.len() as u32)
    }
    /// set `resize` to `true` if you want to put the channels back at their original row count afterwards
    pub fn shrink_all_channels(&mut self, step_size: usize, resize: bool) {
        for pattern in self.patterns.iter_mut() {
            for channel in pattern.channels.iter_mut() {
                channel.shrink(step_size);
            }
        }
        if resize {
            self.concat_channels(self.rows as usize);
        }
        self.rows = self.patterns.get(0).unwrap().get_true_len() as u32;
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
    type Item = &'a Cell;

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
                        return Some(cell);
                    }
                }
            }
            None
        } else {
            None
        }
    }
}
