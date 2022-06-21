use std::fmt::Display;

use crate::ast::{
    expression::{IsTimesVisible, ShouldBeParenthesized},
    Equation, Expression, Node, NodeOrExpression, Product, Sign,
};

use super::expression::Element;

impl Display for Equation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();

        for side in self.sides.iter() {
            if let Some(operation) = &side.operation {
                result += &format!("{} {} ", side.element, operation);
            } else {
                result += &format!("{}", side.element);
            }
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

impl Display for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();

        let open = self.sign == Sign::Negative;

        if open {
            result.push('(');
        }

        result += &format!("{} {}", self.sign, self.node_or_expression);

        if open {
            result.push(')');
        }

        write!(f, "{}", result)
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
                result += &format!(
                    "{}^{}",
                    to_string_with_parenthesis(base.as_ref()),
                    to_string_with_parenthesis(power.as_ref()),
                );
            }
            Node::Modulo { lhs, rhs } => {
                result += &format!(
                    "{}%{}",
                    to_string_with_parenthesis(lhs.as_ref()),
                    to_string_with_parenthesis(rhs.as_ref())
                );
            }
            Node::Factorial { child } => {
                result += &format!("{}!", to_string_with_parenthesis(child.as_ref()));
            }
            Node::Function { name, arguments } => {
                result += &format!("{}(", name);
                for (index, argument) in arguments.iter().enumerate() {
                    result += &to_string_with_parenthesis(argument).to_string();
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

            if pos != self.products.len() - 1 {
                result.push(' ');
            }
        }

        write!(f, "{}", result)
    }
}

impl Display for Product {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();

        if self.numerator.is_empty() {
            result.push('1');
        }

        // println!("\nNum:\n{:#?}", self);

        let mut last: Option<&Element>;

        for (pos, side) in [&self.numerator, &self.denominator].into_iter().enumerate() {
            last = None;

            for (pos, element) in side.iter().enumerate() {
                let product_open = element.should_be_parenthesized()
                    || (pos > 1 && element.sign == Sign::Negative);

                if let Some(last) = last {
                    if element.is_times_visible(last) {
                        result += " * ";
                    }
                }

                if product_open {
                    result.push('(');
                }

                result += &element.to_string();

                if product_open {
                    result.push(')');
                }

                last = Some(element);
            }

            if pos == 0 && !self.denominator.is_empty() {
                result += "/";
            }
        }

        write!(f, "{}", result)
    }
}
