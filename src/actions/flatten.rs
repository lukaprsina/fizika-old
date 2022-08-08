use tracing::info;

use crate::ast::{product::Product, Element, Expression, NodeOrExpression, Sign};

pub enum FlattenResult {
    Monomial(Product),
    Polynomial,
}

impl Expression {
    pub fn flatten(self) -> FlattenResult {
        for product in self.products.into_iter() {
            for (side_pos, side) in [product.numerator, product.denominator]
                .into_iter()
                .enumerate()
            {
                let side_len = side.len();
                for element in side.into_iter() {
                    match element.node_or_expression {
                        NodeOrExpression::Expression(expression) => {
                            match_expression(expression, element.sign, side_len);
                        }
                        NodeOrExpression::Node(_) => todo!(),
                    }
                }
            }
        }
        FlattenResult::Polynomial
    }
}

fn match_expression(expression: Expression, sign: Sign, side_len: usize) {
    let result = expression.flatten();

    match sign {
        Sign::Positive => match side_len {
            0 => unreachable!(),
            1 => (),
            // > 1
            _ => (),
        },
        Sign::Negative => todo!(),
    }
}
