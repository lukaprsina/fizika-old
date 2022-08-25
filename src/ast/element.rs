use super::{product::Product, Expression, Node};
use std::ops::Mul;

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

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub enum NodeOrExpression {
    Node(Node),
    Expression(Expression),
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
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

    pub fn apply_to_every_element<T: FnMut(&Element)>(&self, mut function: T) {
        function(&self);

        if let NodeOrExpression::Node(node) = &self.node_or_expression {
            match &node {
                Node::Power { base, power } => {
                    function(&base);
                    function(&power);
                }
                Node::Modulo { lhs, rhs } => {
                    function(&lhs);
                    function(&rhs);
                }
                Node::Factorial { child } => {
                    function(&child);
                }
                Node::Function { name: _, arguments } => {
                    for argument in arguments.iter() {
                        function(&argument);
                    }
                }
                _ => (),
            }
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

impl Element {
    pub fn simple_add(lhs: Element, rhs: Element) -> Element {
        let result = Expression::new(vec![
            Product {
                numerator: vec![lhs],
                denominator: vec![],
            },
            Product {
                numerator: vec![rhs],
                denominator: vec![],
            },
        ]);

        Element::new(Sign::Positive, NodeOrExpression::Expression(result))
    }

    pub fn simple_sub(lhs: Element, mut rhs: Element) -> Element {
        rhs.invert_sign();
        let result = Expression::new(vec![
            Product {
                numerator: vec![lhs],
                denominator: vec![],
            },
            Product {
                numerator: vec![rhs],
                denominator: vec![],
            },
        ]);

        Element::new(Sign::Positive, NodeOrExpression::Expression(result))
    }

    pub fn simple_mul(lhs: Element, rhs: Element) -> Element {
        let result = Expression::new(vec![Product {
            numerator: vec![lhs, rhs],
            denominator: vec![],
        }]);

        Element::new(Sign::Positive, NodeOrExpression::Expression(result))
    }

    pub fn simple_div(lhs: Element, rhs: Element) -> Element {
        let result = Expression::new(vec![Product {
            numerator: vec![lhs],
            denominator: vec![rhs],
        }]);

        Element::new(Sign::Positive, NodeOrExpression::Expression(result))
    }

    pub fn simple_neg(mut self) -> Element {
        self.invert_sign();
        self
    }

    pub fn simple_mul_sign(mut self, sign: Sign) -> Element {
        self.sign = self.sign * sign;
        self
    }
}
