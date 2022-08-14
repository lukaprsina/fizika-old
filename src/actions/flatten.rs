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
    pub fn flatten(self) -> (Expression, FlattenResult) {
        info!("Flatten: {}", self);
        let mut new_expression = Expression::new(vec![]);

        for product in self.products.into_iter() {
            let mut new_product = Product::new(vec![], vec![]);

            for (side_pos, side) in [product.numerator, product.denominator]
                .into_iter()
                .enumerate()
            {
                let num_products_in_side = side.len();

                for element in side.into_iter() {
                    match element.node_or_expression {
                        NodeOrExpression::Expression(expression) => {
                            let (mut new_expr, flatten_result) = expression.flatten();
                            info!("Flatten result: {:?} -> {}", flatten_result, new_expr);

                            let create_new_element = match element.sign {
                                Sign::Positive => {
                                    if new_expr.products.len() == 1 {
                                        let new = new_expr.products.remove(0);
                                        new_product.numerator.extend(new.numerator.into_iter());
                                        new_product.denominator.extend(new.denominator.into_iter());
                                        false
                                    } else {
                                        true
                                    }
                                }
                                Sign::Negative => true,
                            };

                            if create_new_element {
                                let new_elem = Element::new(
                                    element.sign,
                                    NodeOrExpression::Expression(new_expr),
                                );

                                move_element_to_product(new_elem, &mut new_product, side_pos)
                            }
                        }
                        NodeOrExpression::Node(_) => {
                            move_element_to_product(element, &mut new_product, side_pos)
                        }
                    }
                }
            }

            new_expression.products.push(new_product);
        }

        let flatten_result = if new_expression.products.len() == 1 {
            FlattenResult::Monomial
        } else {
            FlattenResult::Polynomial
        };

        (new_expression, flatten_result)
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
