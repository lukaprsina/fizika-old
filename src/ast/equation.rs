use crate::{ast::NodeOrExpression, tokenizer::Operation};

use super::expression::Element;

#[derive(Debug, Clone)]
pub struct Equation {
    pub sides: Vec<EquationSide>,
}

#[derive(Debug, Clone)]
pub struct EquationSide {
    pub element: Element,
    pub operation: Option<Operation>,
}

impl EquationSide {
    pub fn new(element: Element, operation: Option<Operation>) -> Self {
        Self { element, operation }
    }
}

impl Equation {
    pub fn flatten(&mut self) {
        for side in self.sides.iter_mut() {
            if let NodeOrExpression::Expression(_expression) = &side.element.node_or_expression {
                // expression.flatten();
            }
        }
    }
}
