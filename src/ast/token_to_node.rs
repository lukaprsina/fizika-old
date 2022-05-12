use crate::tokenizer::{parser::TokenizedString, token::Associativity, Operation, Token};

use super::{Equation, Expression};

#[derive(Debug)]
pub enum TokenParseError {
    MismatchedParenthesis(usize),
    UnexpectedComma(usize),
    NotEnoughOperands(usize),
    TooManyOperands,
}

fn tokenized_string_to_expression(tokenized_string: &TokenizedString) -> (Expression, Operation) {
    let mut stack: Vec<Token> = Vec::new();
    let mut output: Vec<Token> = Vec::new();
    (Expression::new(), Operation::Equal)
}

impl TryFrom<TokenizedString> for Equation {
    type Error = TokenParseError;

    fn try_from(tokenized_string: TokenizedString) -> Result<Equation, TokenParseError> {
        let mut expressions: Vec<(Expression, Operation)> = Vec::new();
        for (pos, token) in tokenized_string.tokens.iter().enumerate() {}
        Ok(Equation { expressions })
    }
}
