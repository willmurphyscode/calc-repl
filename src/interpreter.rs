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
            println!("Calling reduce with {:?}", stack);
            reduce(&mut stack);
        }
    }
    println!("After all parsing, stack was {:?}", stack);
    if stack.len() == 1 {
        if let Some(Token::Operand(value)) = stack.pop() {
            return Ok(value);
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
    loop {
        let mut current_token = stack.pop();
        if let Some(token) = current_token {
            match token {
                Token::LeftParen => break,
                Token::RightParen => {}, // don't push the right paren onto the stack to resolve
                _ => stack_to_resolve.push(token),
            }
        } else {
            panic!("Stack underflow!");
        }

    }
    println!("Stack to resolve was {:?}", stack_to_resolve);
    let operator = stack_to_resolve.pop().unwrap();
    println!("Operator was {:?}", operator);
    let result_here : Result<Token, RuntimeError> = match operator {
        Token::Operator(opcode) => {
            match opcode {
                Opcode::Add => reduce_addition(&mut stack_to_resolve),
                _ => Err(RuntimeError{})
            }
        },
        _ => Err(RuntimeError{})
    };
    println!("LINE 71: {:?}", result_here);
    if let Ok(reduced_token) = result_here {
        stack.push(reduced_token);
    } else {
        panic!("Syntax error parsing at {:?}", stack_to_resolve);
    }
}

fn reduce_addition(stack: &mut Vec<Token>) -> Result<Token, RuntimeError> {
    Ok(stack.iter().fold(Token::Operand(0isize), 
        |sum, value| combine_tokens(sum, value, &|a,b| a + b)
    )
    )
}

fn add_tokens(a: Token, b: &Token) -> Result<Token,RuntimeError> {
    let add = |left, right| left + right;
    Ok(combine_tokens(a,b, &add))
}

fn combine_tokens(a: Token, b: &Token, operation: &Fn(isize, isize) -> isize) -> Token {
    if let Token::Operand(a_value) = a {
        if let Token::Operand(b_value) = *b {
            return Token::Operand(operation(a_value, b_value));
        }
    }
    panic!("Attempted to fold non-operand tokens");
}

#[test]
fn it_adds_tokesn() {
    let a = Token::Operand(3);
    let b = Token::Operand(4);
    let expected = Token::Operand(7);
    let actual = add_tokens(a, &b).expect("Unexpected addition failure");
    assert!(expected == actual);
}

#[test]
fn it_adds_arrays() {
    let mut array = vec![
        Token::Operand(1),
        Token::Operand(2),
        Token::Operand(3)
    ];
    let expected = Token::Operand(6);
    let actual = reduce_addition(&mut array).expect("Unexpected addition failure");
    assert!(expected == actual);
}

#[test]
fn it_evals_simple_stacks() {
    let tokens = vec![
        Token::LeftParen,
        Token::Operator(Opcode::Add),
        Token::Operand(2),
        Token::Operand(3),
        Token::RightParen
    ];
    let expected = 5;
    let actual = eval(tokens).expect("Failed to eval valid simple addition");
    assert!(expected == actual, "Eval failed on simple addition");
}

#[test]
fn it_handles_nested_addition() {
       let tokens = vec![
        Token::LeftParen,
        Token::Operator(Opcode::Add),
        Token::Operand(2),
        Token::Operand(3),
        Token::LeftParen,
        Token::Operator(Opcode::Add),
        Token::Operand(1),
        Token::Operand(2),
        Token::RightParen,
        Token::RightParen
    ];
    let expected = 8;
    let actual = eval(tokens).expect("Failed to eval nested addition");
    assert!(expected == actual, "Eval incorrect on nested addition");
}