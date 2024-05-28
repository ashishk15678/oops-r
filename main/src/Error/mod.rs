use core::panic;
use std::fmt::Display;

pub enum ErrorType {
    TypeError,
}

impl Display for ErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorType::TypeError => write!(f, "TYPE-ERROR"),
            _ => write!(f, "ERROR"),
        }
    }
}

pub fn error(line: u8, e: ErrorType) {
    panic!("{} : Error at line {} ", e, line)
}
