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
            let tokens = parser::parse(&$input[..]).expect($msg);
            let result = interpreter::eval(tokens).expect($msg);
            assert!(result == $expected, $msg);
        }
    }

    #[test]
    fn it_can_reduce_comparisons() {
        test_case!(b"(and (> 5 4) (< 0 4))",
                Type::Bool(true), "failed to eval and with comparisons");
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
}