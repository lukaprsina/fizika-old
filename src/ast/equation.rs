use std::{cell::RefCell, rc::Rc};

use uuid::Uuid;

use crate::tokenizer::{parser::TokenizedString, Operation};

use super::{app::App, context::CreateEquationError, Element, NodeOrExpression};

#[derive(Debug, Clone)]
pub struct EquationCache {}

#[derive(Debug, Clone)]
pub struct Equation {
    pub sides: Vec<Element>,
    pub app: Rc<RefCell<App>>,
    pub context: Uuid,
    pub cache: Option<EquationCache>,
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
    pub fn new(elements: Vec<Element>, app: Rc<RefCell<App>>, ctx_uuid: Uuid) -> Self {
        let equation = Equation {
            sides: elements,
            app: Rc::clone(&app),
            context: ctx_uuid,
            cache: Some(EquationCache {}),
        };

        // info!("{}", equation);
        equation.flatten()
        // println!("{:#?}", equation);
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
    // #[tracing::instrument(skip_all)]
    pub fn flatten(self) -> Equation {
        let new_equation = Equation::new(vec![], Rc::clone(&self.app), self.context);

        // ANATODO
        for element in self.sides {
            match element.node_or_expression {
                NodeOrExpression::Expression(expression) => {
                    let mut new_expr = expression.flatten();
                }
                NodeOrExpression::Node(_) => {}
            }
        }

        new_equation
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
