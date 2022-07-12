use std::collections::HashMap;

use uuid::Uuid;

use crate::{ast::analyzed_equation::AnalyzedElement, tokenizer::parser::ParseError};

use super::{equation::NoContextEquation, token_to_element::TokensToEquationError};

#[derive(Debug, Clone)]
pub struct Context {
    pub expressions: HashMap<Uuid, AnalyzedElement>,
}

#[derive(Debug)]
pub enum CreateEquationError {
    ParseError(ParseError),
    TokensToEquationError(TokensToEquationError),
}

impl Context {
    pub fn new() -> Self {
        Context {
            expressions: HashMap::new(),
        }
    }

    pub fn get_expression(&self, uuid: Uuid) -> Option<&AnalyzedElement> {
        self.expressions.get(&uuid)
    }

    pub fn get_expression_mut(&mut self, uuid: Uuid) -> Option<&mut AnalyzedElement> {
        self.expressions.get_mut(&uuid)
    }

    pub fn try_add_equation<T>(&mut self, input: T) -> Result<Uuid, CreateEquationError>
    where
        T: TryInto<NoContextEquation, Error = CreateEquationError>,
    {
        let equation: NoContextEquation = input.try_into()?;
        Ok(self.add_equation(equation))
    }

    pub fn add_equation<T>(&mut self, input: T) -> Uuid
    where
        T: Into<NoContextEquation>,
    {
        let equation: NoContextEquation = input.into();

        let uuid = Uuid::new_v4();

        for side in equation.sides {
            self.expressions.insert(uuid, side.analyze(&self));
        }

        uuid
    }

    pub fn solve(&self) {}
}
