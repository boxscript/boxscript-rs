#![allow(dead_code)]

use std::fs;

extern crate ansi_term;
#[macro_use]
extern crate clap;
#[macro_use]
extern crate lazy_static;
extern crate regex;

mod lang;

#[cfg(not(tarpaulin_include))]
fn main() {
    let app = clap_app!(BoxScript =>
        (version: "0.1.0")
        (author: "pyxiis <47072520+pyxiis@users.noreply.github.com>")
        (about: "Runs BoxScript code from a file")
        (@arg file: +required "Sets the input file to use")
    );

    let matches = app.get_matches();

    let file = matches.value_of("file");

    if let Some(filename) = file {
        let content = fs::read_to_string(filename);

        if content.is_err() {
            use ansi_term::Colour::Red;
            eprintln!(
                "{} {}: No such file or directory",
                Red.bold().paint("error:"),
                file.unwrap()
            );
        }
    }
}
