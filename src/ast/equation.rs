use crate::{ast::NodeOrExpression, tokenizer::Operation};

use super::expression::Element;

#[derive(Debug, Clone)]
pub struct Equation {
    pub expressions: Vec<(Element, Option<Operation>)>,
}

impl Equation {
    pub fn flatten(&mut self) {
        for (expression, _) in self.expressions.iter_mut() {
            if let NodeOrExpression::Expression(expression) = &expression.node_or_expression {
                // expression.flatten();
            }
        }
    }
}
