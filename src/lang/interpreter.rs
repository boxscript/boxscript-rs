use std::collections::HashMap;

pub trait Runnable {
    fn run(
        &mut self,
        memory: &mut HashMap<i128, i128>,
        stdout: &mut String,
    ) -> Result<(i128, String), &str>;
}

pub trait Parser<T> {
    fn parse(expr: &str) -> Result<Vec<T>, &str>;
}

pub trait Validator<T> {
    fn validate<'a>(children: &'a [T], valid: &mut bool) -> Result<(), &'a str>;
}
