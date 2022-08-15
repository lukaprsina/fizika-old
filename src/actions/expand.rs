use std::ops::Mul;

use crate::ast::{Element, Expression, Node, NodeOrExpression, Sign};

impl Expression {
    pub fn expand(&mut self) {
        for product in self.products.iter_mut() {
            let new_element = Element::new(
                Sign::Positive,
                NodeOrExpression::Expression(Expression::new(vec![])),
            );

            for side in [&product.numerator, &product.denominator] {}
        }
    }
}

impl Mul for Element {
    type Output = Element;

    fn mul(self, rhs: Self) -> Self::Output {
        let lhs = self.simple_mul_sign(rhs.sign);

        match lhs.node_or_expression {
            NodeOrExpression::Node(lhs_node) => match rhs.node_or_expression {
                NodeOrExpression::Node(rhs_node) => mul_node_and_node(lhs_node, rhs_node),
                NodeOrExpression::Expression(rhs_expr) => mul_node_and_expr(lhs_node, rhs_expr),
            },
            NodeOrExpression::Expression(lhs_expr) => match rhs.node_or_expression {
                NodeOrExpression::Node(rhs_node) => mul_node_and_expr(rhs_node, lhs_expr),
                NodeOrExpression::Expression(rhs_expr) => mul_expr_and_expr(lhs_expr, rhs_expr),
            },
        }
    }
}

fn mul_node_and_node(lhs_node: Node, rhs_node: Node) -> Element {}

fn mul_node_and_expr(lhs_node: Node, rhs_expr: Expression) -> Element {}

fn mul_expr_and_expr(lhs_expr: Expression, rhs_expr: Expression) -> Element {}
