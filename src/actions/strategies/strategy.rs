use std::fmt::Debug;

use crate::{
    actions::strategies,
    ast::{app::App, product::Product, Element, Equation, Expression, Node, NodeOrExpression},
};

pub struct Strategy {
    pub equation: Box<dyn FnMut(&mut Equation) -> ()>,
    pub element: Box<dyn FnMut(&mut Element) -> ()>,
    pub node_or_expression: Box<dyn FnMut(&mut NodeOrExpression) -> ()>,
    pub node: Box<dyn FnMut(&mut Node) -> ()>,
    pub expression: Box<dyn FnMut(&mut Expression) -> ()>,
    pub product: Box<dyn FnMut(&mut Product) -> ()>,
}

impl Debug for Strategy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Strategy").finish()
    }
}

impl App {
    pub fn add_strategies(&mut self) {
        self.strategies
            .extend(vec![strategies::simplify::get_simplify()]);
    }
}
