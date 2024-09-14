use crate::components::order::Order;

pub struct Song {
    pub name: String,
    pub rows: u32,
    pub speed: u32,
    pub tempo: u32,
    pub order: Order,
}

impl Song {
    pub fn new() -> Self {
        Self {
            name: "".to_string(),
            rows: 0,
            speed: 0,
            tempo: 0,
            order: Order::new(),
        }
    }
}
