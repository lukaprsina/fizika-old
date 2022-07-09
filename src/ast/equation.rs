use uuid::Uuid;

use crate::tokenizer::{parser::TokenizedString, Operation};

use super::{
    context::{Context, CreateEquationError},
    Element, NodeOrExpression,
};

#[derive(Debug, Clone)]
pub struct Equation {
    pub uuids: Vec<Uuid>,
}

impl Equation {
    pub fn new(uuids: Vec<Uuid>) -> Self {
        Equation { uuids }
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
    pub fn flatten(&mut self, context: &Context) {
        for uuid in self.uuids {
            if let NodeOrExpression::Expression(expression) =
                context.get_expression_mut(uuid).node_or_expression
            {
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
