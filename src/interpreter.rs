use parser;
use token::{Token, Opcode};
use runtime_error::RuntimeError;

pub fn eval(tokens: Vec<Token>) -> Result<isize, RuntimeError> {
    Err(RuntimeError{})
}