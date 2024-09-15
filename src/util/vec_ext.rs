pub trait VecExt {
    fn suffix(&mut self,suffix:&str)->Vec<String>;
}

impl VecExt for Vec<String> {
    /// adds the suffix to the end of each element
    fn suffix(&mut self,suffix:&str)->Vec<String> {
        self.iter().map(|x|format!("{}{}",x,suffix)).collect()
    }
}