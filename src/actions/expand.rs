use std::ops::{Div, Mul};

use crate::ast::{product::Product, Element, Expression, NodeOrExpression, Sign};

impl Expression {
    pub fn expand(&mut self) {
        // info!("\nExpand: {}", self);

        let mut has_expression = false;
        for product in self.products.iter_mut() {
            for side in [&mut product.numerator, &mut product.denominator] {
                for element in side {
                    if let NodeOrExpression::Expression(expr) = &mut element.node_or_expression {
                        has_expression = true;
                        expr.expand();
                    }
                }
            }
        }

        // info!("Element {}: {}", self, has_expression);

        if has_expression {
            let mut new_expression = Expression::new(vec![]);

            for product in self.products.iter_mut() {
                let mut last_element: Option<Element> = None;

                for (side_pos, side) in [&mut product.numerator, &mut product.denominator]
                    .into_iter()
                    .enumerate()
                {
                    for element in side {
                        let new_element = match &mut last_element {
                            Some(last) => match side_pos {
                                0 => last.clone() * element.clone(),
                                1 => last.clone() / element.clone(),
                                _ => unreachable!(),
                            },
                            None => element.clone(),
                        };

                        last_element = Some(new_element);
                    }
                }

                if let Some(element) = last_element {
                    // info!("Last element: {}, product: {}", element, product);
                    new_expression
                        .products
                        .push(Product::new(vec![element], vec![]))
                }
            }

            *self = new_expression;
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

impl Div for Element {
    type Output = Element;

    fn div(self, mut rhs: Self) -> Self::Output {
        let mut lhs = self.simple_mul_sign(rhs.sign);

        match &mut lhs.node_or_expression {
            NodeOrExpression::Node(_) => Element::new(
                Sign::Positive,
                NodeOrExpression::Expression(Expression::new(vec![Product::new(
                    vec![lhs],
                    vec![rhs],
                )])),
            ),
            NodeOrExpression::Expression(lhs_expr) => match &mut rhs.node_or_expression {
                NodeOrExpression::Node(_) => {
                    for product in lhs_expr.products.iter_mut() {
                        product.denominator.push(rhs.clone())
                    }

                    lhs
                }
                NodeOrExpression::Expression(rhs_expr) => {
                    let mut new_expr = Expression::new(vec![]);

                    for rhs_product in rhs_expr.products.iter_mut() {
                        for lhs_product in lhs_expr.products.iter_mut() {
                            let mut new_product = Product::new(
                                lhs_product.numerator.clone(),
                                lhs_product.denominator.clone(),
                            );

                            new_product
                                .numerator
                                .extend(rhs_product.denominator.clone());
                            new_product
                                .numerator
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
