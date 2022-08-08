use std::{cell::RefCell, rc::Rc};

use tracing::info;
use uuid::Uuid;

use crate::tokenizer::{parser::TokenizedString, Operation};

use super::{
    app::App,
    context::{Context, CreateEquationError},
    Element, NodeOrExpression,
};

#[derive(Debug, Clone)]
pub struct Equation {
    pub uuids: Vec<Uuid>,
    pub app: Rc<RefCell<App>>,
    pub context: Uuid,
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
    pub fn new(uuids: Vec<Uuid>, app: Rc<RefCell<App>>, ctx_uuid: Uuid) -> Self {
        let mut equation = Equation {
            uuids,
            app: Rc::clone(&app),
            context: ctx_uuid,
        };

        let mut borrowed_app = app.borrow_mut();
        let context = borrowed_app.get_context_mut(ctx_uuid).unwrap();

        equation.flatten(context);

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
    #[tracing::instrument(skip_all)]
    pub fn flatten(self, context: &mut Context) {
        for uuid in self.uuids.into_iter() {
            let mut expression = context.elements.remove(&uuid).unwrap();

            if let NodeOrExpression::Expression(expression) = expression.element.node_or_expression
            {
                info!("Before flatten {:#?}", expression);
                expression.flatten();
                // info!("After flatten: {}", expression);
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
