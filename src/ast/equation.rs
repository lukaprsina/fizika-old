use crate::{ast::NodeOrExpression, tokenizer::Operation};

use super::{Expression, Product};

#[derive(Debug)]
pub struct Equation {
    pub expressions: Vec<(NodeOrExpression, Option<Operation>)>,
}

impl Equation {
    pub fn flatten(self: &Self) {
        for (expression, _) in self.expressions.iter() {
            if let NodeOrExpression::Expression(expression) = expression {
                expression.flatten();
            }
        }
    }
}

pub trait Flatten {
    fn flatten(self: &Self) -> Vec<usize>;
}

impl Flatten for Expression {
    fn flatten(self: &Self) -> Vec<usize> {
        for product in self.products.iter() {
            product.flatten();
        }

        vec![]
    }
}

impl Flatten for Product {
    fn flatten(self: &Self) -> Vec<usize> {
        for node in self.denominator.iter() {
            if let NodeOrExpression::Expression(expression) = node {
                expression.flatten();
            }

            if self.numerator.is_empty() {
                // TODO: maybe return modified expression
            } else {
            }
        }

        vec![]
    }
}
