use std::fmt::Display;

use crate::ast::{
    expression::{IsTimesVisible, ShouldBeParenthesized},
    Equation, Expression, Node, NodeOrExpression, Product, Sign,
};

impl Display for Equation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();

        for (expression, operation) in self.expressions.iter() {
            if let Some(operation) = operation {
                result += &format!("{} {} ", expression, operation);
            } else {
                result += &format!("{}", expression);
            }
        }

        write!(f, "{}", result)
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();

        for (position, product) in self.products.iter().enumerate() {
            let mut open = false;
            match product.sign {
                Sign::Positive => {
                    if position != 0 {
                        result += "+ ";
                    }
                }
                Sign::Negative => {
                    result += "- ";
                    if product.numerator.len() > 1 {
                        open = true;
                        result.push('(');
                    }
                }
            }

            result += &product.to_string();

            if open {
                result.push(')');
            }

            if position != self.products.len() - 1 {
                result.push(' ');
            }
        }

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

impl Display for NodeOrExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let result = match self {
            NodeOrExpression::Node(node) => node.to_string(),
            NodeOrExpression::Expression(expression) => expression.to_string(),
        };

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

        let mut last: Option<&NodeOrExpression>;

        for (pos, side) in [&self.numerator, &self.denominator].iter().enumerate() {
            last = None;
            /* let mut open = false;
            if let Some(first) = side.first() {
                if first.should_be_parenthesized() {
                    open = true;
                }
            }
            if open {
                result.push('(');
            } */

            for node_or_expression in *side {
                let product_open = node_or_expression.should_be_parenthesized();

                if let Some(last) = last {
                    if node_or_expression.is_times_visible(last) {
                        result += " * ";
                    }
                }

                if product_open {
                    result.push('(');
                }

                result += &node_or_expression.to_string();

                if product_open {
                    result.push(')');
                }

                last = Some(node_or_expression);
            }

            /* if open {
                result.push(')');
            } */

            if pos == 0 && !self.denominator.is_empty() {
                result += "/";
            }
        }

        write!(f, "{}", result)
    }
}

/* impl Debug for Equation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Equation")
            .field("expressions", &self.expressions)
            .finish()
    }
}

impl Debug for NodeOrExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Node(arg0) => f.debug_tuple("Node").field(arg0).finish(),
            Self::Expression(arg0) => f.debug_tuple("Expression").field(arg0).finish(),
        }
    }
} */
