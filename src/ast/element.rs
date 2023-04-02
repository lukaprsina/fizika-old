use itertools::Itertools;

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

    pub fn apply_to_every_element(&self, function: &mut impl FnMut(&Element)) {
        function(self);

        match &self.node_or_expression {
            NodeOrExpression::Node(node) => match node {
                Node::Power { base, power } => {
                    base.apply_to_every_element(function);
                    power.apply_to_every_element(function);
                }
                Node::Modulo { lhs, rhs } => {
                    lhs.apply_to_every_element(function);
                    rhs.apply_to_every_element(function);
                }
                Node::Factorial { child } => {
                    child.apply_to_every_element(function);
                }
                Node::Function { name: _, arguments } => {
                    for argument in arguments.iter() {
                        argument.apply_to_every_element(function);
                    }
                }
                _ => (),
            },
            NodeOrExpression::Expression(expression) => {
                for product in &expression.products {
                    for side in [&product.numerator, &product.denominator] {
                        for element in side {
                            element.apply_to_every_element(function);
                        }
                    }
                }
            }
        }
    }

    pub fn apply_to_every_element_mut(&mut self, function: &mut impl FnMut(&mut Element)) {
        function(self);

        match &mut self.node_or_expression {
            NodeOrExpression::Node(node) => match node {
                Node::Power { base, power } => {
                    base.apply_to_every_element_mut(function);
                    power.apply_to_every_element_mut(function);
                }
                Node::Modulo { lhs, rhs } => {
                    lhs.apply_to_every_element_mut(function);
                    rhs.apply_to_every_element_mut(function);
                }
                Node::Factorial { child } => {
                    child.apply_to_every_element_mut(function);
                }
                Node::Function { name: _, arguments } => {
                    for argument in arguments.iter_mut() {
                        argument.apply_to_every_element_mut(function);
                    }
                }
                _ => (),
            },
            NodeOrExpression::Expression(expression) => {
                for product in &mut expression.products {
                    for side in [&mut product.numerator, &mut product.denominator] {
                        for element in side {
                            element.apply_to_every_element_mut(function);
                        }
                    }
                }
            }
        }
    }

    pub fn apply_to_every_element_into(
        mut self,
        function: &mut impl FnMut(Element) -> Element,
    ) -> Element {
        self = function(self);

        match self.node_or_expression {
            NodeOrExpression::Node(node) => {
                let new_node = match node {
                    Node::Power { base, power } => Node::Power {
                        base: Box::new(base.apply_to_every_element_into(function)),
                        power: Box::new(power.apply_to_every_element_into(function)),
                    },
                    Node::Modulo { lhs, rhs } => Node::Modulo {
                        lhs: Box::new(lhs.apply_to_every_element_into(function)),
                        rhs: Box::new(rhs.apply_to_every_element_into(function)),
                    },
                    Node::Factorial { child } => Node::Factorial {
                        child: Box::new(child.apply_to_every_element_into(function)),
                    },
                    Node::Function { name, arguments } => {
                        let new_args = arguments
                            .into_iter()
                            .map(|argument| argument.apply_to_every_element_into(function))
                            .collect_vec();

                        Node::Function {
                            name,
                            arguments: new_args,
                        }
                    }
                    _ => node,
                };

                Element::new(self.sign, NodeOrExpression::Node(new_node))
            }
            NodeOrExpression::Expression(expression) => {
                let mut new_expression = Expression::new(vec![]);

                for product in expression.products {
                    let mut new_product = Product::new(vec![], vec![]);

                    for element in product.numerator {
                        new_product
                            .numerator
                            .push(element.apply_to_every_element_into(function));
                    }
                    for element in product.denominator {
                        new_product
                            .denominator
                            .push(element.apply_to_every_element_into(function));
                    }

                    new_expression.products.push(new_product);
                }

                Element::new(self.sign, NodeOrExpression::Expression(new_expression))
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
