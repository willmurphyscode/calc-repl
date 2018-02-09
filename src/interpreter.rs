use token::{Token, Opcode, Type};
use runtime_error::RuntimeError;


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

    while !left_side.is_empty() {
        left_side = shift(&mut stack, left_side);
        if stack.last() == Some(&Token::RightParen) {
            reduce(&mut stack);
        }
    }
    if stack.len() == 1 {
        if let Some(Token::Operand(value)) = stack.pop() {
            return Ok(value);
        }
    }
    Err(RuntimeError{})
}

fn shift<'a>(stack: &mut Vec<Token>, remaining: &'a [Token]) -> &'a [Token] {
    if !remaining.is_empty() {
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
        let current_token = stack.pop();
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
    let operator = stack_to_resolve.pop().unwrap();
    let result_here : Result<Token, RuntimeError> = match operator {
        Token::Operator(opcode) => {
            match opcode {
                Opcode::Add => reduce_addition(&mut stack_to_resolve),
                Opcode::Subtract => reduce_subtraction(&mut stack_to_resolve),
                Opcode::Multiply => reduce_multiplication(&mut stack_to_resolve),
                Opcode::Divide => reduce_division(&mut stack_to_resolve)
            }
        },
        _ => Err(RuntimeError{})
    };
    if let Ok(reduced_token) = result_here {
        stack.push(reduced_token);
    } else {
        panic!("Syntax error parsing at {:?}", stack_to_resolve);
    }
}

fn reduce_addition(stack: &mut Vec<Token>) -> Result<Token, RuntimeError> {
    let operands = unwrap_operand_tokens(stack);
    match operands {
        Ok(operand_vec) => Ok(Token::Operand(
                operand_vec
                    .iter()
                    .fold(0, |sum, value| sum + value))),
        Err(_) => Err(RuntimeError{})
    }
}

fn reduce_subtraction(stack: &mut Vec<Token>) -> Result<Token, RuntimeError> {
    let operands = unwrap_operand_tokens(stack);
    match operands {
        Ok(mut operand_vec) =>{
            let initial_positive_option = operand_vec.pop();
            if let Some(initial_positive) = initial_positive_option {
                Ok(Token::Operand(
                    operand_vec
                        .iter()
                        .fold(initial_positive, |sum, value| sum - value)))
            } else {
                Err(RuntimeError{})
            }
        },
        Err(_) => Err(RuntimeError{})
    }
}

fn reduce_multiplication(stack: &mut Vec<Token>) -> Result<Token, RuntimeError> {
    let operands = unwrap_operand_tokens(stack);
    match operands {
        Ok(operand_vec) => Ok(Token::Operand(
                operand_vec
                    .iter()
                    .fold(1, |prod, value| prod * value))),
        Err(_) => Err(RuntimeError{})
    }
}

fn reduce_division(stack: &mut Vec<Token>) -> Result<Token, RuntimeError> {
    let operands = unwrap_operand_tokens(stack);
    match operands {
        Ok(mut operand_vec) =>{
            let initial_numerator_option = operand_vec.pop();
            if let Some(initial_numerator) = initial_numerator_option {
                Ok(Token::Operand(
                    operand_vec
                        .iter()
                        .fold(initial_numerator, |numerator, value| numerator / value)))
            } else {
                Err(RuntimeError{})
            }
        },
        Err(_) => Err(RuntimeError{})
    }
}

fn unwrap_operand_tokens(tokens: &Vec<Token>) -> Result<Vec<isize>, RuntimeError> {
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
fn it_evals_simple_stacks() {
    let tokens = vec![
        Token::LeftParen,
        Token::Operator(Opcode::Add),
        Token::Operand(Type::Integer(2)),
        Token::Operand(Type::Integer(3)),
        Token::RightParen
    ];
    let expected = 5;
    let actual = eval(tokens).expect("Failed to eval valid simple addition");
    assert!(expected == actual, "Eval failed on simple addition");
}

#[test]
fn it_evals_simple_stacks_with_subtract() {
    let tokens = vec![
        Token::LeftParen,
        Token::Operator(Opcode::Subtract),
        Token::Operand(Type::Integer(2)),
        Token::Operand(Type::Integer(3)),
        Token::RightParen
    ];
    let expected = -1;
    let actual = eval(tokens).expect("Failed to eval valid simple addition");
    assert!(expected == actual, "Eval failed on simple addition");
}

#[test]
fn it_handles_nested_addition() {
       let tokens = vec![
        Token::LeftParen,
        Token::Operator(Opcode::Add),
        Token::Operand(Type::Integer(2)),
        Token::Operand(Type::Integer(3)),
        Token::LeftParen,
        Token::Operator(Opcode::Add),
        Token::Operand(Type::Integer(1)),
        Token::Operand(Type::Integer(2)),
        Token::RightParen,
        Token::RightParen
    ];
    let expected = 8;
    let actual = eval(tokens).expect("Failed to eval nested addition");
    assert!(expected == actual, "Eval incorrect on nested addition");
}

#[test]
fn it_handles_nested_addition_with_subrtraction() {
       let tokens = vec![
        Token::LeftParen,
        Token::Operator(Opcode::Add),
        Token::Operand(Type::Integer(2)),
        Token::Operand(Type::Integer(3)),
        Token::LeftParen,
        Token::Operator(Opcode::Subtract),
        Token::Operand(Type::Integer(1)),
        Token::Operand(Type::Integer(2)),
        Token::RightParen,
        Token::RightParen
    ];
    let expected = 4;
    let actual = eval(tokens).expect("Failed to eval nested addition");
    assert!(expected == actual, "Eval incorrect on nested addition with subraction");
}

#[test]
fn it_handles_nested_nonses_with_all_ops() {
        let tokens = vec![
        Token::LeftParen,
        Token::Operator(Opcode::Add),
        Token::Operand(Type::Integer(2)),
        Token::Operand(Type::Integer(3)),
        Token::LeftParen,
            Token::Operator(Opcode::Subtract),
            Token::Operand(Type::Integer(1)),
            Token::Operand(Type::Integer(2)),
            Token::RightParen,
        Token::LeftParen,
            Token::Operator(Opcode::Multiply),
            Token::LeftParen,
                Token::Operator(Opcode::Divide),
                Token::Operand(Type::Integer(4)),
                Token::Operand(Type::Integer(2)),
                Token::RightParen,
            Token::Operand(Type::Integer(3)),
            Token::RightParen,
        Token::RightParen
    ];

    // (+ 2 3 (- 1 2)(* (/ 4 2) 3)) == 10
    let expected = 10;
    let actual = eval(tokens).expect("Failed to eval complex expression");
    assert!(expected == actual, "failed to get correct result for complex expression");
}