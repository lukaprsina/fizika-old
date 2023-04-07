use crate::ast::{Equation, Node, NodeOrExpression};

use super::strategy::Strategy;

// imply that it has been analysed
fn simplify_equation(equation: &mut Equation, _: &str) {
    for side_element in &mut equation.eq_sides {
        side_element.apply_to_every_element_mut(
            &mut |element| match &mut element.node_or_expression {
                NodeOrExpression::Node(node) => match node {
                    Node::Number(_) => todo!(),
                    Node::Variable(_) => todo!(),
                    Node::Power { base, power } => {
                        // assume already multiplied, because bottom-up
                    }
                    Node::Modulo { lhs, rhs } => todo!(),
                    Node::Factorial { child } => todo!(),
                    Node::Function { name, arguments } => todo!(),
                },
                NodeOrExpression::Expression(expression) => todo!(),
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
