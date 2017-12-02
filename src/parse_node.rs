use std::fmt;

pub enum ParseNode {
    LeftParen,
    RightParen,
    Operator(Opcode),
    Operand(isize)
}

#[derive(Clone, Copy)]
pub enum Opcode {
    Add, Subtract, Multiply, Divide,
}

impl fmt::Display for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Opcode::Add => write!(f, "Add"),
            Opcode::Subtract => write!(f, "Subtract"),
            Opcode::Multiply => write!(f, "Multiply"),
            Opcode::Divide => write!(f, "Divide"),
        }
    }
}

impl fmt::Display for ParseNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        match *self {
            ParseNode::LeftParen => write!(f, "Token:LeftParen"),
            ParseNode::RightParen => write!(f, "Token:RightParen"),
            ParseNode::Operator(code) => write!(f, "Token:Operator:{}", code),
            ParseNode::Operand(value) => write!(f, "Token:Operand:{}", value),
            _ => write!(f, "not implemented")
        }
    }
}