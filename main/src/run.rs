use std::{fs::File, io::BufReader};

use crate::token::Tokenize;

pub fn run(source: String) {
    //println!("{}", &source);
    let file = File::open(&source).expect("Failed to read file");
    let reader = BufReader::new(file);
    Tokenize(reader);
}
