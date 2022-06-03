use std::ops::{Add, Div, Mul, Neg, Sub};

use crate::ast::Node;

use super::token_to_node::match_binary;
pub(crate) trait ShouldBeParenthesized {
    fn should_be_parenthesized(&self) -> bool;
}

pub(crate) trait IsTimesVisible {
    fn is_times_visible(&self, last: &NodeOrExpression) -> bool;
}

#[derive(Debug, Clone)]
pub enum NodeOrExpression {
    Node(Node),
    Expression(Expression),
}

impl IsTimesVisible for NodeOrExpression {
    fn is_times_visible(&self, last: &NodeOrExpression) -> bool {
        match self {
            NodeOrExpression::Node(node) => node.is_times_visible(last),
            NodeOrExpression::Expression(expression) => expression.is_times_visible(last),
        }
    }
}

impl ShouldBeParenthesized for NodeOrExpression {
    fn should_be_parenthesized(&self) -> bool {
        match self {
            NodeOrExpression::Node(node) => node.should_be_parenthesized(),
            NodeOrExpression::Expression(expression) => expression.should_be_parenthesized(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Sign {
    Positive,
    Negative,
}

#[derive(Debug, Clone)]
pub struct Product {
    pub sign: Sign,
    pub numerator: Vec<NodeOrExpression>,
    pub denominator: Vec<NodeOrExpression>,
}

impl Product {
    pub fn new(
        sign: Sign,
        numerator: Vec<NodeOrExpression>,
        denominator: Vec<NodeOrExpression>,
    ) -> Product {
        Product {
            sign,
            numerator,
            denominator,
        }
    }
}

impl IsTimesVisible for Product {
    fn is_times_visible(&self, last: &NodeOrExpression) -> bool {
        if !self.numerator.is_empty() {
            self.numerator[0].is_times_visible(last)
        } else {
            true
        }
    }
}

impl ShouldBeParenthesized for Product {
    fn should_be_parenthesized(&self) -> bool {
        if self.numerator.len() == 1 {
            self.numerator[0].should_be_parenthesized()
        } else {
            true
        }
    }
}

#[derive(Debug, Clone)]
pub struct Expression {
    pub products: Vec<Product>,
}

impl Expression {
    pub fn new() -> Self {
        Expression {
            products: Vec::new(),
        }
    }
}

impl Default for Expression {
    fn default() -> Self {
        Self::new()
    }
}

impl IsTimesVisible for Expression {
    fn is_times_visible(&self, last: &NodeOrExpression) -> bool {
        if !self.products.is_empty() {
            self.products[0].is_times_visible(last)
        } else {
            true
        }
    }
}

impl ShouldBeParenthesized for Expression {
    fn should_be_parenthesized(&self) -> bool {
        if self.products.len() == 1 {
            self.products[0].should_be_parenthesized()
        } else {
            true
        }
    }
}

impl Add for NodeOrExpression {
    type Output = NodeOrExpression;
    fn add(self, other: NodeOrExpression) -> NodeOrExpression {
        match_binary(
            self,
            other,
            |lhs: NodeOrExpression, rhs: NodeOrExpression| -> NodeOrExpression {
                let mut result = Expression::new();
                result
                    .products
                    .push(Product::new(Sign::Positive, vec![lhs], vec![]));
                result
                    .products
                    .push(Product::new(Sign::Positive, vec![rhs], vec![]));
                NodeOrExpression::Expression(result)
            },
        )
    }
}

impl Sub for NodeOrExpression {
    type Output = NodeOrExpression;
    fn sub(self, other: NodeOrExpression) -> NodeOrExpression {
        match_binary(
            self,
            other,
            |lhs: NodeOrExpression, rhs: NodeOrExpression| -> NodeOrExpression {
                let mut result = Expression::new();
                result
                    .products
                    .push(Product::new(Sign::Positive, vec![lhs], vec![]));
                result
                    .products
                    .push(Product::new(Sign::Negative, vec![rhs], vec![]));
                NodeOrExpression::Expression(result)
            },
        )
    }
}

impl Mul for NodeOrExpression {
    type Output = NodeOrExpression;
    fn mul(self, other: NodeOrExpression) -> NodeOrExpression {
        match_binary(
            self,
            other,
            |lhs: NodeOrExpression, rhs: NodeOrExpression| -> NodeOrExpression {
                let mut result = Expression::new();
                result
                    .products
                    .push(Product::new(Sign::Positive, vec![lhs, rhs], vec![]));
                NodeOrExpression::Expression(result)
            },
        )
    }
}

impl Div for NodeOrExpression {
    type Output = NodeOrExpression;
    fn div(self, other: NodeOrExpression) -> NodeOrExpression {
        match_binary(
            self,
            other,
            |lhs: NodeOrExpression, rhs: NodeOrExpression| -> NodeOrExpression {
                let mut result = Expression::new();
                result
                    .products
                    .push(Product::new(Sign::Positive, vec![lhs], vec![rhs]));
                NodeOrExpression::Expression(result)
            },
        )
    }
}

impl Neg for NodeOrExpression {
    type Output = NodeOrExpression;
    fn neg(self) -> NodeOrExpression {
        let mut result = Expression::new();
        result
            .products
            .push(Product::new(Sign::Negative, vec![self], vec![]));
        NodeOrExpression::Expression(result)
    }
}
