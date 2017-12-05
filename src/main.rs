#[macro_use]
extern crate nom;

mod interpreter;
mod parser;
mod runtime_error;
mod token;
mod tokenization_error;

fn main() {
    let tokens = parser::parse(&b"(+ 3 4)"[..]).expect("failed to parse valid input");
    println!("{:?}", tokens);
    let result = interpreter::eval(tokens);
    println!("{:?}", result);
    let a = token::Token::Operand(32);
    println!("{}", a);
    println!("Hello, world!");
}
