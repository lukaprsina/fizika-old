use std::{cell::RefCell, rc::Rc};

use uuid::Uuid;

use crate::{
    ast::analyzed_expression::{Analyze, ExpressionInfo},
    tokenizer::{parser::TokenizedString, Operation},
};

use super::{
    app::App,
    context::{Context, CreateEquationError},
    Element, NodeOrExpression, Sign,
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
        let equation = Equation {
            uuids,
            app: Rc::clone(&app),
            context: ctx_uuid,
        };

        // info!("{}", equation);
        {
            let mut borrowed_app = app.borrow_mut();
            let context = borrowed_app.get_context_mut(ctx_uuid).unwrap();

            equation.flatten(context);
        }
        // println!("{:#?}", equation);

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
    // #[tracing::instrument(skip_all)]
    pub fn flatten(&self, context: &mut Context) {
        for &uuid in self.uuids.iter() {
            let analyzed_element = context.elements.remove(&uuid).unwrap();

            // ANATODO
            /* if let NodeOrExpression::Expression(expression) =
                analyzed_element.element.node_or_expression
            {
                // info!("Before flatten {}", expression);
                // println!("{:#?}", expression);
                let mut new_expr = expression.flatten();

                // TODO: after remove analyzed element fix this sign
                if analyzed_element.element.sign == Sign::Negative {}
                // info!("After flatten: {}", new_expr);
                // println!("{:#?}", new_expr);

                let mut info = ExpressionInfo::default();
                let mut is_number = false;

                new_expr.analyze(context, &mut info, &mut is_number);

                // ANATODO
                context.elements.insert(
                    uuid,
                    Element {
                        sign: Sign::Positive,
                        node_or_expression: NodeOrExpression::Expression(new_expr),
                        is_number,
                    },
                );
            } else {
                context.elements.insert(uuid, analyzed_element);
            } */
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
