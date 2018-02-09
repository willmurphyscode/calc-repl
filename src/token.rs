use std::fmt;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Token {
    LeftParen,
    RightParen,
    Operator(Opcode),
    Operand(Type),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Opcode {
    Add, Subtract, Multiply, Divide, And, Or
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Type {
    Bool(bool), Integer(isize)
}

impl fmt::Display for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Opcode::Add => write!(f, "Add"),
            Opcode::Subtract => write!(f, "Subtract"),
            Opcode::Multiply => write!(f, "Multiply"),
            Opcode::Divide => write!(f, "Divide"),
            Opcode::And => write!(f, "And"),
            Opcode::Or => write!(f, "Or"),
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Token::LeftParen => write!(f, "Token:LeftParen"),
            Token::RightParen => write!(f, "Token:RightParen"),
            Token::Operator(code) => write!(f, "Token:Operator:{}", code),
            Token::Operand(operand) => write!(f, "Token:Operand:{}", operand),
        }
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Type::Bool(b) => write!(f, "Bool:{}",b),
            Type::Integer(int) => write!(f, "Integer:{}",int),
        }
    }
}