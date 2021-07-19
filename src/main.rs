#![allow(dead_code)]

use std::env;
use std::fs;
use std::process;

mod lang;

use crate::lang::matrix::chars;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("\u{001b}[31m Exactly 1 argument is required: `filename`\u{001b}[0m");
        process::exit(1);
    }

    let filename: &String = &args[1];

    let code: String = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let lines: Vec<Vec<char>> = chars(&code);

    for line in lines {
        for chr in line {
            print!("{}", chr);
        }
        println!();
    }
}
