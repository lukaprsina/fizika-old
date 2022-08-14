use std::{cell::RefCell, collections::HashMap, fmt::Debug, rc::Rc};

use tracing::info;
use uuid::Uuid;

use crate::{actions::strategies::strategy::Strategy, tokenizer::parser::ParseError};

use super::{
    context::{Context, CreateEquationError},
    equation::NoContextEquation,
    Equation,
};

#[derive(Debug)]
pub struct App {
    pub formulas: Uuid,
    pub contexts: HashMap<Uuid, Context>,
    pub strategies: Vec<Strategy>,
}

impl App {
    pub fn new() -> Result<Rc<RefCell<App>>, CreateEquationError> {
        let app = Rc::new(RefCell::new(App {
            formulas: Uuid::nil(),
            contexts: HashMap::new(),
            strategies: vec![],
        }));

        let ctx_uuid = {
            let mut borrowed_app = app.borrow_mut();

            let context = Context::new(Rc::clone(&app));
            borrowed_app.formulas = borrowed_app.add_context(context);
            borrowed_app.formulas
        };

        for line in include_str!("../../formulas.txt")
            .lines()
            .filter_map(|line| {
                let new_line = line.trim();
                if new_line.is_empty() {
                    None
                } else {
                    Some(new_line)
                }
            })
        {
            info!("\n\nNew formula: {}", line);

            if let Some(eq_err) = App::try_add_equation(Rc::clone(&app), ctx_uuid, line).err() {
                let mut throw = true;

                if let CreateEquationError::ParseError(parse_err) = &eq_err {
                    if let ParseError::Empty = parse_err {
                        throw = false;
                    }
                }

                if throw {
                    return Err(eq_err);
                }
            }
        }

        Ok(app)
    }

    pub fn add_context(&mut self, mut context: Context) -> Uuid {
        let uuid = Uuid::new_v4();
        context.uuid = uuid;
        self.contexts.insert(uuid, context);
        uuid
    }

    pub fn get_context(&self, uuid: Uuid) -> Option<&Context> {
        self.contexts.get(&uuid)
    }

    pub fn get_context_mut(&mut self, uuid: Uuid) -> Option<&mut Context> {
        self.contexts.get_mut(&uuid)
    }

    pub fn try_add_equation<T: Debug + TryInto<NoContextEquation, Error = CreateEquationError>>(
        app: Rc<RefCell<App>>,
        ctx_uuid: Uuid,
        input: T,
    ) -> Result<Equation, CreateEquationError> {
        let no_ctx_equation: NoContextEquation = input.try_into()?;
        let equation = App::add_equation(Rc::clone(&app), ctx_uuid, no_ctx_equation);

        Ok(equation)
    }

    pub fn add_equation<T: Into<NoContextEquation>>(
        app: Rc<RefCell<App>>,
        ctx_uuid: Uuid,
        input: T,
    ) -> Equation {
        let equation: NoContextEquation = input.into();

        let mut uuids: Vec<Uuid> = Vec::new();

        {
            let mut borrowed_app = app.borrow_mut();
            for side in equation.sides {
                let uuid = Uuid::new_v4();

                let element = side.analyze(borrowed_app.get_context(ctx_uuid).unwrap());
                let ctx = borrowed_app.get_context_mut(ctx_uuid).unwrap();
                ctx.elements.insert(uuid, element);

                uuids.push(uuid);
            }
        }

        let equation = Equation::new(uuids, Rc::clone(&app), ctx_uuid);

        equation
    }
}
