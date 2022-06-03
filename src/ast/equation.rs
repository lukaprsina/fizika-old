use crate::{ast::NodeOrExpression, tokenizer::Operation};

#[derive(Debug, Clone)]
pub struct Equation {
    pub expressions: Vec<(NodeOrExpression, Option<Operation>)>,
}

impl Equation {
    pub fn flatten(&mut self) {
        for (expression, _) in self.expressions.iter_mut() {
            if let NodeOrExpression::Expression(expression) = expression {
                expression.flatten();
            }
        }
    }
}
