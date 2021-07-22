use std::collections::HashMap;

pub trait Runnable {
    fn run(&mut self, memory: &mut HashMap<i128, i128>) -> Result<(i128, String), &str>;
}
