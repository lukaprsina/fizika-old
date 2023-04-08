#![allow(unused_variables)]
use crate::ast::{Equation, NodeOrExpression};

use super::strategy::Strategy;

// assume that it has been analysed
fn simplify_equation(equation: &mut Equation, _: &str) {
    for side_element in &mut equation.eq_sides {
        side_element.apply_to_every_element_mut(
            &mut |element| match &mut element.node_or_expression {
                NodeOrExpression::Node(node) => (),
                NodeOrExpression::Expression(expression) => {
                    // assume already multiplied, because bottom-up
                    for product in &mut expression.products {
                        let rationalized = product.rationalize();
                    }
                }
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
