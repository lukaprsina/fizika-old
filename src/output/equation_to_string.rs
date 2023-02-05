use crate::ast::{Equation, Expression, Node, Sign};
use std::fmt::{Display, Write};

use crate::ast::{element::ShouldBeParenthesized, product::Product, Element, NodeOrExpression};

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

        // println!("{self:#?}");

        for (side_pos, side) in [&self.numerator, &self.denominator].into_iter().enumerate() {
            let side_length = side.len();
            /* let open = if side_length == 1 {
                let element = side.first().unwrap();
                element.sign != Sign::Positive
                //  self.denominator.is_empty()
            } else {
                side_length != 0
            }; */
            let open = false;

            if open {
                result.push('(');
            }

            for element in side.iter() {
                result += &element.to_string();
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
        // println!("{self:#?}");

        if self.sign != Sign::Positive {
            result.push_str(&format!("{}", self.sign));
        }

        match &self.node_or_expression {
            // TODO: if it's only a node, it's shit
            NodeOrExpression::Expression(expression) => {
                let product_len = expression.products.len();

                let open_expr = product_len > 1;

                if open_expr {
                    result.push_str("(");
                }

                for product in &expression.products {
                    // let open = !product.denominator.is_empty() && self.sign != Sign::Positive;
                    // let open_elem = false;
                    let open_elem = !product.denominator.is_empty()
                        || self.sign != Sign::Positive && product_len > 1;
                    if open_elem {
                        result.push_str("(");
                    }

                    for (side_pos, side) in [&product.numerator, &product.denominator]
                        .into_iter()
                        .enumerate()
                    {
                        let side_length = side.len();
                        let open_product = if product_len == 1 {
                            let element = side.first().unwrap();
                            element.sign != Sign::Positive
                            //  self.denominator.is_empty()
                        } else {
                            side_length != 0
                        };

                        if open_product {
                            // result.push('p');
                        }

                        for element in side {
                            result += &element.to_string();
                        }

                        if open_product {
                            // result.push(')');
                        }

                        if side_pos == 0 && !product.denominator.is_empty() {
                            result.push('/');
                        }
                    }

                    if open_elem {
                        result.push(')');
                    }
                }

                if open_expr {
                    result.push(')');
                }
            }
            NodeOrExpression::Node(node) => result += &node.to_string(),
        }

        // println!("{result}");
        write!(f, "{}", result)
    }
}
