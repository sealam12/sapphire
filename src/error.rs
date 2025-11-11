use std::fmt;
use std::error::Error;

#[derive(Debug)] // Required for the Error trait
pub struct ParseError {
    message: String,
}

impl ParseError {
    pub fn new(message: &str) -> Self {
        ParseError {
            message: message.to_string(),
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Parse Error: {}", self.message)
    }
}

impl Error for ParseError {}

//////////////////////////////////////////////

#[derive(Debug)]
pub struct ScanError {
    message: String,
}

impl ScanError {
    pub fn new(message: &str) -> Self {
        ScanError {
            message: message.to_string(),
        }
    }
}

impl fmt::Display for ScanError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Parse Error: {}", self.message)
    }
}

impl Error for ScanError {}