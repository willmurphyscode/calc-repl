use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct RuntimeError {}

impl Error for RuntimeError {
    fn description(&self) -> &str {
        "Error evaluating program"
    }
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Could not evaluate supplied string")
    }
}