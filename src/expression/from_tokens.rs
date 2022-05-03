use crate::{
    tokenizer::{parser::TokenizedString, token::Associativity, Token},
    Expression,
};

use super::ast::ExpressionOrEquation;

pub enum TokenParseError {}

impl From<TokenizedString> for Result<ExpressionOrEquation, TokenParseError> {
    fn from(tokens: TokenizedString) -> Self {
        let mut result = Expression::new();
        let mut stack: Vec<Token> = Vec::new();

        for token in tokens.tokens.iter() {
            match token {
                Token::Binary(_) => {
                    let pa1 = token.get_precedence_and_associativity();
                    for prev_token in stack.iter_mut().rev() {
                        let pa2 = prev_token.get_precedence_and_associativity();
                        match (pa1, pa2) {
                            ((i, Associativity::Left), (j, _)) if i <= j => {}
                            ((i, Associativity::Right), (j, _)) if i < j => {}
                            _ => {
                                break;
                            }
                        }
                    }
                }
                Token::Unary(_) => todo!(),
                Token::LeftParenthesis => todo!(),
                Token::RightParenthesis => todo!(),
                Token::Comma => todo!(),
                Token::Number(_) | Token::Identifier { .. } => todo!(),
                Token::Function(_, _) => todo!(),
            }
        }
        Ok(ExpressionOrEquation::Expression(result))
    }
}
