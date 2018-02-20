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

#[cfg(text)] 
mod tests {
    use super::*;
    use token::Type;

    #[test]
    fn it_subtracts_correctly() {
        let input = b"(- 1 2 2 2)";
        let mut tokens = parser::parse(&input);
        let actual = interpreter::eval(&mut tokens)
            .expect("failed to eval simple subtraction");
        let expected = Type::Integer(-5);
        assert!(expected == actual, "expect {} but got {}", expected, actual);
    }

}