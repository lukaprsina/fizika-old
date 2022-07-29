use std::{cell::RefCell, collections::HashMap, fmt::Debug, rc::Rc};

use uuid::Uuid;

use crate::actions::strategies::strategy::Strategy;

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

        for line in include_str!("../../formulas.txt").lines() {
            App::try_add_equation(Rc::clone(&app), ctx_uuid, line)?;
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
        let equation: NoContextEquation = input.try_into()?;
        Ok(App::add_equation(Rc::clone(&app), ctx_uuid, equation))
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
                borrowed_app
                    .get_context_mut(ctx_uuid)
                    .unwrap()
                    .elements
                    .insert(uuid, element);

                uuids.push(uuid);
            }
        }

        let equation = Equation::new(uuids, Rc::clone(&app), ctx_uuid);

        equation
    }
}
