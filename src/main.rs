#[macro_use]
extern crate nom;

mod token;
mod tokenization_error;
mod parser;

fn main() {
    parser::parse(&b"( + 3 4)"[..]);
    let a = token::Token::Operand(32);
    println!("{}", a);
    println!("Hello, world!");
}
