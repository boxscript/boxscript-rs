#![allow(dead_code)]

use std::env;
use std::fs;
use std::process;

mod lang;

use crate::lang::matrix::chars;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("\u{001b}[31mExactly 1 argument is required: `filename`\u{001b}[0m");
        process::exit(1);
    }

    let filename: &String = &args[1];

    let content = fs::read_to_string(filename);

    if content.is_err() {
        println!("\u{001b}[31mNo file exists at `{}`\u{001b}[0m", filename);
        process::exit(1);
    }

    let lines: Vec<Vec<char>> = chars(&content.unwrap());

    for line in lines {
        for chr in line {
            print!("{}", chr);
        }
        println!();
    }
}
