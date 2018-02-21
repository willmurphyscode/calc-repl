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

// TODO write some end to end tests that exercise the parser and interpreter together
#[cfg(test)]
mod tests {
    use super::*;
    use token::Type;

    #[test]
    fn it_can_reduce_comparisons() {
        let input = b"(and (> 5 4) (< 0 4))";
        let tokens = parser::parse(&input[..]).expect("failed to tokenize boolean expression");
        let result = interpreter::eval(tokens).expect("failed to eval boolean expression");
        assert!(result == token::Type::Bool(true));
    }

    #[test]
    fn it_subtracts_correctly() {
        let input = b"(- 1 2 2 2)";
        let tokens = parser::parse(&input[..])
            .expect("failed to parse subtraction string");
        let actual = interpreter::eval(tokens)
            .expect("failed to eval simple subtraction");
        let expected = Type::Integer(-5);
        assert!(expected == actual, "expect {} but got {}", expected, actual);
    }
}