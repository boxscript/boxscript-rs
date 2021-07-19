use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("\u{001b}[31m Exactly 1 argument is required: `filename`\u{001b}[0m");
        process::exit(1);
    }

    let filename: &String = &args[1];

    let code: String = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let length: Option<usize> = code.lines().map(|line| line.chars().count()).max();

    if length.is_none() {
        process::exit(0);
    }

    let lines: Vec<Vec<char>> = code
        .lines()
        .map(|line| {
            format!("{1:\u{0}<0$}", length.unwrap(), line)
                .chars()
                .collect()
        })
        .collect();

    for line in lines {
        for chr in line {
            print!("{}", chr);
        }
        println!();
    }
}
