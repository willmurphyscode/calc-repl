
use nom;
use nom::IResult;
use token::{Token,Opcode,Type};
use tokenization_error::TokenizationError;
use std::str;

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

named!(and_operator<&[u8], Token>,
    do_parse!(tag!("and") >> (Token::Operator(Opcode::And)))
);

named!(or_operator<&[u8], Token>,
    do_parse!(tag!("or") >> (Token::Operator(Opcode::Or)))
);

named!(gt_operator<&[u8], Token>,
    do_parse!(tag!(">") >> (Token::Operator(Opcode::Gt)))
);

named!(lt_operator<&[u8], Token>,
    do_parse!(tag!("<") >> (Token::Operator(Opcode::Lt)))
);

named!(if_operator<&[u8], Token>,
    do_parse!(tag!("if") >> (Token::Operator(Opcode::If)))
);

named!(comparator<&[u8], Token>,
    alt!(lt_operator | gt_operator)
);


named!(bool_operator<&[u8], Token>,
    alt!(and_operator | or_operator)
);

named!(bool_literal_true<&[u8], Token>,
    do_parse!(alt!(tag!("#t") | tag!("true")) >> (Token::Operand(Type::Bool(true))))
);

named!(bool_literal_false<&[u8], Token>,
    do_parse!(alt!(tag!("#f") | tag!("false")) >> (Token::Operand(Type::Bool(false))))
);

named!(integer_literal<&[u8], Token>,
    map!(nom::digit, parse_string_to_operand)
);

named!(operand<&[u8], Token>, 
    alt!(
        bool_literal_true |
        bool_literal_false |
        integer_literal
    )
);

fn parse_string_to_operand(slice: &[u8]) -> Token {
    // I believe `unwrap()` is OK because the nom macro digit! shouldn't match anything
    // that doesn't parse as an int.
    let string = str::from_utf8(slice).unwrap();
    Token::Operand(Type::Integer(string.parse::<isize>().unwrap()))
}

named!(single_token<&[u8], Token>,
    alt!(
        left_paren |
        right_paren |
        addition_sign |
        subtraction_sign |
        multiplication_sign |
        division_sign |
        bool_operator |
        comparator |
        if_operator |
        operand
    )
);

named!(tokens<&[u8], Vec<Token>>,
    ws!(many0!(single_token))
);

pub fn parse(bytes: &[u8]) -> Result<Vec<Token>, TokenizationError> {
    let parse_result = tokens(bytes);
    match parse_result {
        IResult::Done(rest, result) => if rest.is_empty() {
            Ok(result)
        } else {
            Err(TokenizationError{})
        },
        IResult::Error(_) => Err(TokenizationError {}),
        IResult::Incomplete(_) => Err(TokenizationError{}),
    }
}

#[test]
fn test_several_tokens() {
    let input = "(+ 3 4)".as_bytes();
    let expected = vec![
        Token::LeftParen,
        Token::Operator(Opcode::Add),
        Token::Operand(Type::Integer(3)),
        Token::Operand(Type::Integer(4)),
        Token::RightParen
    ];

    let actual = tokens(&input).to_result().expect("failed to parse valid string of several tokens");
    assert!(actual == expected, "failed to parse '(+ 3 4)'");
}

#[test]
fn test_several_tokens_with_long_numeric_token() {
    let input = "(+ 3 40424)".as_bytes();
    let expected = vec![
        Token::LeftParen,
        Token::Operator(Opcode::Add),
        Token::Operand(Type::Integer(3)),
        Token::Operand(Type::Integer(40_424)),
        Token::RightParen
    ];

    let actual = tokens(&input).to_result().expect("failed to parse valid string of several tokens");
    assert!(actual == expected, "failed to parse '(+ 3 4)'");
}

#[test]
fn test_single_token() {
    assert!(single_token(&b"("[..]).to_result().expect("failed to parse token (") == Token::LeftParen);
}

#[test]
fn test_single_numeric_token() {
    assert!(single_token(&b"4"[..]).to_result().expect("failed to parse token (") == Token::Operand(Type::Integer(4isize)));
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

#[test]
fn operand_parser() {
    assert!(operand(&b"123"[..]).to_result().expect("failed to parse numeric operand") == Token::Operand(Type::Integer(123isize)));
}

#[test]
fn bool_literal_true_test() {
    assert!(operand(&b"true"[..])
        .to_result()
        .expect("failed to parse literal 'true'") == Token::Operand(Type::Bool(true)));
}

#[test]
fn bool_literal_false_test() {
    assert!(operand(&b"false"[..])
        .to_result()
        .expect("failed to parse literal 'false'") == Token::Operand(Type::Bool(false)));
}

#[test]
fn bool_and_test() {
    assert!(single_token(&b"and"[..])
        .to_result()
        .expect("failed to parse bool op 'and'") == Token::Operator(Opcode::And));
}

#[test]
fn gt_test() {
    assert!(single_token(&b">"[..])
        .to_result()
        .expect("failed to parse op '>'") == Token::Operator(Opcode::Gt));
}


#[test]
fn lt_test() {
    assert!(single_token(&b"<"[..])
        .to_result()
        .expect("failed to parse op '<'") == Token::Operator(Opcode::Lt));
}

#[test]
fn it_can_parse_if() {
    assert!(single_token(&b"if"[..])
        .to_result()
        .expect("failed to parse if") == Token::Operator(Opcode::If));
}