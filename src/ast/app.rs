use std::{cell::RefCell, collections::HashMap, fmt::Debug, rc::Rc};

use uuid::Uuid;

use crate::actions::strategies::strategy::Strategy;

use super::{
    context::{Context, CreateEquationError},
    equation::NoContextEquation,
    Element, Equation,
};

const STRATEGY_ORDER: [&'static str; 2] = ["flatten", "simplify"];

#[derive(Debug)]
pub struct App {
    pub formulas: Uuid,
    pub contexts: HashMap<Uuid, Context>,
    pub strategies: HashMap<String, Strategy>,
}

impl App {
    pub fn new() -> Result<Rc<RefCell<App>>, CreateEquationError> {
        let mut app = App {
            formulas: Uuid::nil(),
            contexts: HashMap::new(),
            strategies: HashMap::new(),
        };

        app.add_strategies();

        let app = Rc::new(RefCell::new(app));

        /* let ctx_uuid = {
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
            // println!("\n\nNew formula: {}", line);

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
        } */

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

    pub fn solve(&mut self, context_uuid: Uuid) {
        // println!("Context {}", self.uuid);
        let context = self
            .get_context_mut(context_uuid)
            .expect("Context not found");

        for (uuid, equation) in &mut context.equations {
            let mut i = 0;
            loop {
                for element in &mut equation.equation_sides {
                    element.analyze(None);
                }

                let strategy = STRATEGY_ORDER[i % STRATEGY_ORDER.len()];
                context.apply_strategy(self, strategy, *uuid);
                // self.apply_strategy(strategy, *uuid, context.uuid);

                println!("{}", equation);
                i += 1;
            }
        }

        // println!("Analysis: {:#?}", analysis);
    }

    pub fn try_add_equation<T: Debug + TryInto<NoContextEquation, Error = CreateEquationError>>(
        app: Rc<RefCell<App>>,
        ctx_uuid: Uuid,
        input: T,
    ) -> Result<Uuid, CreateEquationError> {
        let no_ctx_equation: NoContextEquation = input.try_into()?;

        /* no_ctx_equation
        .sides
        .iter()
        .for_each(|side| println!("{:#?}", side.element)); */

        let equation = App::add_equation(Rc::clone(&app), ctx_uuid, no_ctx_equation);

        Ok(equation)
    }

    pub fn add_equation<T: Into<NoContextEquation>>(
        app: Rc<RefCell<App>>,
        ctx_uuid: Uuid,
        input: T,
    ) -> Uuid {
        let no_ctx_eq: NoContextEquation = input.into();

        let mut elements: Vec<Element> = Vec::new();

        for side in no_ctx_eq.sides {
            // info!("{}", side.element);
            // TODO: ignores operation
            elements.push(side.element);
        }

        let equation = Equation::new(elements, Rc::clone(&app), ctx_uuid);

        {
            let mut borrowed_app = app.borrow_mut();
            let ctx = borrowed_app.get_context_mut(ctx_uuid).unwrap();
            ctx.insert_equation(equation)
        }
    }
}
