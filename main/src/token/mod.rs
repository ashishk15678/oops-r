#![allow(non_snake_case)]

pub mod enums;

// IMPORTS
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn Tokenize(source: BufReader<File>) {
    for line in source.lines() {
        let line = line.expect("Error reading line");
        println!("{}", line);
    }
}
