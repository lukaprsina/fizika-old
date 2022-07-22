use std::{cell::RefCell, collections::HashMap, fmt::Debug, rc::Rc};

use thiserror::Error;
use uuid::Uuid;

use crate::{ast::analyzed_expression::AnalyzedElement, tokenizer::parser::ParseError};

use super::{
    app::App, equation::NoContextEquation, token_to_element::TokensToEquationError, Equation,
};

#[derive(Debug, Clone)]
pub struct Context {
    pub app: Rc<RefCell<App>>,
    pub elements: HashMap<Uuid, AnalyzedElement>,
    pub uuid: Uuid,
}

#[derive(Debug, Error)]
pub enum CreateEquationError {
    #[error("{0}")]
    ParseError(ParseError),
    #[error("{0}")]
    TokensToEquationError(TokensToEquationError),
}

impl Context {
    pub fn new(app: Rc<RefCell<App>>) -> Context {
        Context {
            elements: HashMap::new(),
            app,
            uuid: Uuid::nil(),
        }
    }

    pub fn get_expression(&self, uuid: Uuid) -> Option<&AnalyzedElement> {
        self.elements.get(&uuid)
    }

    pub fn get_expression_mut(&mut self, uuid: Uuid) -> Option<&mut AnalyzedElement> {
        self.elements.get_mut(&uuid)
    }

    pub fn try_add_equation<T: Debug + TryInto<NoContextEquation, Error = CreateEquationError>>(
        &mut self,
        input: T,
    ) -> Result<Equation, CreateEquationError> {
        let equation: NoContextEquation = input.try_into()?;
        Ok(Context::add_equation(self, equation))
    }

    pub fn add_equation<T: Into<NoContextEquation>>(&mut self, input: T) -> Equation {
        let equation: NoContextEquation = input.into();

        let mut uuids: Vec<Uuid> = Vec::new();

        for side in equation.sides {
            let uuid = Uuid::new_v4();
            let element = side.analyze(&self);

            self.elements.insert(uuid, element);

            uuids.push(uuid);
        }

        Equation::new(uuids, Rc::clone(&self.app), self.uuid)
    }

    pub fn solve(&mut self) {
        println!("Context:");
        for (uuid, analyzed_element) in self.elements.iter() {
            println!(
                "{}: {}\nIs number?: {}\n{:#?}\n\n",
                uuid, analyzed_element.element, analyzed_element.is_number, analyzed_element.info
            );
        }
    }
}
