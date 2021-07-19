use std::collections::HashMap;

pub trait Runnable {
    fn run(&self, memory: HashMap<i128, i128>) -> i128;
}