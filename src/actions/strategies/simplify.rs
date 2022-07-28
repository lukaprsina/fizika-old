use crate::ast::Equation;

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
        equation: simplify_equation,
        analyzed_element: simplify_analyzed_element,
        element: simplify_element,
        node_or_expression: simplify_node_or_expression,
        node: simplify_node,
        expression: simplify_expression,
        product: simplify_product,
    }
}
