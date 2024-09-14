use crate::components::channel::Channel;

pub struct Pattern {
    pub channels: Vec<Channel>,
}

impl Pattern {
    pub fn new() -> Self {
        Self { channels: vec![] }
    }
}
