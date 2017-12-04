use parser;
use token::{Token, Opcode};
use runtime_error::RuntimeError;

pub fn eval(tokens: Vec<Token>) -> Result<isize, RuntimeError> {
    Err(RuntimeError{})

    // shift reduce parsing
    // two operations
    // shift: advance our position in the array of tokens by one,
    // and push the token that's now to our left onto a stack.
    // reduce: reduce operands that are on the left side of the stack
    // by applying an operator, and replace on the stack the result of the
    // reduction
}

fn shift<'a>(stack: &'a mut Vec<&'a Token>, remaining: &'a [&'a Token]) {
    if(remaining.len() > 0) {
        stack.push(remaining[0]);
    }
}

fn reduce<'a>(stack: &mut Vec<&Token>, slice: &'a [&'a Token]) -> &'a [&'a Token] {
    // this accepts a slice of tokens
    // that starts and ends with parentheses, and includes only operands
    // and operators
    slice
}