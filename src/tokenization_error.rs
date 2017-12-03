use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct TokenizationError {}

impl Error for TokenizationError {
    fn description(&self) -> &str {
        "Could not tokenize supplied string"
    }
}

impl fmt::Display for TokenizationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Could not tokenize supplied string")
    }
}