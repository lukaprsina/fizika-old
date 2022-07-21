use std::{cell::RefCell, rc::Rc};

use super::context::{Context, CreateEquationError};

pub struct App {
    pub formulas: Rc<RefCell<Context>>,
    pub contexts: Vec<Rc<RefCell<Context>>>,
}

impl App {
    pub fn new() -> Result<App, CreateEquationError> {
        let formulas = Context::new();

        for line in include_str!("../../formulas.txt").lines() {
            Context::try_add_equation(Rc::clone(&formulas), line)?;
        }

        Ok(App {
            formulas,
            contexts: vec![],
        })
    }

    pub fn add_context() {}
}
