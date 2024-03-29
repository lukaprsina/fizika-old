use crate::ast::{Equation, Expression, Node, Sign};
use std::fmt::{Display, Write};

use crate::ast::{
    element::IsTimesVisible, element::ShouldBeParenthesized, product::Product, Element,
    NodeOrExpression,
};

impl Display for Equation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();

        let len = self.eq_sides.len() as isize - 2;
        if len.is_positive() {
            for side in &self.eq_sides[0..len as usize] {
                result += &format!("{} = ", side)
            }
        }

        if let Some(last) = self.eq_sides.last() {
            result += &format!("{}", last)
        }

        write!(f, "{}", result)
    }
}

impl Display for Sign {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Sign::Positive => write!(f, "+"),
            Sign::Negative => write!(f, "-"),
        }
    }
}

impl Display for NodeOrExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let result = match self {
            NodeOrExpression::Node(node) => node.to_string(),
            NodeOrExpression::Expression(expression) => expression.to_string(),
        };

        write!(f, "{}", result)
    }
}

fn to_string_with_parenthesis<T: ShouldBeParenthesized + Display>(item: &T) -> String {
    let parenthesis = item.should_be_parenthesized();
    format!(
        "{}{}{}",
        if parenthesis { "(" } else { "" },
        &item.to_string(),
        if parenthesis { ")" } else { "" },
    )
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();
        match self {
            Node::Number(number) => result += &number.to_string(),
            Node::Variable(variable) => result += variable,
            Node::Unit(unit) => result += unit,
            Node::Power { base, power } => {
                write!(
                    result,
                    "{}^{}",
                    to_string_with_parenthesis(base.as_ref()),
                    to_string_with_parenthesis(power.as_ref()),
                )
                .unwrap();
            }
            Node::Modulo { lhs, rhs } => {
                write!(
                    result,
                    "{}%{}",
                    to_string_with_parenthesis(lhs.as_ref()),
                    to_string_with_parenthesis(rhs.as_ref())
                )
                .unwrap();
            }
            Node::Factorial { child } => {
                write!(result, "{}!", to_string_with_parenthesis(child.as_ref())).unwrap();
            }
            Node::Function { name, arguments } => {
                write!(result, "{}(", name).unwrap();
                for (index, argument) in arguments.iter().enumerate() {
                    result += &argument.to_string();
                    if index < arguments.len() - 1 {
                        result += ", ";
                    }
                }
                result += ")";
            }
        }

        write!(f, "{}", result)
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();

        for (pos, product) in self.products.iter().enumerate() {
            result += &product.to_string();
        }

        write!(f, "{}", result)
    }
}

impl Display for Product {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();

        if self.numerator.is_empty() {
            result += " 1";
        }

        let mut last: Option<&Element>;

        for (side_pos, side) in [&self.numerator, &self.denominator].into_iter().enumerate() {
            last = None;

            for element in side.iter() {
                result += &element.to_string();

                last = Some(element);
            }

            if side_pos == 0 && !self.denominator.is_empty() {
                result.push('/');
            }
        }

        write!(f, "{}", result)
    }
}

impl Display for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();

        result += &self.node_or_expression.to_string();

        write!(f, "{}", result)
    }
}
