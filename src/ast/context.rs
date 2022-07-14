use std::{collections::HashMap, sync::Arc};

use uuid::Uuid;

use crate::{ast::analyzed_expression::AnalyzedElement, tokenizer::parser::ParseError};

use super::{equation::NoContextEquation, token_to_element::TokensToEquationError, Equation};

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

    pub fn try_add_equation<T>(&mut self, input: T) -> Result<Equation, CreateEquationError>
    where
        T: TryInto<NoContextEquation, Error = CreateEquationError>,
    {
        let equation: NoContextEquation = input.try_into()?;
        Ok(self.add_equation(equation))
    }

    pub fn add_equation<T: Into<NoContextEquation>>(&mut self, input: T) -> Equation {
        let equation: NoContextEquation = input.into();

        let mut uuids: Vec<Uuid> = Vec::new();

        for side in equation.sides {
            let uuid = Uuid::new_v4();
            self.expressions.insert(uuid, side.analyze(&self));
            uuids.push(uuid);
        }

        let arc = Arc::new(self);
        Equation::new(uuids, arc)
    }

    pub fn solve(&self) {
        for (uuid, analyzed_element) in self.expressions.iter() {
            println!(
                "{}: {}\nIs number?:{}\n{:#?}\n\n",
                uuid, analyzed_element.element, analyzed_element.is_number, analyzed_element.info
            );
        }
    }
}
