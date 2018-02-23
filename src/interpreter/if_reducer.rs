use token::{Token,Type};
use runtime_error::RuntimeError;

pub fn reduce_if(stack: &mut Vec<Token>) -> Result<Token, RuntimeError> {
    // Everything should already be reduced here.
    // so we should have something like true some_value some_value
    if stack.len() == 3 {
        // the stack is backwards because we built it by pushing
        // onto it
        let true_false = stack[2];
        let val_if_true = stack[1];
        let val_if_false = stack[0];
        if let Token::Operand(Type::Bool(val)) = true_false {
            return if val {
                Ok(val_if_true)
            } else {
                Ok(val_if_false)
            };
        }
    }
    Err(RuntimeError{})
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
        ].into_iter().rev().collect();
        let result = reduce_if(&mut input).expect("failed to parse if");
        assert!(result == Token::Operand(Type::Integer(2)), "expected true to return first following operand");
    }
}