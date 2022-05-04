use crate::{tokenizer::Token, Expression};

use super::{ast::ExpressionOrEquation, token_to_rpn::ReversePolishNotation};

impl From<ReversePolishNotation> for ExpressionOrEquation {
    fn from(rpn: ReversePolishNotation) -> Self {
        let mut result = Expression::new();
        let mut stack: Vec<Token> = Vec::new();

        for token in rpn.tokens.into_iter() {
            match token {
                Token::Binary(operation) => {
                    let left = stack.pop().unwrap();
                    let right = stack.pop().unwrap();
                    // not quite right
                }
                _ => stack.push(token),
            }
        }
        ExpressionOrEquation::Expression(result)
    }
}
