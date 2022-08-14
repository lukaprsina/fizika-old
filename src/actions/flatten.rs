use tracing::info;

use crate::ast::{product::Product, Element, Expression, NodeOrExpression, Sign};

#[derive(Debug)]
pub enum FlattenResult {
    Monomial,
    Polynomial,
}

fn move_element_to_product(element: Element, new_product: &mut Product, side_pos: usize) {
    match side_pos {
        0 => new_product.numerator.push(element.clone()),
        1 => new_product.denominator.push(element.clone()),
        _ => unreachable!(),
    }
}

impl Expression {
    #[tracing::instrument(skip_all)]
    pub fn flatten(self) -> Expression {
        info!("Flatten: {}", self);

        let mut result_expression = Expression::new(vec![]);

        for product in self.products.into_iter() {
            // debug!("New product: {}", product);
            let mut result_product = Product::new(vec![], vec![]);

            for (side_pos, side) in [product.numerator, product.denominator]
                .into_iter()
                .enumerate()
            {
                // debug!("Side {}", side_pos);
                // println!("{:#?}", side);
                let num_elements_in_expr = side.len();

                for element in side.into_iter() {
                    // debug!("Element: {}", element);
                    match element.node_or_expression {
                        NodeOrExpression::Expression(expression) => {
                            let mut new_expr = expression.flatten();

                            let create_new_element = match element.sign {
                                Sign::Positive => {
                                    match num_elements_in_expr {
                                        1 => {
                                            info!("num_elements_in_expr 1");
                                            /* for product in new_expr.products.clone() {
                                                result_product.numerator.extend(product.numerator);
                                                result_product
                                                    .denominator
                                                    .extend(product.denominator);
                                            } */

                                            // println!("New expr:\n{}", new_expr);

                                            result_expression
                                                .products
                                                .extend(new_expr.products.clone());
                                            false
                                        }
                                        0 => unreachable!(),
                                        _ => {
                                            info!("num_elements_in_expr > 1");
                                            if new_expr.products.len() == 1 {
                                                let only_product = new_expr.products.remove(0);

                                                result_product
                                                    .numerator
                                                    .extend(only_product.numerator);

                                                result_product
                                                    .denominator
                                                    .extend(only_product.denominator);

                                                false
                                            } else {
                                                true
                                            }
                                        }
                                    }
                                    /* if new_expr.products.len() == 1 {
                                        let new = new_expr.products.remove(0);
                                        new_product.numerator.extend(new.numerator.into_iter());
                                        new_product.denominator.extend(new.denominator.into_iter());
                                        false
                                    } else {
                                        true
                                    } */
                                }
                                Sign::Negative => true,
                            };

                            if create_new_element {
                                let new_elem = Element::new(
                                    element.sign,
                                    NodeOrExpression::Expression(new_expr),
                                );

                                move_element_to_product(new_elem, &mut result_product, side_pos)
                            }
                        }
                        NodeOrExpression::Node(_) => {
                            move_element_to_product(element, &mut result_product, side_pos)
                        }
                    }
                }
            }

            if !result_product.numerator.is_empty() || !result_product.denominator.is_empty() {
                result_expression.products.push(result_product);
            }
        }

        result_expression
    }
}

/* #[tracing::instrument(skip_all)]
fn match_expression(expression: Expression, sign: Sign, side_len: usize) -> Expression {
    let (mut new_expr, flatten_result) = expression.flatten();
    info!("Flatten result: {:#?}", flatten_result);
    info!("New expr: {}", new_expr);

    match sign {
        Sign::Positive => {
            if new_expr.products.len() == 1 {
                let first = new_expr.products.first_mut().unwrap();
            }
            /* match side_len {
                0 => unreachable!(),
                1 => (),
                // > 1
                _ => {

                }
            } */
        }
        Sign::Negative => (),
    }

    new_expr
} */
