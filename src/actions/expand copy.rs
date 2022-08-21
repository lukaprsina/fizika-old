use std::ops::{Div, Mul};

use tracing::info;

use crate::ast::{product::Product, Element, Expression, NodeOrExpression, Sign};

impl Expression {
    // TODO: own self
    pub fn expand(&mut self) {
        for product in self.products.iter_mut() {
            // 7 * a * (2 - a) / 2 * (b + 4) * 4
            let mut has_expression = false;
            let mut last_element: Option<Element> = None;

            for (side_pos, side) in [&mut product.numerator, &mut product.denominator]
                .into_iter()
                .enumerate()
            {
                let mut new_side: Vec<Element> = vec![];
                let mut nodes: Vec<Element> = vec![];

                for mut element in side.clone() {
                    let is_node = match &mut element.node_or_expression {
                        NodeOrExpression::Node(_) => {
                            nodes.push(element.clone());
                            true
                        }
                        NodeOrExpression::Expression(expression) => {
                            has_expression = true;
                            expression.expand();
                            info!("Expanded: {}", element);
                            false
                        }
                    };

                    let new_element = match last_element {
                        Some(last) => match side_pos {
                            0 => last * element,
                            1 => last / element,
                            _ => unreachable!(),
                        },
                        None => element,
                    };

                    last_element = Some(new_element.clone());

                    info!("Is node?: {}", is_node);
                    info!("Has expression?: {}", has_expression);
                    info!("Multiplied: {}\n", new_element);

                    if !is_node || new_element.to_string() == "(3 * 2)/5 - (4 * 2)/5" {
                        new_side.push(new_element);
                    }
                }

                info!("Has expression?: {}", has_expression);
                for node in nodes.iter() {
                    info!("Node: {}", node);
                }

                side.clear();

                if !has_expression {
                    side.append(&mut nodes);
                }

                side.append(&mut new_side);
                for element in side.iter() {
                    info!("Element: {}", element);
                }

                println!("\n\n");
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
