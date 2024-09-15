pub struct Token{
    key:u32,
    val:String,
}

impl Token{
    pub fn new(key:u32,val:String)->Self{
        Self { key, val }
    }
}