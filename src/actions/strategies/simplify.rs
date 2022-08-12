use crate::ast::{
    analyzed_expression::AnalyzedElement, product::Product, Element, Equation, Expression, Node,
    NodeOrExpression,
};

use super::strategy::Strategy;

fn simplify_equation(_equation: &mut Equation) {}

fn simplify_analyzed_element(_analyzed_element: &mut AnalyzedElement) {}

fn simplify_element(_element: &mut Element) {}

fn simplify_node_or_expression(_node_or_expression: &mut NodeOrExpression) {}

fn simplify_node(_node: &mut Node) {}

fn simplify_expression(_expression: &mut Expression) {}

fn simplify_product(_product: &mut Product) {}

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
