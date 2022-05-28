use std::collections::HashMap;

use crate::{ast::NodeOrExpression, tokenizer::Operation};

use super::{Expression, Product};

#[derive(Debug)]
pub struct Equation {
    pub expressions: Vec<(NodeOrExpression, Option<Operation>)>,
}

impl Equation {
    pub fn flatten(self: &mut Self) {
        for (expression, _) in self.expressions.iter_mut() {
            if let NodeOrExpression::Expression(expression) = expression {
                expression.flatten();
            }
        }
    }
}

/* pub trait Flatten {
    fn flatten(self: &mut Self) -> bool;
} */

impl Expression {
    fn flatten(self: &mut Self) -> bool {
        let mut result = Vec::new();
        for index in 0..self.products.len() - 1 {
            result.push(self.products[index].clone());
            result.append(&mut self.products[index].flatten());
        }

        self.products = result;

        // return if it is necesarry to flatten
        true
    }
}

impl Product {
    fn flatten(self: &mut Self) -> Vec<Product> {
        let mut result = Vec::new();

        if !self.denominator.is_empty() {
            return vec![];
        }

        for node in self.numerator.iter_mut() {
            if let NodeOrExpression::Expression(expression) = node {
                if expression.flatten() {
                    result.append(&mut expression.products);
                }
            }
        }

        result
    }
}
