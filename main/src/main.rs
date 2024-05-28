#![allow(non_snake_case)]

pub mod Error;
pub mod run;
pub mod token;

use core::panic;
use std::env;

use Error::error;

use crate::run::run;

fn main() {
    error(2, Error::ErrorType::TypeError);
    let args: Vec<String> = env::args().collect();
    // println!("{:?}", args);
    if args.len() > 3 {
        panic!("ERROR : Usage : oops-r <source>")
    } else if args.len() < 2 {
        println!("Rust with OOPS , Oops-rusty");
        println!(
            "\nUsage : oops-r <source> \n\nHELP : pass -h flag for help
OUTPUT : pass -o flag for output in some desired location
                 "
        );
    } else if args.len() == 2 {
        //   println!("{:?}", args[1]);
        if args[1] == "-h" {
            println!("HELP STATEMENT");
            // todo!("Complete help");
        } else {
            run(args[1].clone());
        }
    } else {
    }
}
