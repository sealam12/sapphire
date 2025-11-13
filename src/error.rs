use std::fmt;
use std::error::Error;

#[macro_export]
macro_rules! define_error {
    ($name:ident) => {
        #[derive(Debug)]
        // Required for the Error trait
        pub struct $name {
            pub message: String,
        }

        impl $name {
            pub fn new(message: &str) -> Self {
                $name {
                    message: message.to_string(),
                }
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{}", self.message)
            }
        }
        
        impl Error for $name {}
    };
}

define_error!(ParseError);
define_error!(ScanError);

#[derive(Debug)]
// Required for the Error trait
pub struct RuntimeError {
    pub message: String,
    pub line: usize
}

impl RuntimeError {
    pub fn new(message: &str, line: usize) -> Self {
        RuntimeError {
            message: message.to_string(),
            line: line,
        }
    }
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for RuntimeError {}