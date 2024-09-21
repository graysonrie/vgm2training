#[derive(Clone)]
pub struct OrderEntry {
    pub number:u32,
    pub hex: String,
    pub patternhexes: Vec<String>,
}

impl OrderEntry{
    pub fn new()->Self{
        Self { number: 0, hex: "".to_string(), patternhexes: vec![] }
    }
}

#[derive(Clone)]
pub struct Order {
    pub orders: Vec<OrderEntry>,
}

impl Order{
    pub fn new(orders:Vec<OrderEntry>)->Self{
        Self { orders }
    }
}