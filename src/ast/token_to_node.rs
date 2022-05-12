use crate::tokenizer::{parser::TokenizedString, token::Associativity, Operation, Token};

use super::{Equation, Expression, NodeOrExpression};

#[derive(Debug)]
pub enum TokenParseError {
    MismatchedParenthesis(usize),
    UnexpectedComma(usize),
    NotEnoughOperands(usize),
    TooManyOperands,
}

fn tokenized_string_to_expression<'a, I>(iterator: &mut I) -> (Expression, Option<Operation>)
where
    I: Iterator<Item = &'a Token>,
{
    let mut stack: Vec<NodeOrExpression> = Vec::new();
    let mut output: Vec<Token> = Vec::new();
    let mut equal_sign = None;

    for (pos, token) in iterator.enumerate() {
        match token {
            Token::Binary(operation) => {
                if operation.is_equal_sign() {
                    equal_sign = Some(operation.clone());
                    break;
                }
            }
            Token::Unary(_) => (),
            Token::LeftParenthesis => (),
            Token::RightParenthesis => (),
            Token::Comma => (),
            Token::Number(_) => (),
            Token::Identifier {
                name,
                could_be_unit,
            } => (),
            Token::Function {
                name,
                num_of_args,
                arguments,
            } => (),
        }
    }

    (Expression::new(), equal_sign)
}

impl TryFrom<TokenizedString> for Equation {
    type Error = TokenParseError;

    fn try_from(tokenized_string: TokenizedString) -> Result<Equation, TokenParseError> {
        let mut expressions: Vec<(Expression, Option<Operation>)> = Vec::new();
        let mut token_iter = tokenized_string.iter();
        let mut should_continue = true;
        while should_continue {
            let result = tokenized_string_to_expression(&mut token_iter);
            if result.1.is_none() {
                should_continue = false;
            }
            expressions.push(result);
        }
        Ok(Equation { expressions })
    }
}
