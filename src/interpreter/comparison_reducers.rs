use token::{Token, Opcode, Type};
use runtime_error::RuntimeError;

pub fn reduce_gt(stack: &mut Vec<Token>) -> Result<Token, RuntimeError> {
    // let bool_result = unwrap_bool_token_vec(stack);
    // if let Ok(bool_vec) = bool_result {
    //     Ok(Token::Operand(Type::Bool(
    //         bool_vec.iter().all(|b| *b)
    //     )))
    // } else {
    //     Err(RuntimeError{ })
    // }
    unimplemented!()
}

pub fn reduce_lt(stack: &mut Vec<Token>) -> Result<Token, RuntimeError> {
    // let bool_result = unwrap_bool_token_vec(stack);
    // if let Ok(bool_vec) = bool_result {
    //     Ok(Token::Operand(Type::Bool(
    //         bool_vec.iter().all(|b| *b)
    //     )))
    // } else {
    //     Err(RuntimeError{ })
    // }
    unimplemented!()
}

#[test]
fn one_is_less_than_two() {
    let mut stack = vec![
        Token::Operator(Opcode::Lt),
        Token::Operand(Type::Integer(1)),
        Token::Operand(Type::Integer(2)),
    ];
    let expected = Token::Operand(Type::Bool(true));
    let actual = reduce_lt(&mut stack).expect("failed to reduce stack for LT");
    assert!(expected == actual);
}

#[test]
fn two_is_greater_than_one() {
    let mut stack = vec![
        Token::Operator(Opcode::Gt),
        Token::Operand(Type::Integer(2)),
        Token::Operand(Type::Integer(1)),
    ];
    let expected = Token::Operand(Type::Bool(true));
    let actual = reduce_lt(&mut stack).expect("failed to reduce stack for LT");
    assert!(expected == actual);
}