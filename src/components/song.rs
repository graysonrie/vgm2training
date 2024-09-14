use crate::components::order::Order;

use super::pattern::Pattern;

pub struct Song {
    pub name: String,
    pub rows: u32,
    pub speed: u32,
    pub tempo: u32,
    pub order: Order,
    pub patterns:Vec<Pattern>,
    pub num_channels:usize,
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
            num_channels:0,
        }
    }
    /// adjusts the channels in the patterns based off the orders
    pub fn line_up_patterns(&mut self){
        println!("lining up patterns");
    }
}
