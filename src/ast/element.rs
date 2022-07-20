use super::{product::Product, Expression, Node};
use std::ops::{Add, Div, Mul, Neg, Sub};

pub(crate) trait ShouldBeParenthesized {
    fn should_be_parenthesized(&self) -> bool;
}

pub(crate) trait IsTimesVisible {
    fn is_times_visible(&self, last: &Element) -> bool;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Sign {
    Positive,
    Negative,
}

impl Mul for Sign {
    type Output = Sign;

    fn mul(self, rhs: Self) -> Self::Output {
        if self == rhs {
            Sign::Positive
        } else {
            Sign::Negative
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum NodeOrExpression {
    Node(Node),
    Expression(Expression),
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Element {
    pub sign: Sign,
    pub node_or_expression: NodeOrExpression,
    pub is_number: bool,
}

impl Element {
    pub fn new(sign: Sign, node_or_expression: NodeOrExpression) -> Self {
        Self {
            sign,
            node_or_expression,
            is_number: false,
        }
    }

    pub fn invert_sign(&mut self) {
        match self.sign {
            Sign::Positive => self.sign = Sign::Negative,
            Sign::Negative => self.sign = Sign::Positive,
        }
    }
}

impl IsTimesVisible for Element {
    fn is_times_visible(&self, last: &Element) -> bool {
        match &self.node_or_expression {
            NodeOrExpression::Node(node) => node.is_times_visible(last),
            NodeOrExpression::Expression(expression) => expression.is_times_visible(last),
        }
    }
}

impl ShouldBeParenthesized for Element {
    fn should_be_parenthesized(&self) -> bool {
        match &self.node_or_expression {
            NodeOrExpression::Node(node) => node.should_be_parenthesized(),
            NodeOrExpression::Expression(expression) => expression.should_be_parenthesized(),
        }
    }
}

impl Add for Element {
    type Output = Element;
    fn add(self, other: Element) -> Self::Output {
        let result = Expression::new(vec![
            Product {
                numerator: vec![self],
                denominator: vec![],
            },
            Product {
                numerator: vec![other],
                denominator: vec![],
            },
        ]);

        Element::new(Sign::Positive, NodeOrExpression::Expression(result))
    }
}

impl Sub for Element {
    type Output = Element;
    fn sub(self, mut other: Element) -> Self::Output {
        other.invert_sign();
        let result = Expression::new(vec![
            Product {
                numerator: vec![self],
                denominator: vec![],
            },
            Product {
                numerator: vec![other],
                denominator: vec![],
            },
        ]);

        Element::new(Sign::Positive, NodeOrExpression::Expression(result))
    }
}

impl Mul for Element {
    type Output = Element;
    fn mul(self, other: Element) -> Self::Output {
        let result = Expression::new(vec![Product {
            numerator: vec![self, other],
            denominator: vec![],
        }]);

        Element::new(Sign::Positive, NodeOrExpression::Expression(result))
    }
}

impl Div for Element {
    type Output = Element;
    fn div(self, other: Element) -> Self::Output {
        let result = Expression::new(vec![Product {
            numerator: vec![self],
            denominator: vec![other],
        }]);

        Element::new(Sign::Positive, NodeOrExpression::Expression(result))
    }
}

impl Neg for Element {
    type Output = Element;
    fn neg(mut self) -> Self::Output {
        self.invert_sign();
        self
    }
}

impl Mul<Sign> for Element {
    type Output = Element;

    fn mul(mut self, rhs: Sign) -> Self::Output {
        self.sign = self.sign * rhs;
        self
    }
}
