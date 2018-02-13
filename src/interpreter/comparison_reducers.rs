use token::{Token, Type};
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