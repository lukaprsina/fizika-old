use crate::ast::{
    analyzed_expression::AnalyzedElement, product::Product, Element, Equation, Expression, Node,
    NodeOrExpression,
};

use super::strategy::Strategy;

fn simplify_equation(equation: &mut Equation) {}

fn simplify_analyzed_element(analyzed_element: &mut AnalyzedElement) {}

fn simplify_element(element: &mut Element) {}

fn simplify_node_or_expression(node_or_expression: &mut NodeOrExpression) {}

fn simplify_node(node: &mut Node) {}

fn simplify_expression(expression: &mut Expression) {}

fn simplify_product(product: &mut Product) {}

pub fn get_simplify() -> Strategy {
    Strategy {
        equation: Box::new(simplify_equation),
        analyzed_element: Box::new(simplify_analyzed_element),
        element: Box::new(simplify_element),
        node_or_expression: Box::new(simplify_node_or_expression),
        node: Box::new(simplify_node),
        expression: Box::new(simplify_expression),
        product: Box::new(simplify_product),
    }
}
