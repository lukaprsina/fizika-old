use crate::ast::{analyzed_expression::AnalyzedElement, Equation, Expression, Node, Sign};
use std::fmt::{Display, Write};

use crate::ast::{
    element::IsTimesVisible, element::ShouldBeParenthesized, product::Product, Element,
    NodeOrExpression,
};

impl Display for Equation {
    /* TODO: equation has a reference to context */
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();

        /* let sides = self.sides();

        let mut chunks = sides.peekable();
        while let Some(side) = chunks.next() {
            if chunks.peek().is_some() {
                result += &format!("{} = ", side.element)
            } else {
                result += &format!("{}", side.element)
            }
        } */

        let app = self.app.borrow();

        for (pos, &uuid) in self.uuids.iter().enumerate() {
            let analyzed_element = app
                .get_context(self.context)
                .unwrap()
                .get_expression(uuid)
                .expect("The UUID must be valid");

            if pos < self.uuids.len() - 1 {
                write!(result, "{} = ", analyzed_element.element).unwrap()
            } else {
                write!(result, "{}", analyzed_element.element).unwrap()
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
            match product.numerator.first() {
                Some(element) => {
                    if pos == 0 {
                        if element.sign == Sign::Negative {
                            write!(result, "{} ", element.sign).unwrap()
                        }
                    } else {
                        write!(result, "{} ", element.sign).unwrap()
                    }
                }
                None => result += "+ 1",
            }

            result += &product.to_string();

            if pos != self.products.len() - 1 {
                result.push(' ');
            }
        }

        write!(f, "{}", result)
    }
}

impl Display for AnalyzedElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.element)
    }
}

impl Display for Product {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();

        // println!("\nNum:\n{:#?}", self);

        let mut last: Option<&Element>;

        for (pos, side) in [&self.numerator, &self.denominator].into_iter().enumerate() {
            last = None;

            for (pos, element) in side.iter().enumerate() {
                let explicit_minus = pos > 0 && element.sign == Sign::Negative;
                let product_open = element.should_be_parenthesized() || explicit_minus;

                if let Some(last) = last {
                    if element.is_times_visible(last) {
                        result += " * ";
                    }
                }

                if product_open {
                    result.push('(');
                }

                if explicit_minus {
                    write!(result, "{} ", element.sign).unwrap();
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

impl Display for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();

        let open = false; // TODO self.sign == Sign::Negative; // self.should_be_parenthesized();

        if open {
            write!(result, "({}", self.sign).unwrap();
        }

        result += &self.node_or_expression.to_string();

        if open {
            result.push(')');
        }

        write!(f, "{}", result)
    }
}
