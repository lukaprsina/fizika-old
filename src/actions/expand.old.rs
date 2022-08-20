use std::ops::Mul;

use tracing::info;

use crate::ast::{product::Product, Element, Expression, NodeOrExpression, Sign};

impl Expression {
    // TODO: own self
    pub fn expand(self) -> Expression {
        let mut new_expression = Expression::new(vec![]);

        for product in self.products {
            // 7 * a * (2 - a) / 2 * (b + 4) * 4
            let mut new_product = Product::new(vec![], vec![]);

            for (side_pos, side) in [product.numerator, product.denominator]
                .into_iter()
                .enumerate()
            {
                let mut new_side = vec![];
                let mut last_element: Option<Element> = None;

                for element in side {
                    info!("Element: {}", element);

                    let expanded_expr = match element.node_or_expression.clone() {
                        // TODO: expand node fields
                        NodeOrExpression::Node(_) => None,
                        NodeOrExpression::Expression(expression) => {
                            let expanded_expr = expression.expand();
                            info!("Expr: {}", expanded_expr);
                            Some(expanded_expr)
                        }
                    };

                    match expanded_expr {
                        Some(expr) => {
                            let new_element =
                                Element::new(element.sign, NodeOrExpression::Expression(expr));

                            last_element = if let Some(last) = last_element.clone() {
                                Some(new_element * last)
                            } else {
                                Some(new_element)
                            };
                        }
                        None => {
                            last_element = Some(element.clone());
                        }
                    }

                    match last_element.clone() {
                        Some(last_expr) => new_side.push(last_expr),
                        None => new_side.push(element),
                    }
                    // new_side.push(last_element.clone().unwrap());
                }

                match side_pos {
                    0 => new_product.numerator = new_side,
                    1 => new_product.denominator = new_side,
                    _ => (),
                }
            }

            new_expression.products.push(new_product);
        }

        new_expression
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
