#[macro_use]
extern crate nom;

mod interpreter;
mod parser;
mod runtime_error;
mod token;
mod tokenization_error;

fn main() {
    parser::parse(&b"( + 3 4)"[..]);
    let a = token::Token::Operand(32);
    println!("{}", a);
    println!("Hello, world!");
}
