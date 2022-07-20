use std::{cell::RefCell, rc::Rc};

use uuid::Uuid;

use crate::tokenizer::{parser::TokenizedString, Operation};

use super::{
    context::{Context, CreateEquationError},
    Element, NodeOrExpression,
};

#[derive(Debug, Clone)]
pub struct Equation {
    pub uuids: Vec<Uuid>,
    pub context: Rc<RefCell<Context>>,
}

pub struct NoContextEquation {
    pub sides: Vec<EquationSide>,
}

#[derive(Debug, Clone)]
pub struct EquationSide {
    pub element: Element,
    pub operation: Option<Operation>,
}

impl Equation {
    pub fn new(uuids: Vec<Uuid>, context: Rc<RefCell<Context>>) -> Self {
        let mut equation = Equation {
            uuids,
            context: Rc::clone(&context),
        };

        equation.flatten(&mut context.borrow_mut());

        equation
    }
}

impl EquationSide {
    pub fn new(element: Element, operation: Option<Operation>) -> Self {
        Self { element, operation }
    }
}

impl NoContextEquation {
    pub fn new(sides: Vec<EquationSide>) -> Self {
        NoContextEquation { sides }
    }
}

impl Equation {
    pub fn flatten(&mut self, context: &mut Context) {
        for &mut uuid in self.uuids.iter_mut() {
            let expression = context.get_expression_mut(uuid).unwrap();
            if let NodeOrExpression::Expression(expression) =
                &mut expression.element.node_or_expression
            {
                expression.flatten();
            }
        }
    }
}

impl TryFrom<&str> for NoContextEquation {
    type Error = CreateEquationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let tokens = TokenizedString::try_from(value).map_err(CreateEquationError::ParseError)?;

        let ast = NoContextEquation::try_from(tokens)
            .map_err(CreateEquationError::TokensToEquationError)?;

        Ok(ast)
    }
}
