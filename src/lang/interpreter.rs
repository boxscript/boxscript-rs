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
