use std::collections::HashMap;

use crate::{ast::NodeOrExpression, tokenizer::Operation};

use super::{expression, Expression, Product};

#[derive(Debug, Clone)]
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

enum FlattenResult {
    Monomial,
    Polynomial,
    NotPossible,
}

impl Expression {
    fn flatten(self: &mut Self) -> FlattenResult {
        let mut new_products = Vec::new();

        for mut product in self.products.iter().cloned() {
            // let only_one = product.numerator.len() <= 1;
            let mut changed = false;

            for index in 0..product.numerator.len() - 1 {
                let result = if let NodeOrExpression::Expression(expression) =
                    &mut product.numerator[index]
                {
                    expression.flatten()
                } else {
                    FlattenResult::NotPossible
                };

                let mut new_product = product.clone();
                let node_or_expression = &mut product.numerator[index];
                if let NodeOrExpression::Expression(expression) = node_or_expression {
                    /* if expression.flatten() && only_one {
                        new_products.append(&mut expression.products);
                        changed = true;
                    } */
                    match result {
                        FlattenResult::Polynomial => {
                            new_products.append(&mut expression.products);
                            changed = true;
                        }
                        FlattenResult::Monomial => {
                            new_product
                                .numerator
                                .append(&mut expression.products[0].numerator);
                            new_products.push(new_product);
                            changed = true;
                        }
                        FlattenResult::NotPossible => (),
                    }
                }
            }

            if !changed {
                new_products.push(product);
            }
        }

        self.products = new_products;

        if self.products.len() <= 1 {
            return FlattenResult::Monomial;
        }

        let mut can_flatten = FlattenResult::Polynomial;

        for product in self.products.iter() {
            for node_or_expression in product.numerator.iter() {
                if matches!(node_or_expression, NodeOrExpression::Expression(_)) {
                    can_flatten = FlattenResult::NotPossible;
                    break;
                }
            }
        }

        can_flatten
    }
}
