use tracing::debug;

use crate::ast::{product::Product, Element, Expression, NodeOrExpression, Sign};

impl Element {
    pub fn flatten(&mut self) {
        let sign = Sign::Positive;

        self.apply_to_every_element_mut(
            &mut |element| {
                // debug!("{element:#?}");

                let node_or_expression = match &mut element.node_or_expression {
                    NodeOrExpression::Node(node) => {
                        // a
                        NodeOrExpression::Node(node.clone())
                    }
                    NodeOrExpression::Expression(expression) => {
                        let mut new_expression = Expression::new(vec![]);

                        for product in &mut expression.products {
                            let is_surrounded =
                                (product.numerator.len() + product.denominator.len()) >= 2;

                            let mut new_product = Product::new(vec![], vec![]);

                            for (side_pos, side) in
                                [&mut product.numerator, &mut product.denominator]
                                    .into_iter()
                                    .enumerate()
                            {
                                for inner_element in side {
                                    process_inner_element(
                                        inner_element,
                                        &mut new_product,
                                        side_pos,
                                        is_surrounded,
                                    );
                                }
                            }

                            new_expression.products.push(new_product);
                        }

                        NodeOrExpression::Expression(new_expression)
                    }
                };

                *element = Element::new(sign, node_or_expression);
            },
            false,
            None,
        );
    }
}

fn move_element_to_product(element: &Element, new_product: &mut Product, side_pos: usize) {
    match side_pos {
        0 => new_product.numerator.push(element.clone()),
        1 => new_product.denominator.push(element.clone()),
        _ => unreachable!(),
    }
}

fn process_inner_element(
    inner_element: &mut Element,
    new_product: &mut Product,
    side_pos: usize,
    is_surrounded: bool,
) {
    if inner_element.sign != Sign::Positive {
        move_element_to_product(inner_element, new_product, side_pos);
        return;
    }

    let move_to_product = match &mut inner_element.node_or_expression {
        NodeOrExpression::Expression(inner_expression) => {
            if inner_expression.products.len() == 1 {
                transfer_products(inner_expression.clone(), new_product, side_pos);
                false
            } else {
                if !is_surrounded {
                    transfer_products(inner_expression.clone(), new_product, side_pos);
                    false
                } else {
                    // don't move
                    true
                }
            }
        }
        _ => true,
    };

    if move_to_product {
        move_element_to_product(inner_element, new_product, side_pos);
    }
}

fn transfer_products(inner_expression: Expression, new_product: &mut Product, side_pos: usize) {
    for inner_product in inner_expression.products {
        if side_pos == 0 {
            new_product.numerator.extend(inner_product.numerator);

            new_product.denominator.extend(inner_product.denominator);
        } else if side_pos == 1 {
            new_product.numerator.extend(inner_product.denominator);

            new_product.denominator.extend(inner_product.numerator);
        } else {
            panic!("Too many ratio sides");
        }
    }
}
