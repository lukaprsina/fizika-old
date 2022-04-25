use crate::{tokenizer::parser::TokenizedString, Expression};

use super::ast::ExpressionOrEquation;

impl From<TokenizedString> for ExpressionOrEquation {
    fn from(_tokens: TokenizedString) -> Self {
        let a = Expression::new();
        ExpressionOrEquation::Expression(a)
    }
}
