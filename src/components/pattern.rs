use crate::components::channel::Channel;

pub struct Pattern {
    pub hex: String,
    pub channels: Vec<Channel>,
}

impl Pattern {
    pub fn new() -> Self {
        Self {
            hex: "".to_string(),
            channels: vec![],
        }
    }
    /// TODO: account for D00 and C00 pattern breaks
    pub fn get_len(&self) -> usize {
        if let Some(channel) = self.channels.get(0) {
            channel.cells.len()
        } else {
            panic!("Pattern has no channels for some reason")
        }
    }
    /// TODO: account for D00 and C00 pattern breaks (Currently does not)
    pub fn is_outside_bounds(&self, row: usize) -> bool {
        row >= self.get_len()
    }
}
