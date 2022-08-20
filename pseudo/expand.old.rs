use std::ops::Mul;

use tracing::info;

use crate::ast::{product::Product, Element, Expression, NodeOrExpression, Sign};

impl Expression {
    // TODO: own self
    pub fn expand(&mut self) {
        for product in self.products.iter_mut() {
            // 7 * a * (2 - a) / 2 * (b + 4) * 4
            for side in [&mut product.numerator, &mut product.denominator] {
                let mut last_element: Option<Element> = None;

                for element in side {
                    match &mut element.node_or_expression {
                        NodeOrExpression::Node(_) => (), //nodes.push(node),
                        NodeOrExpression::Expression(expression) => expression.expand(),
                    }

                    last_element = if let Some(last) = last_element {
                        Some(element.clone() * last.clone())
                    } else {
                        Some(element.clone())
                    };

                    *element = last_element.clone().unwrap();
                    info!("{}", element);
                }
            }
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
                    let mut new_expr = Expression::new(vec![]);

                    for rhs_product in rhs_expr.products.iter_mut() {
                        for lhs_product in lhs_expr.products.iter_mut() {
                            let mut new_product = Product::new(
                                lhs_product.numerator.clone(),
                                lhs_product.denominator.clone(),
                            );

                            new_product.numerator.extend(rhs_product.numerator.clone());
                            new_product
                                .denominator
                                .extend(rhs_product.denominator.clone());

                            new_expr.products.push(new_product);
                        }
                    }

                    Element::new(Sign::Positive, NodeOrExpression::Expression(new_expr))
                }
            },
        }
    }
}
