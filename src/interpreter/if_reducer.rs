use token::Token;
use runtime_error::RuntimeError;

pub fn reduce_if(_stack: &mut Vec<Token>) -> Result<Token, RuntimeError> {
    // Everything should already be reduced here.
    // so we should have something like true some_value some_value
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use token::{Token, Type};

    #[test]
    fn it_reduces_if() {
        let mut input = vec![
            Token::Operand(Type::Bool(true)),
            Token::Operand(Type::Integer(2)),
            Token::Operand(Type::Integer(3)),
        ];
        let result = reduce_if(&mut input).expect("failed to parse if");
        assert!(result == Token::Operand(Type::Integer(2)), "expected true to return first following operand");
    }
}