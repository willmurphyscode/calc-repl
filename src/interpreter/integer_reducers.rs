use token::{Token, Type};
use super::helpers;
use runtime_error::RuntimeError;

pub fn reduce_addition(stack: &mut Vec<Token>) -> Result<Token, RuntimeError> {
    let operands = helpers::unwrap_integer_tokens(stack);
    match operands {
        Ok(operand_vec) => Ok(Token::Operand(Type::Integer(
                operand_vec
                    .iter()
                    .fold(0, |sum, value| sum + value)))),
        Err(_) => Err(RuntimeError{})
    }
}

pub fn reduce_subtraction(stack: &mut Vec<Token>) -> Result<Token, RuntimeError> {
    let operands = helpers::unwrap_integer_tokens(stack);
    match operands {
        Ok(mut operand_vec) =>{
            let initial_positive_option = operand_vec.pop();
            if let Some(initial_positive) = initial_positive_option {
                Ok(Token::Operand(Type::Integer(
                    operand_vec
                        .iter()
                        .fold(initial_positive, |sum, value| sum - value))))
            } else {
                Err(RuntimeError{})
            }
        },
        Err(_) => Err(RuntimeError{})
    }
}

pub fn reduce_multiplication(stack: &mut Vec<Token>) -> Result<Token, RuntimeError> {
    let operands = helpers::unwrap_integer_tokens(stack);
    match operands {
        Ok(operand_vec) => Ok(Token::Operand(Type::Integer(
                operand_vec
                    .iter()
                    .fold(1, |prod, value| prod * value)))),
        Err(_) => Err(RuntimeError{})
    }
}

pub fn reduce_division(stack: &mut Vec<Token>) -> Result<Token, RuntimeError> {
    let operands = helpers::unwrap_integer_tokens(stack);
    match operands {
        Ok(mut operand_vec) =>{
            let initial_numerator_option = operand_vec.pop();
            if let Some(initial_numerator) = initial_numerator_option {
                Ok(Token::Operand(Type::Integer(
                    operand_vec
                        .iter()
                        .fold(initial_numerator, |numerator, value| numerator / value))))
            } else {
                Err(RuntimeError{})
            }
        },
        Err(_) => Err(RuntimeError{})
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_adds_arrays() {
        let mut array = vec![
            Token::Operand(Type::Integer(1)),
            Token::Operand(Type::Integer(2)),
            Token::Operand(Type::Integer(3))
        ];
        let expected = Token::Operand(Type::Integer(6));
        let actual = reduce_addition(&mut array).expect("Unexpected addition failure");
        assert!(expected == actual);
    }

    #[test]
    fn it_subtracts() {
        // getting these from the stack would have reversed them
        let mut array = vec![
            Token::Operand(Type::Integer(1)),
            Token::Operand(Type::Integer(2))
        ].into_iter().rev().collect();
        let expected = Token::Operand(Type::Integer(-1));
        let actual = reduce_subtraction(&mut array).expect("Unexpected addition failure");
        assert!(expected == actual);
    }

    #[test]
    fn it_multiplies() {
        let mut array = vec![
            Token::Operand(Type::Integer(1)),
            Token::Operand(Type::Integer(2)),
            Token::Operand(Type::Integer(3))
        ];
        let expected = Token::Operand(Type::Integer(6));
        let actual = reduce_multiplication(&mut array).expect("Unexpected addition failure");
        assert!(expected == actual);
    }

    #[test]
    fn it_divides() {
        let mut array = vec![
            Token::Operand(Type::Integer(16)),
            Token::Operand(Type::Integer(2)),
            Token::Operand(Type::Integer(2))
        ].into_iter().rev().collect();
        let expected = Token::Operand(Type::Integer(4));
        let actual = reduce_division(&mut array).expect("Unexpected addition failure");
        assert!(expected == actual);
    }
}