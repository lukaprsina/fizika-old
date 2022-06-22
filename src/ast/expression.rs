use std::ops::{Add, Div, Mul, Neg, Sub};

use super::{
    element::{IsTimesVisible, ShouldBeParenthesized},
    product::Product,
    Element, NodeOrExpression, Sign,
};

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
