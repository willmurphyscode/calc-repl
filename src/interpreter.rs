use parser;
use token::{Token, Opcode};
use runtime_error::RuntimeError;
use std::ops::Range;

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
        if stack.last() == Some(&Token::RightParen) {
            reduce(&mut stack);
        }
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
    let mut stack_to_resolve = Vec::new();
    for ix in (0usize..stack.len() -1).rev() {
        match stack[ix] {
            Token::LeftParen => break,
            _ => stack_to_resolve.push(stack[ix]),
        }
    }
    println!("Stack to resolve was {:?}", stack_to_resolve);
    let operator = stack_to_resolve.pop().unwrap();
    println!("Operator was {:?}", operator);
    let result_here : Result<isize, RuntimeError> = match operator {
        Token::Operator(opcode) => {
            match opcode {
                Opcode::Add => reduce_addition(&mut stack_to_resolve),
                _ => Err(RuntimeError{})
            }
        },
        _ => Err(RuntimeError{})
    };
    println!("Result was {:?}", result_here);
}

fn reduce_addition(stack: &mut Vec<Token>) -> Result<isize, RuntimeError> {
    Ok(stack.iter().fold(Token::Operand(0isize), 
        |sum, value| add_tokens(sum, value).expect("")
    ))
}

fn add_tokens(a: Token, b: Token) -> Result<Token,RuntimeError> {
    if let Token::Operand(a_value) = a {
        if let Token::Operand(b_value) = b {
            Ok(Token::Operand(a_value + b_value))
        } else {
            Err(RuntimeError{})
        }
    } else {
        Err(RuntimeError{})
    }
}

#[test]
fn it_adds_tokesn() {
    let a = Token::Operand(3);
    let b = Token::Operand(4);
    let expected = Token::Operand(7);
    let actual = add_tokens(a,b).expect("Unexpected addition failure");
    assert!(expected == actual);
}

#[test]
fn it_adds_arrays() {
    let mut array = vec![
        Token::Operand(1),
        Token::Operand(2),
        Token::Operand(3)
    ];
    let expected = 6;
    let actual = reduce_addition(&mut array).expect("Unexpected addition failure");
    assert!(expected == actual);
}