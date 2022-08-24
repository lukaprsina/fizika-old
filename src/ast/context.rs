use std::{cell::RefCell, collections::HashMap, fmt::Debug, rc::Rc};

use thiserror::Error;
use uuid::Uuid;

use crate::tokenizer::parser::ParseError;

use super::{app::App, token_to_element::TokensToEquationError, Equation};

#[derive(Debug, Clone)]
pub struct Context {
    pub app: Rc<RefCell<App>>,
    equations: HashMap<Uuid, Equation>,
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
            equations: HashMap::new(),
            app,
            uuid: Uuid::nil(),
        }
    }

    pub fn get_equation(&self, uuid: Uuid) -> Option<&Equation> {
        self.equations.get(&uuid)
    }

    pub fn get_equation_mut(&mut self, uuid: Uuid) -> Option<&mut Equation> {
        let mut equation = self.equations.get_mut(&uuid);

        if let Some(eq) = &mut equation {
            eq.cache = None;
        }

        equation
    }

    pub fn remove_equation(&mut self, uuid: Uuid) -> Option<Equation> {
        let mut equation = self.equations.remove(&uuid);

        if let Some(eq) = &mut equation {
            eq.cache = None;
        }

        equation
    }

    pub(crate) fn insert_equation(&mut self, equation: Equation) -> Uuid {
        let uuid = Uuid::new_v4();
        self.equations.insert(uuid, equation);
        uuid
    }

    pub fn solve(&mut self) {
        /* println!("Context:");
        for (uuid, analyzed_element) in self.elements.iter() {
            println!(
                "{}: {}\nIs number?: {}\n{:#?}\n\n",
                uuid, analyzed_element.element, analyzed_element.is_number, analyzed_element.info
            );
        } */
    }
}
