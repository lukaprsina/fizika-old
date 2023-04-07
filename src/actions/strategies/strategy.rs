use std::fmt::Debug;

use crate::{
    actions::strategies,
    ast::{app::App, Equation},
};

pub struct Strategy {
    pub equation: Option<Box<dyn FnMut(&mut Equation, &str) -> ()>>,
}

impl Debug for Strategy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Strategy").finish()
    }
}

impl App {
    pub fn add_strategies(&mut self) {
        self.strategies.extend(vec![
            strategies::simplify::get_simplify(),
            strategies::solve_one_variable::get_solve_one_variable(),
        ]);
    }
}
