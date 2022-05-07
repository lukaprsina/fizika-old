use std::fmt::Display;

use crate::Node;

#[derive(Debug, PartialEq, Clone)]
pub enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
    Mod,
    Power,
    Factorial,
    Equal,
    NotEqual,
    LessThan,
    LessThanOrEqual,
    GreaterThanOrEqual,
    GreaterThan,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Number {
    Int(i64),
    Float(f64),
}

impl Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Number::Int(integer) => write!(f, "{}", integer),
            Number::Float(float) => {
                let mut buffer = ryu::Buffer::new();
                write!(f, "{}", buffer.format(*float))
            }
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Binary(Operation),
    Unary(Operation),
    LeftParenthesis,
    RightParenthesis,
    Comma,
    Number(Number),
    Identifier {
        name: String,
        could_be_unit: bool,
    },
    Function {
        name: String,
        num_of_args: Option<usize>,
        arguments: Vec<Vec<Token>>,
    },
}

#[derive(Clone, Copy)]
pub enum Associativity {
    Left,
    Right,
    NA,
}

impl Token {
    pub fn get_precedence_and_associativity(self: &Self) -> (u32, Associativity) {
        match self {
            Token::Binary(operation) => match operation {
                Operation::Add | Operation::Subtract => (1, Associativity::Left),
                Operation::Multiply | Operation::Divide | Operation::Mod => {
                    (2, Associativity::Left)
                }
                Operation::Power => (4, Associativity::Right),
                Operation::Equal
                | Operation::NotEqual
                | Operation::LessThan
                | Operation::LessThanOrEqual
                | Operation::GreaterThanOrEqual
                | Operation::GreaterThan => (5, Associativity::Left),
                _ => unimplemented!(),
            },
            Token::Unary(operation) => match operation {
                Operation::Add | Operation::Subtract => (3, Associativity::NA),
                Operation::Factorial => (5, Associativity::NA),
                _ => unimplemented!(),
            },
            _ => (0, Associativity::NA),
        }
    }
}
