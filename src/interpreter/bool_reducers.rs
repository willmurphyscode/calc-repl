use token::{Token, Type};
use runtime_error::RuntimeError;

pub fn reduce_and(stack: &mut Vec<Token>) -> Result<Token, RuntimeError> {
    let bool_result = unwrap_bool_token_vec(stack);
    if let Ok(bool_vec) = bool_result {
        Ok(Token::Operand(Type::Bool(
            bool_vec.iter().all(|b| *b)
        )))
    } else {
        Err(RuntimeError{ })
    }
}

pub fn reduce_or(stack: &mut Vec<Token>) -> Result<Token, RuntimeError> {
    unimplemented!()
}

fn unwrap_bool_token_vec(tokens: &Vec<Token>) -> Result<Vec<bool>, RuntimeError> {
    let result : Vec<bool> = tokens.iter().filter_map(|t| match *t {
        Token::Operand(Type::Bool(val)) => Some(val),
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