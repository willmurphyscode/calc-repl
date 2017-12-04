use parser;
use token::{Token, Opcode};
use runtime_error::RuntimeError;

pub fn eval(tokens: Vec<Token>) -> Result<isize, RuntimeError> {

    // shift reduce parsing
    // two operations
    // shift: advance our position in the array of tokens by one,
    // and push the token that's now to our left onto a stack.
    // reduce: reduce operands that are on the left side of the stack
    // by applying an operator, and replace on the stack the result of the
    // reduction

    let mut left_side = &tokens[..];
    let mut stack : Vec<Token> = Vec::new();
    while left_side.len() > 0 {
        left_side = shift(&mut stack, left_side);
        reduce(&mut stack);
    }
    Err(RuntimeError{})
}

fn shift<'a>(stack: &mut Vec<Token>, remaining: &'a [Token]) -> &'a [Token] {
    if(remaining.len() > 0) {
        stack.push(remaining[0]);
    }
    &remaining[1..]
}

fn reduce<'a>(stack: &mut Vec<Token>) {
    // this accepts the stack of tokens
    // that starts and ends with parentheses, and includes only operands
    // and operators
    // TODO get the rightmost set of stuff wrapped by parentheses
    for ix in range(0usize, stack.len() -1).rev() {
        
    }
}