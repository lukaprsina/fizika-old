use std::collections::HashMap;

use crate::tokenizer::{parser::TokenizedString, Operation};

use super::{context::CreateEquationError, Element, NodeOrExpression};

#[derive(Debug, Clone)]
pub struct Equation {
    pub sides: Vec<EquationSide>,
}

impl Equation {
    pub fn new(sides: Vec<EquationSide>) -> Self {
        Equation { sides }
    }
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
            if let NodeOrExpression::Expression(expression) = &mut side.element.node_or_expression {
                expression.flatten();
            }
        }
    }
}

impl TryFrom<&str> for Equation {
    type Error = CreateEquationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let tokens =
            TokenizedString::try_new(&value).map_err(|err| CreateEquationError::ParseError(err))?;

        let mut ast = Equation::try_from(tokens)
            .map_err(|err| CreateEquationError::TokensToEquationError(err))?;

        ast.flatten();

        Ok(ast)
    }
}
