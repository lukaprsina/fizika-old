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
            match product.numerator.first() {
                Some(element) => {
                    if pos == 0 {
                        if element.sign == Sign::Negative {
                            // write!(result, "{} ", element.sign).unwrap()
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

impl Display for Product {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();

        // println!("\nNum:\n{:#?}", self);

        let mut last: Option<&Element>;

        for (side_pos, side) in [&self.numerator, &self.denominator].into_iter().enumerate() {
            last = None;

            let open = side.len() >= 2;
            if open {
                result.push('(');
            }

            for element in side.iter() {
                let explicit_minus = side_pos > 0 && element.sign == Sign::Negative;

                let product_open = element.should_be_parenthesized() || explicit_minus;

                if let Some(last) = last {
                    if element.is_times_visible(last) {
                        result += " * ";
                    }
                }

                if product_open {
                    result.push('(');
                }

                /* if explicit_minus {
                    write!(result, "{} ", element.sign).unwrap();
                } */

                result += &element.to_string();

                if product_open {
                    result.push(')');
                }

                last = Some(element);
            }

            if open {
                result.push(')');
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

        // let open = false; // TODO self.sign == Sign::Negative; // self.should_be_parenthesized();
        let open = self.sign == Sign::Negative
            && matches!(self.node_or_expression, NodeOrExpression::Expression(_));

        if self.sign == Sign::Negative {
            write!(result, "{}", self.sign).unwrap();
        }

        if open {
            result.push('(');
        }

        result += &self.node_or_expression.to_string();

        if open {
            result.push(')');
        }

        write!(f, "{}", result)
    }
}
