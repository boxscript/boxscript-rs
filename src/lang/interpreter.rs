use std::collections::HashMap;

pub trait Runnable {
    fn run(
        &mut self,
        memory: &mut HashMap<i128, i128>,
        stdout: &mut String,
    ) -> Result<(i128, String), String>;
}

pub trait Parser<T> {
    fn parse(expr: &str) -> Result<Vec<T>, String>;
}

pub trait Validator<T> {
    fn validate(children: &[T], valid: &mut bool) -> Result<(), String>;
}
