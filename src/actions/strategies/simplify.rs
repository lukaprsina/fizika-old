#![allow(unused_variables)]

use tracing::debug;

use crate::ast::{Element, Equation, Expression, NodeOrExpression};

use super::strategy::Strategy;

// assume that it has been analysed
fn simplify_equation(equation: &mut Equation, _: &str) {
    for side_element in &mut equation.eq_sides {
        debug!("{side_element:#?}");

        side_element.apply_to_every_element_mut(
            &mut |element| {
                let node_or_expression = match &mut element.node_or_expression {
                    NodeOrExpression::Expression(expression) => {
                        let mut new_expr = Expression::new(vec![]);
                        for product in &mut expression.products {
                            let rationalized = product.rationalize();
                            new_expr.products.push(rationalized);
                        }

                        NodeOrExpression::Expression(new_expr)
                    }
                    _ => element.node_or_expression.clone(),
                };

                *element = Element::new(element.sign, node_or_expression);
            },
            false,
            None,
        );

        debug!("{side_element:#?}");

        side_element.apply_to_every_element_mut(
            &mut |element| {
                // a
                debug!("{element} {}", element.is_number());
            },
            false,
            None,
        );
    }
}

pub fn get_simplify() -> Strategy {
    Strategy {
        equation: Some(Box::new(simplify_equation)),
    }
}
