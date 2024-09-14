use crate::components::channel::Channel;

pub struct Pattern {
    pub hex:String,
    pub channels: Vec<Channel>,
}

impl Pattern {
    pub fn new() -> Self {
        Self { hex:"".to_string(), channels: vec![] }
    }
}
