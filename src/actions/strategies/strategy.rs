use crate::{
    actions::strategies,
    ast::{
        analyzed_expression::AnalyzedElement, app::App, product::Product, Element, Equation,
        Expression, Node, NodeOrExpression,
    },
};

#[derive(Debug)]
pub struct Strategy {
    pub equation: Box<dyn FnMut(&mut Equation) -> ()>,
    pub analyzed_element: Box<dyn FnMut(&mut AnalyzedElement) -> ()>,
    pub element: Box<dyn FnMut(&mut Element) -> ()>,
    pub node_or_expression: Box<dyn FnMut(&mut NodeOrExpression) -> ()>,
    pub node: Box<dyn FnMut(&mut Node) -> ()>,
    pub expression: Box<dyn FnMut(&mut Expression) -> ()>,
    pub product: Box<dyn FnMut(&mut Product) -> ()>,
}

impl App {
    pub fn add_strategies(&mut self) {
        self.strategies
            .extend(vec![strategies::simplify::get_simplify()]);
    }
}
