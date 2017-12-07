use token::{Token, Opcode};
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

    while left_side.len() > 0 {
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
    if(remaining.len() > 0) {
        stack.push(remaining[0]);
    }
    &remaining[1..]
}

fn add(a: isize, b: isize) -> isize {
    a + b
}

fn reduce<'a>(stack: &mut Vec<Token>) {
    // this accepts the stack of tokens
    // that starts and ends with parentheses, and includes only operands
    // and operators
    // TODO get the rightmost set of stuff wrapped by parentheses
    let mut stack_to_resolve = Vec::new();
    loop {
        let mut current_token = stack.pop();
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
    Ok(stack.iter().fold(Token::Operand(0isize), 
        |sum, value| combine_tokens(sum, value, &|a,b| a + b)
    )
    )
}

fn reduce_subtraction(stack: &mut Vec<Token>) -> Result<Token, RuntimeError> {
    let initial_positive = stack.pop().unwrap();
    Ok(stack.iter().fold(initial_positive, 
        |sum, value| combine_tokens(sum, value, &|a,b| a - b)
    )
    )
}

fn reduce_multiplication(stack: &mut Vec<Token>) -> Result<Token, RuntimeError> {
    Ok(stack.iter().fold(Token::Operand(1isize), 
        |sum, value| combine_tokens(sum, value, &|a,b| a * b)
    )
    )
}

fn reduce_division(stack: &mut Vec<Token>) -> Result<Token, RuntimeError> {
    let initial_numerator = stack.pop().unwrap();
    Ok(stack.iter().fold(initial_numerator, 
        |sum, value| combine_tokens(sum, value, &|a,b| a / b)
    )
    )
}

fn add_tokens(a: Token, b: &Token) -> Result<Token,RuntimeError> {
    let add = |left, right| left + right;
    Ok(combine_tokens(a,b, &add))
}

fn combine_tokens(a: Token, b: &Token, operation: &Fn(isize, isize) -> isize) -> Token {
    if let Token::Operand(a_value) = a {
        if let Token::Operand(b_value) = *b {
            return Token::Operand(operation(a_value, b_value));
        }
    }
    panic!("Attempted to fold non-operand tokens");
}

#[test]
fn it_adds_tokesn() {
    let a = Token::Operand(3);
    let b = Token::Operand(4);
    let expected = Token::Operand(7);
    let actual = add_tokens(a, &b).expect("Unexpected addition failure");
    assert!(expected == actual);
}

#[test]
fn it_adds_arrays() {
    let mut array = vec![
        Token::Operand(1),
        Token::Operand(2),
        Token::Operand(3)
    ];
    let expected = Token::Operand(6);
    let actual = reduce_addition(&mut array).expect("Unexpected addition failure");
    assert!(expected == actual);
}

#[test]
fn it_evals_simple_stacks() {
    let tokens = vec![
        Token::LeftParen,
        Token::Operator(Opcode::Add),
        Token::Operand(2),
        Token::Operand(3),
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
        Token::Operand(2),
        Token::Operand(3),
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
        Token::Operand(2),
        Token::Operand(3),
        Token::LeftParen,
        Token::Operator(Opcode::Add),
        Token::Operand(1),
        Token::Operand(2),
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
        Token::Operand(2),
        Token::Operand(3),
        Token::LeftParen,
        Token::Operator(Opcode::Subtract),
        Token::Operand(1),
        Token::Operand(2),
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
        Token::Operand(2),
        Token::Operand(3),
        Token::LeftParen,
            Token::Operator(Opcode::Subtract),
            Token::Operand(1),
            Token::Operand(2),
            Token::RightParen,
        Token::LeftParen,
            Token::Operator(Opcode::Multiply),
            Token::LeftParen,
                Token::Operator(Opcode::Divide),
                Token::Operand(4),
                Token::Operand(2),
                Token::RightParen,
            Token::Operand(3),
            Token::RightParen,
        Token::RightParen
    ];

    // (+ 2 3 (- 1 2)(* (/ 4 2) 3)) == 10
    let expected = 10;
    let actual = eval(tokens).expect("Failed to eval complex expression");
    assert!(expected == actual, "failed to get correct result for complex expression");
}