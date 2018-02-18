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

    #[test]
    fn it_can_reduce_comparisons() {
        let input = b"(and (> 5 4) (< 0 4))";
        let tokens = parser::parse(&input[..]).expect("failed to tokenize boolean expression");
        let result = interpreter::eval(tokens).expect("failed to eval boolean expression");
        assert!(result == token::Type::Bool(true));
    }
}