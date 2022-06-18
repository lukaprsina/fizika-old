use std::ops::{Add, Div, Mul, Neg, Sub};

use crate::ast::Node;

pub(crate) trait ShouldBeParenthesized {
    fn should_be_parenthesized(&self) -> bool;
}

pub(crate) trait IsTimesVisible {
    fn is_times_visible(&self, last: &Element) -> bool;
}

#[derive(Debug, Clone)]
pub enum NodeOrExpression {
    Node(Node),
    Expression(Expression),
}

#[derive(Debug, Clone)]
pub struct Element {
    pub sign: Sign,
    pub node_or_expression: NodeOrExpression,
}

impl Element {
    pub fn new(sign: Sign, node_or_expression: NodeOrExpression) -> Self {
        Self {
            sign,
            node_or_expression,
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

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Sign {
    Positive,
    Negative,
}

#[derive(Debug, Clone)]
pub struct Product {
    pub numerator: Vec<Element>,
    pub denominator: Vec<Element>,
}

impl Product {
    // TODO: don't accept empty products
    pub fn new(numerator: Vec<Element>, denominator: Vec<Element>) -> Product {
        Product {
            numerator,
            denominator,
        }
    }

    pub fn get_sign(&self) -> Sign {
        match self.numerator.first() {
            Some(first) => first.sign,
            None => match self.denominator.first() {
                Some(first) => first.sign,
                None => unreachable!("Empty products are not allowed"),
            },
        }
    }
}

impl IsTimesVisible for Product {
    fn is_times_visible(&self, last: &Element) -> bool {
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
    fn is_times_visible(&self, last: &Element) -> bool {
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

impl Add for Element {
    type Output = Element;
    fn add(self, other: Element) -> Self::Output {
        let mut result = Expression::new();

        result.products.push(Product {
            numerator: vec![self],
            denominator: vec![],
        });
        result.products.push(Product {
            numerator: vec![other],
            denominator: vec![],
        });

        Element::new(Sign::Positive, NodeOrExpression::Expression(result))
    }
}

impl Sub for Element {
    type Output = Element;
    fn sub(self, mut other: Element) -> Self::Output {
        let mut result = Expression::new();

        result.products.push(Product {
            numerator: vec![self],
            denominator: vec![],
        });

        other.invert_sign();

        result.products.push(Product {
            numerator: vec![other],
            denominator: vec![],
        });

        Element::new(Sign::Positive, NodeOrExpression::Expression(result))
    }
}

impl Mul for Element {
    type Output = Element;
    fn mul(self, other: Element) -> Self::Output {
        let mut result = Expression::new();

        result.products.push(Product {
            numerator: vec![self, other],
            denominator: vec![],
        });

        Element::new(Sign::Positive, NodeOrExpression::Expression(result))
    }
}

impl Div for Element {
    type Output = Element;
    fn div(self, other: Element) -> Self::Output {
        let mut result = Expression::new();

        result.products.push(Product {
            numerator: vec![self],
            denominator: vec![other],
        });

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
