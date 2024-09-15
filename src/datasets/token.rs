use std::cmp::Ordering;

pub struct Token{
    pub key:u32,
    pub val:String,
}

impl Token{
    pub fn new(key:u32,val:String)->Self{
        Self { key, val }
    }
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key && self.val == other.val
    }
}

// Implement Eq because we need total equality for Ord
impl Eq for Token {}

// Implement PartialOrd to allow for partial comparison
impl PartialOrd for Token {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Implement Ord for total ordering based on key and then val
impl Ord for Token {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.key.cmp(&other.key) {
            Ordering::Equal => {
                self.val.cmp(&other.val)
            }
            other_order => other_order,
        }
    }
}