use token::{Token, Type};
use runtime_error::RuntimeError;

pub fn reduce_and(stack: &mut Vec<Token>) -> Result<Token, RuntimeError> {
    let bool_result = unwrap_tokens_to_bools(stack);
    if let Ok(bool_vec) = bool_result {
        Ok(Token::Operand(Type::Bool(
            bool_vec.iter().all(|b| *b)
        )))
    } else {
        Err(RuntimeError{ })
    }
}

pub fn reduce_or(stack: &mut Vec<Token>) -> Result<Token, RuntimeError> {
    let bool_result = unwrap_tokens_to_bools(stack);
    if let Ok(bool_vec) = bool_result {
        Ok(Token::Operand(Type::Bool(
            bool_vec.iter().any(|b| *b)
        )))
    } else {
        Err(RuntimeError{ })
    }
}

fn unwrap_tokens_to_bools(tokens: &Vec<Token>) -> Result<Vec<bool>, RuntimeError> {
    let result : Vec<bool> = tokens.iter().filter_map(|t| match *t {
        Token::Operand(Type::Bool(val)) => Some(val),
        Token::Operand(Type::Integer(val)) => {
            if val == 0 {
                Some(false)
            } else {
                Some(true) // non-zero ints are truthy
            }
        },
        _ => None,
    })
    .collect();
    if result.len() == tokens.len() {
        Ok(result)
    } else {
        Err(RuntimeError{})
    }
}

#[test]
fn it_should_reduce_trues_to_true() {
    let mut stack = vec![
        Token::Operand(Type::Bool(true)),
        Token::Operand(Type::Bool(true)),
        Token::Operand(Type::Bool(true)),
        Token::Operand(Type::Bool(true)),
    ];

    let expected = Token::Operand(Type::Bool(true));
    let actual = reduce_and(&mut stack).expect("error on valid bool stack");
    assert!(expected == actual, "failed to reduce all trues to true with AND");
}

#[test]
fn it_should_reduce_mixed_to_false() {
    let mut stack = vec![
        Token::Operand(Type::Bool(true)),
        Token::Operand(Type::Bool(true)),
        Token::Operand(Type::Bool(false)),
        Token::Operand(Type::Bool(true)),
    ];

    let expected = Token::Operand(Type::Bool(false));
    let actual = reduce_and(&mut stack).expect("error on valid bool stack");
    assert!(expected == actual, "failed to reduce all false on mixed vec with AND");
}

#[test]
fn it_should_reduce_mixed_to_true() {
    let mut stack = vec![
        Token::Operand(Type::Bool(true)),
        Token::Operand(Type::Bool(false)),
        Token::Operand(Type::Bool(false)),
    ];

    let expected = Token::Operand(Type::Bool(true));
    let actual = reduce_or(&mut stack).expect("error on valid bool stack");
    assert!(expected == actual, "failed to reduce mixed vec to true with OR");
}

#[test]
fn it_should_reduce_falses_to_false() {
    let mut stack = vec![
        Token::Operand(Type::Bool(false)),
        Token::Operand(Type::Bool(false)),
        Token::Operand(Type::Bool(false)),
    ];

    let expected = Token::Operand(Type::Bool(false));
    let actual = reduce_or(&mut stack).expect("error on valid bool stack");
    assert!(expected == actual, "failed to reduce all false vec to false with OR");
}

#[test]
fn is_should_reduce_truthy_values_or() {
        let mut stack = vec![
        Token::Operand(Type::Bool(false)),
        Token::Operand(Type::Integer(7)),
        Token::Operand(Type::Bool(false)),
    ];

    let expected = Token::Operand(Type::Bool(true));
    let actual = reduce_or(&mut stack).expect("error on valid bool stack");
    assert!(expected == actual, "failed to reduce all false vec to false with OR");
}

#[test]
fn is_should_reduce_truthy_values_and() {
        let mut stack = vec![
        Token::Operand(Type::Bool(true)),
        Token::Operand(Type::Integer(7)),
        Token::Operand(Type::Integer(0)),
    ];

    let expected = Token::Operand(Type::Bool(false));
    let actual = reduce_and(&mut stack).expect("error on valid bool stack");
    assert!(expected == actual, "failed to reduce all false vec to false with OR");
}