use std::collections::HashMap;

pub trait Runnable {
    fn run(
        &mut self,
        memory: &mut HashMap<i128, i128>,
        stdout: &mut String,
    ) -> Result<(i128, String), &str>;
}
