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

impl Expression {
    fn flatten(self: &mut Self) -> bool {
        let mut result = Vec::new();
        for index in 0..self.products.len() - 1 {
            /* result.push(self.products[index].clone());
            let mut a = self.products[index].flatten();
            result.append(&mut a);
            println!("{:#?}", result); */
            if self.products[index].flatten() {
                for node in self.products[index].numerator.iter() {
                    if let NodeOrExpression::Expression(expression) = node {
                        result.append(&mut expression.products.clone());
                    } else {
                        result.push(self.products[index].clone());
                    }
                }
            }
        }

        self.products = result;

        // return if it is necesarry to flatten
        false
    }
}

impl Product {
    fn flatten(self: &mut Self) -> bool {
        let mut result = true;

        if !self.denominator.is_empty() {
            return false;
        }

        for node in self.numerator.iter_mut() {
            if let NodeOrExpression::Expression(expression) = node {
                if !expression.flatten() {
                    result = false;
                    break;
                }
            }
        }

        result
    }
}
