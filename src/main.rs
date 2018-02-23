#![deny(warnings)]

#[macro_use]
extern crate nom;
extern crate termion;

mod interpreter;
mod parser;
mod repl;
mod runtime_error;
mod token;
mod tokenization_error;

fn main() {
    repl::repl();
}

#[cfg(test)]
mod tests {
    use super::*;
    use token::Type;

    macro_rules! test_case {
        ($input:expr, $expected:expr, $msg:expr) => {
            let tokens = parser::parse(&$input[..])
                .expect(&format!("{} at parsing", $msg));
            let result = interpreter::eval(tokens)
                .expect(&format!("{} at eval", $msg));
            assert!(result == $expected, &$msg);
        }
    }

    #[test]
    fn it_can_reduce_comparisons() {
        test_case!(b"(and (> 5 4) (< 0 4))",
                   Type::Bool(true),
                   "failed to eval and with comparisons");
    }

    #[test]
    fn it_can_reduce_comparisons_with_other_ops() {
        test_case!(b"(or (> 5 4 (- 2 1)) (< 7 4))",
                   Type::Bool(true),
                   "failed to eval with bool and comparison");
    }

    #[test]
    fn it_subtracts_correctly() {
        test_case!(b"(- 1 2 2 2)", Type::Integer(-5), "failed to subtract correctly");
    }

    #[test]
    fn it_handle_if_true_branch() {
        test_case!(b"(if (< 2 3) 4 5)",
                  Type::Integer(4),
                  "failed to handle if-else on true branch");
    }

    #[test]
    fn it_handle_if_false_branch() {
        test_case!(b"(if (> 2 3) 4 5)",
                  Type::Integer(5),
                  "failed to handle if-else on true branch");
    }

    #[test]
    fn it_handle_if_nested_bool_expression() {
        test_case!(b"(and (< 4 (if (> 2 3) 4 5)) (> (+ 1 2) (- 1 2)))",
                  Type::Bool(true),
                  "failed to handle a nested if and expression");
    }
}