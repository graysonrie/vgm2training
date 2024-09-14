pub struct OrderEntry {
    pub hex: String,
    pub patternhexes: Vec<String>,
}

pub struct Order {
    pub orders: Vec<OrderEntry>,
}

impl Order{
    pub fn new()->Self{
        Self { orders: vec![] }
    }
}