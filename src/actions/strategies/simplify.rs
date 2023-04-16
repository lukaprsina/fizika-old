#![allow(unused_variables)]

use tracing::debug;

use crate::ast::{Element, Equation, Expression, Node, NodeOrExpression};

use super::strategy::Strategy;

// assume that it has been analysed
fn simplify_equation(equation: &mut Equation) {
    for side_element in &mut equation.equation_sides {
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

        side_element.analyze(None);

        side_element.apply_to_every_element_mut(
            &mut |element| {
                debug!("{element} {}", element.is_number());
                if let NodeOrExpression::Expression(expression) = &mut element.node_or_expression {
                    for product in &mut expression.products {
                        let mut delete_denominator = false;

                        for pr_elem in &mut product.denominator {
                            if let NodeOrExpression::Node(Node::Number(number)) =
                                &mut pr_elem.node_or_expression
                            {
                                if *number
                                    == num::BigInt::new(num::bigint::Sign::Plus, vec![1]).into()
                                {
                                    delete_denominator = true;
                                }
                            }
                        }

                        if delete_denominator {
                            product.denominator.clear();
                        }
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
