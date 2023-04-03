#![allow(unused_variables)]

use crate::ast::{Element, Equation, Node, NodeOrExpression};

use super::strategy::Strategy;

// imply that it has been analysed
fn simplify_equation(equation: &mut Equation, variable_name: &str) {
    if equation.eq_sides.len() != 2 {
        return;
    }

    for side_element in &mut equation.eq_sides {
        match &mut side_element.cache {
            Some(cache) => {
                if cache.variables.len() > 1 {
                    break;
                }

                // get to variable
                // keep track of the operations
                // then do the inverse
                build_stack(side_element, variable_name);
            }
            None => panic!("Equation has not been analyzed, cannot simplify"),
        }
    }
}

pub fn get_simplify() -> Strategy {
    Strategy {
        equation: Some(Box::new(simplify_equation)),
    }
}

fn build_stack(side: &Element, variable_name: &str) -> Vec<Element> {
    let mut stack = vec![];

    let mut element = side;

    loop {
        match &element.node_or_expression {
            NodeOrExpression::Node(node) => {
                // stack.push(node.clone());
            }
            NodeOrExpression::Expression(expression) => {
                for product in &expression.products {
                    for product_elem in &product.numerator {
                        if let Some(cache) = &product_elem.cache {
                            if cache.variables.contains(variable_name) {
                                stack.push(element.clone());
                                element = &product_elem;
                            }
                            // here
                        }
                    }
                }
            }
        }
    }

    stack
}
