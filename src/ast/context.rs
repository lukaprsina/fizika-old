use std::{cell::RefCell, collections::HashMap, fmt::Debug, rc::Rc};

use uuid::Uuid;

use crate::{ast::analyzed_expression::AnalyzedElement, tokenizer::parser::ParseError};

use super::{equation::NoContextEquation, token_to_element::TokensToEquationError, Equation};

#[derive(Debug, Clone)]
pub struct Context {
    pub elements: HashMap<Uuid, AnalyzedElement>,
}

#[derive(Debug)]
pub enum CreateEquationError {
    ParseError(ParseError),
    TokensToEquationError(TokensToEquationError),
}

impl Context {
    pub fn new() -> Rc<RefCell<Context>> {
        Rc::new(RefCell::new(Context {
            elements: HashMap::new(),
        }))
    }

    pub fn get_expression(&self, uuid: Uuid) -> Option<&AnalyzedElement> {
        self.elements.get(&uuid)
    }

    pub fn get_expression_mut(&mut self, uuid: Uuid) -> Option<&mut AnalyzedElement> {
        self.elements.get_mut(&uuid)
    }

    pub fn try_add_equation<T: Debug + TryInto<NoContextEquation, Error = CreateEquationError>>(
        context: Rc<RefCell<Context>>,
        input: T,
    ) -> Result<Equation, CreateEquationError> {
        let equation: NoContextEquation = input.try_into()?;
        Ok(Context::add_equation(context, equation))
    }

    pub fn add_equation<T: Into<NoContextEquation>>(
        context: Rc<RefCell<Context>>,
        input: T,
    ) -> Equation {
        let equation: NoContextEquation = input.into();

        let mut uuids: Vec<Uuid> = Vec::new();

        for side in equation.sides {
            let uuid = Uuid::new_v4();
            let element = side.analyze(&context.borrow());

            context.borrow_mut().elements.insert(uuid, element);

            uuids.push(uuid);
        }

        Equation::new(uuids, Rc::clone(&context))
    }

    pub fn solve(&self) {
        println!("Context:");
        for (uuid, analyzed_element) in self.elements.iter() {
            println!(
                "{}: {}\nIs number?: {}\n{:#?}\n\n",
                uuid, analyzed_element.element, analyzed_element.is_number, analyzed_element.info
            );
        }
    }
}
