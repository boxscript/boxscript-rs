#![allow(dead_code)]

use std::env;
use std::process;

mod cli;
mod lang;

#[cfg(not(tarpaulin_include))]
fn main() {
    let args: Vec<String> = env::args().collect();

    let file = cli::env::read(args[1..].to_vec());

    if file.is_err() {
        println!("{}", file.err().unwrap());
        process::exit(1);
    }
}
