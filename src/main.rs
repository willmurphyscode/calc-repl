#[macro_use]
extern crate nom;

mod parse_node;
mod parser;

fn main() {
    parser::parse();
    let a = parse_node::Token::Operand(32);
    println!("{}", a);
    println!("Hello, world!");
}
