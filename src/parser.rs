
use nom;
use token::{Token,Opcode};
use std::num::{ParseIntError};
use std::str::{FromStr};

//This is a super helpful example
// https://github.com/Rydgel/monkey-rust/blob/32d6db16c6b9c99202deafa8b36175f50f6522af/lib/lexer/mod.rs

named!(left_paren<&[u8], Token>, 
    do_parse!(tag!("(") >> (Token::LeftParen)) // parentheses around `Token::LeftParen` are important
    // you'll get a mysterious syntax error from nom macro expansions without them.
);

named!(right_paren<&[u8], Token>,
    do_parse!(tag!(")") >> (Token::RightParen))
);

named!(addition_sign<&[u8], Token>,
    do_parse!(tag!("+") >> (Token::Operator(Opcode::Add)))
);

named!(subtraction_sign<&[u8], Token>,
    do_parse!(tag!("-") >> (Token::Operator(Opcode::Subtract)))
);

named!(multiplication_sign<&[u8], Token>,
    do_parse!(tag!("*") >> (Token::Operator(Opcode::Multiply)))
);

named!(division_sign<&[u8], Token>,
    do_parse!(tag!("/") >> (Token::Operator(Opcode::Divide)))
);

named!(operand<&str, Token>,
    map!(nom::digit, parse_string_to_operand)
);

fn parse_string_to_operand(string: &str) -> Token {
    // I believe `unwrap()` is OK because the nom macro digit! shouldn't match anything
    // that doesn't parse as an int. 
    Token::Operand(isize::from_str(string).unwrap())
}

pub fn parse() {
    let a = left_paren(&b"("[..]);
    println!("{}", left_paren(&b"("[..]).to_result().expect("failed to parse"));
}

#[test]
fn left_paren_parser() {
    assert!(left_paren(&b"("[..]).to_result().expect("failed to parse left paren") == Token::LeftParen);
}

#[test]
fn right_paren_parser() {
    assert!(right_paren(&b")"[..]).to_result().expect("failed to parse right paren") == Token::RightParen);
}

#[test]
fn addition_sign_parser() {
    assert!(addition_sign(&b"+"[..]).to_result().expect("failed to parse addition sign") == Token::Operator(Opcode::Add));
}

#[test]
fn subtraction_sign_parser() {
    assert!(subtraction_sign(&b"-"[..]).to_result().expect("failed to parse subtraction sign") == Token::Operator(Opcode::Subtract));
}

#[test]
fn multiplication_sign_parser() {
    assert!(multiplication_sign(&b"*"[..]).to_result().expect("failed to parse multiplication sign") == Token::Operator(Opcode::Multiply));
}

#[test]
fn division_sign_parser() {
    assert!(division_sign(&b"/"[..]).to_result().expect("failed to parse division sign") == Token::Operator(Opcode::Divide));
}