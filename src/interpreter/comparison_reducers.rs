use std::isize;
use token::{Token, Type};
use runtime_error::RuntimeError;
use super::helpers;

pub fn reduce_gt(stack: &mut Vec<Token>) -> Result<Token, RuntimeError> {
    let integers_vec = helpers::unwrap_integer_tokens(stack);
    if let Ok(integers) = integers_vec {
        if integers.len() >= 2 {
            let mut result = true;
            // rev because what's coming in was built up by pushing a stack,
            // but order matters for these comparators
            integers.iter().rev().fold(isize::MAX, |a, b| {
                result = result && (a > *b);
                *b
            });
            return Ok(Token::Operand(Type::Bool(
                result
            )));
        }
    }
    Err(RuntimeError{})
}

pub fn reduce_lt(stack: &mut Vec<Token>) -> Result<Token, RuntimeError> {
    let integers_vec = helpers::unwrap_integer_tokens(stack);
    if let Ok(integers) = integers_vec {
        if integers.len() >= 2 {
            let mut result = true;
            // rev because what's coming in was built up by pushing a stack,
            // but order matters for these comparators
            integers.iter().rev().fold(isize::MIN, |a, b| {
                result = result && (a < *b);
                *b
            });
            return Ok(Token::Operand(Type::Bool(
                result
            )));
        }
    }
    Err(RuntimeError{})
}

#[test]
fn one_is_less_than_two() {
    let mut stack = vec![
        Token::Operand(Type::Integer(1)),
        Token::Operand(Type::Integer(2)),
    ].into_iter().rev().collect();
    let expected = Token::Operand(Type::Bool(true));
    let actual = reduce_lt(&mut stack).expect("failed to reduce stack for LT");
    assert!(expected == actual);
}

#[test]
fn two_is_greater_than_one() {
    let mut stack = vec![
        Token::Operand(Type::Integer(2)),
        Token::Operand(Type::Integer(1)),
    ].into_iter().rev().collect();
    let expected = Token::Operand(Type::Bool(true));
    let actual = reduce_gt(&mut stack).expect("failed to reduce stack for LT");
    assert!(expected == actual);
}

// TODO think hard about how stacks get reversed in order