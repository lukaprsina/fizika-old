use std::sync::Arc;

use uuid::Uuid;

use crate::tokenizer::{parser::TokenizedString, Operation};

use super::{
    analyzed_equation::AnalyzedElement,
    context::{Context, CreateEquationError},
    Element, NodeOrExpression,
};

#[derive(Debug, Clone)]
pub struct Equation<'a> {
    pub uuids: Vec<Uuid>,
    context: Arc<&'a mut Context>,
}

pub struct NoContextEquation {
    pub sides: Vec<EquationSide>,
}

#[derive(Debug, Clone)]
pub struct EquationSide {
    pub element: Element,
    pub operation: Option<Operation>,
}

impl<'a> Equation<'a> {
    pub fn new(uuids: Vec<Uuid>, context: Arc<&'a mut Context>) -> Self {
        Equation {
            uuids,
            context: Arc::clone(&context),
        }
    }

    pub fn sides(&'a self) -> impl Iterator<Item = &AnalyzedElement> {
        self.uuids
            .iter()
            .map_while(|&uuid| self.context.get_expression(uuid))
    }

    /* pub fn sides_mut<'b>(&'a mut self) -> impl Iterator<Item = &mut AnalyzedElement> {
        self.uuids
            .iter_mut()
            .map_while(|&mut uuid| self.context.get_expression_mut(uuid))
    } */
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

impl<'a> Equation<'a> {
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
        let tokens =
            TokenizedString::try_from(value).map_err(|err| CreateEquationError::ParseError(err))?;

        let ast = NoContextEquation::try_from(tokens)
            .map_err(|err| CreateEquationError::TokensToEquationError(err))?;

        Ok(ast)
    }
}
