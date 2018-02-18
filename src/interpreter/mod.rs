use token::{Token, Opcode, Type};
mod bool_reducers;
mod comparison_reducers;
mod helpers;
mod integer_reducers;
use runtime_error::RuntimeError;


pub fn eval(tokens: Vec<Token>) -> Result<Type, RuntimeError> {

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
                Opcode::Add => integer_reducers::reduce_addition(&mut stack_to_resolve),
                Opcode::Subtract => integer_reducers::reduce_subtraction(&mut stack_to_resolve),
                Opcode::Multiply => integer_reducers::reduce_multiplication(&mut stack_to_resolve),
                Opcode::Divide => integer_reducers::reduce_division(&mut stack_to_resolve),
                Opcode::And => bool_reducers::reduce_and(&mut stack_to_resolve),
                Opcode::Or => bool_reducers::reduce_or(&mut stack_to_resolve),
                Opcode::Gt => comparison_reducers::reduce_gt(&mut stack_to_resolve),
                Opcode::Lt => comparison_reducers::reduce_lt(&mut stack_to_resolve),
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


#[cfg(test)]
mod tests {
    use token::Type::*;
    use super::*;

    #[test]
    fn it_evals_simple_stacks() {
        let tokens = vec![
            Token::LeftParen,
            Token::Operator(Opcode::Add),
            Token::Operand(Type::Integer(2)),
            Token::Operand(Type::Integer(3)),
            Token::RightParen
        ];
        let expected = Type::Integer(5);
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
        let expected = Type::Integer(-1);
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
        let expected = Type::Integer(8);
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
        let expected = Type::Integer(4);
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
        let expected = Type::Integer(10);
        let actual = eval(tokens).expect("Failed to eval complex expression");
        assert!(expected == actual, "failed to get correct result for complex expression");
    }

    #[test]
    fn it_should_handle_all_bool_tree() {
        let tokens = vec![
            Token::LeftParen,
            Token::Operator(Opcode::And),
            Token::Operand(Bool(true)),
            Token::Operand(Bool(true)),
            Token::Operand(Bool(true)),
            Token::RightParen
        ];

        let expected = Type::Bool(true);
        let actual = eval(tokens).expect("fail to parse simple bool expression");
        assert!(expected == actual, "failed to eval simple 'and'");
    }
}
