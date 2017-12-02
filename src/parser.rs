
use nom;
use parse_node::{ParseNode,Opcode};

//This is a super helpful example
// https://github.com/Rydgel/monkey-rust/blob/32d6db16c6b9c99202deafa8b36175f50f6522af/lib/lexer/mod.rs

named!(left_paren<&[u8], ParseNode>, 
    do_parse!(tag!("(") >> (ParseNode::LeftParen)) // parentheses around `ParseNode::LeftParen` are important
    // you'll get a mysterious syntax error from nom macro expansions without them.
);

named!(right_paren<&[u8], ParseNode>,
    do_parse!(tag!(")") >> (ParseNode::RightParen))
);

pub fn parse() {
    let a = left_paren(&b"("[..]);
    println!("{}", left_paren(&b"("[..]).to_result().expect("failed to parse"));
}

#[test]
fn left_paren_parser() {
    assert!(left_paren(&b"("[..]).to_result().expect("failed to parse left paren") == ParseNode::LeftParen);
}