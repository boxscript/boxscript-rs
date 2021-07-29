use num_traits::{PrimInt, Signed, ToPrimitive};

pub trait BoxInt: PrimInt + Signed + ToPrimitive + std::hash::Hash + std::fmt::Display {}

impl BoxInt for i8 {}
impl BoxInt for i16 {}
impl BoxInt for i32 {}
impl BoxInt for i64 {}
impl BoxInt for i128 {}

pub trait Runnable<T> {
    fn run(
        &mut self,
        memory: &mut std::collections::HashMap<T, T>,
        stdout: &mut String,
    ) -> Result<(T, String), String>;
}

pub trait Parser<T> {
    fn parse(expr: &str) -> Result<Vec<T>, String>;
}

pub trait Validator<T> {
    fn validate(children: &[T], valid: &mut bool) -> Result<(), String>;
}
