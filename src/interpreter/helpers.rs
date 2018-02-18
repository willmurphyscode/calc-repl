use token::{Token, Type};
use runtime_error::RuntimeError;

pub fn unwrap_integer_tokens(tokens: &Vec<Token>) -> Result<Vec<isize>, RuntimeError> {
    let result : Vec<isize> = tokens.iter().filter_map(|t| match *t {
        Token::Operand(Type::Integer(val)) => Some(val),
        _ => None,
    })
    .collect();
    if result.len() == tokens.len() {
        Ok(result)
    } else {
        Err(RuntimeError{})
    }
}