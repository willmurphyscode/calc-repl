#[macro_use]
extern crate nom;

mod token;
mod parser;

fn main() {
    parser::parse();
    let a = token::Token::Operand(32);
    println!("{}", a);
    println!("Hello, world!");
}
