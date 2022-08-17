use std::ops::Mul;

use crate::ast::{product::Product, Element, Expression, Node, NodeOrExpression, Sign};

impl Expression {
    pub fn expand(&mut self) {
        for product in self.products.iter_mut() {
            let new_element = Element::new(
                Sign::Positive,
                NodeOrExpression::Expression(Expression::new(vec![])),
            );

            for side in [&product.numerator, &product.denominator] {}
        }
    }
}

impl Mul for Element {
    type Output = Element;

    fn mul(self, mut rhs: Self) -> Self::Output {
        let mut lhs = self.simple_mul_sign(rhs.sign);

        match &mut lhs.node_or_expression {
            NodeOrExpression::Node(_) => match &mut rhs.node_or_expression {
                NodeOrExpression::Node(_) => Element::new(
                    Sign::Positive,
                    NodeOrExpression::Expression(Expression::new(vec![Product::new(
                        vec![lhs, rhs],
                        vec![],
                    )])),
                ),
                NodeOrExpression::Expression(rhs_expr) => {
                    for product in rhs_expr.products.iter_mut() {
                        product.numerator.push(lhs.clone())
                    }

                    rhs
                }
            },
            NodeOrExpression::Expression(lhs_expr) => match &mut rhs.node_or_expression {
                NodeOrExpression::Node(_) => {
                    for product in lhs_expr.products.iter_mut() {
                        product.numerator.push(rhs.clone())
                    }

                    lhs
                }
                // (7/a + 2) * (ab - 3)
                NodeOrExpression::Expression(rhs_expr) => {
                    for lhs_product in lhs_expr.products.iter_mut() {
                        for rhs_product in rhs_expr.products.iter_mut() {
                            lhs_product.numerator.extend(rhs_product.numerator.clone());
                            lhs_product
                                .denominator
                                .extend(rhs_product.denominator.clone());
                        }
                    }

                    lhs
                }
            },
        }
    }
}
