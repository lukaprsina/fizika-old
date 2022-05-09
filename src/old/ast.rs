use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Neg, Sub},
};

use crate::tokenizer::{Number, Operation};

#[derive(Debug)]
pub struct Context {
    pub expressions: Vec<Expression>,
    pub equations: Vec<Equation>,
}

#[derive(Debug)]
pub enum ComparisonSign {
    Equal,
    NotEqual,
    LessThan,
    LessThanOrEqual,
    GreaterThanOrEqual,
    GreaterThan,
}

#[derive(Debug)]
pub struct Equation {
    pub expressions: Vec<(Expression, Operation)>,
}

impl Display for Equation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();

        for (expression, operation) in self.expressions.iter() {
            result += &format!("{} {} ", expression, operation);
        }

        write!(f, "{}", result)
    }
}

pub struct AnalyzedEquation {
    pub equation: Equation,
}

trait ShouldBeParenthesized {
    fn should_be_parenthesized(&self) -> bool;
}

#[derive(Debug)]
pub struct Expression {
    pub products: Vec<Product>,
}

impl ShouldBeParenthesized for Expression {
    fn should_be_parenthesized(&self) -> bool {
        self.products.len() > 1
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();

        if self.products.len() > 1 {
            result.push('(');
        }

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
                    if product.top.len() > 1 {
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

        if self.products.len() > 1 {
            result.push(')');
        }

        write!(f, "{}", result)
    }
}

impl Expression {
    pub fn new() -> Expression {
        Expression { products: vec![] }
    }
}

impl Default for Expression {
    fn default() -> Expression {
        Expression::new()
    }
}

#[derive(Debug)]
pub enum Sign {
    Positive,
    Negative,
}

#[derive(Debug)]
pub enum EquationSide {
    Left,
    Right,
}

#[derive(Debug)]
pub enum Node {
    Number(Number),
    Variable(String),
    Unit(String),
    Power {
        base: Box<NodeOrExpression>,
        power: Box<NodeOrExpression>,
    },
    Modulo {
        lhs: Box<NodeOrExpression>,
        rhs: Box<NodeOrExpression>,
    },
    Factorial {
        child: Box<NodeOrExpression>,
    },
    Function {
        name: String,
        arguments: Vec<NodeOrExpression>,
    },
}

impl ShouldBeParenthesized for Node {
    fn should_be_parenthesized(&self) -> bool {
        match self {
            Node::Power { base, power } => {
                base.should_be_parenthesized() || power.should_be_parenthesized()
            }
            Node::Modulo { lhs, rhs } => {
                lhs.should_be_parenthesized() || rhs.should_be_parenthesized()
            }
            Node::Factorial { child } => child.should_be_parenthesized(),
            _ => false,
        }
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();
        match self {
            Node::Number(number) => {
                result += &number.to_string();
            }
            Node::Variable(variable) => result += variable,
            Node::Unit(unit) => result += unit,
            Node::Power { base, power } => {
                let parenethesis =
                    base.should_be_parenthesized() || power.should_be_parenthesized();

                result += &format!(
                    "{}^{}{}{}",
                    base,
                    if parenethesis { "(" } else { "" },
                    power,
                    if parenethesis { ")" } else { "" }
                );

                /* for expression in [&base.products, &power.products] {
                    match expression.len() {
                        2.. => {
                            parenethesis = true;
                            break;
                        }
                        1 => {
                            for side in [&expression[0].top, &expression[0].bottom] {
                                match side.len() {
                                    2.. => {
                                        parenethesis = true;
                                        break;
                                    }
                                    1 => {
                                        if let NodeOrExpression::Expression(expression) = &side[0] {
                                            if expression.should_be_parenthesized() {
                                                parenethesis = true;
                                                break;
                                            }
                                        }
                                    }
                                    _ => (),
                                }
                            }
                        }
                        _ => (),
                    };
                } */
            }
            Node::Modulo { lhs, rhs } => {
                result += &format!("{}%{}", lhs, rhs);
            }
            Node::Factorial { child } => {
                result += &format!("{}!", child);
            }
            Node::Function { name, arguments } => {
                result += &format!("{}(", name);
                for (index, argument) in arguments.iter().enumerate() {
                    result += &format!("{}", argument);
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

#[derive(Debug)]
pub enum NodeOrExpression {
    Node(Node),
    Expression(Expression),
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

impl ShouldBeParenthesized for NodeOrExpression {
    fn should_be_parenthesized(&self) -> bool {
        match self {
            NodeOrExpression::Node(node) => node.should_be_parenthesized(),
            NodeOrExpression::Expression(expression) => expression.should_be_parenthesized(),
        }
    }
}

trait IsTimesVisible {
    fn is_times_visible(&self, last: &NodeOrExpression) -> bool;
}

impl IsTimesVisible for NodeOrExpression {
    fn is_times_visible(&self, last: &NodeOrExpression) -> bool {
        match self {
            // last * thing
            NodeOrExpression::Node(node) => match node {
                Node::Number(_)
                | Node::Power { .. }
                | Node::Function { .. }
                | Node::Modulo { .. }
                | Node::Factorial { .. } => true,
                Node::Variable(_) | Node::Unit(_) => match last {
                    NodeOrExpression::Node(var_node) => !matches!(
                        var_node,
                        Node::Number(_) | Node::Variable(_) | Node::Unit(_)
                    ),
                    NodeOrExpression::Expression(_) => false,
                },
            },
            NodeOrExpression::Expression(expression) => {
                if expression.products.len() >= 1 {
                    expression.products[0].is_times_visible(last)
                } else {
                    true
                }
            }
        }
    }
}

pub(crate) fn match_over_equation(
    lhs: NodeOrExpressionOrEquation,
    rhs: NodeOrExpressionOrEquation,
    mut func: impl FnMut(
        NodeOrExpressionOrEquation,
        NodeOrExpressionOrEquation,
    ) -> NodeOrExpressionOrEquation,
) -> NodeOrExpressionOrEquation {
    match lhs {
        NodeOrExpressionOrEquation::Node(node_lhs) => match rhs {
            NodeOrExpressionOrEquation::Node(node_rhs) => func(
                NodeOrExpressionOrEquation::Node(node_lhs),
                NodeOrExpressionOrEquation::Node(node_rhs),
            ),
            NodeOrExpressionOrEquation::Expression(exp_rhs) => func(
                NodeOrExpressionOrEquation::Node(node_lhs),
                NodeOrExpressionOrEquation::Expression(exp_rhs),
            ),
        },
        NodeOrExpressionOrEquation::Expression(exp_lhs) => match rhs {
            NodeOrExpressionOrEquation::Node(node_rhs) => func(
                NodeOrExpressionOrEquation::Expression(exp_lhs),
                NodeOrExpressionOrEquation::Node(node_rhs),
            ),
            NodeOrExpressionOrEquation::Expression(exp_rhs) => func(
                NodeOrExpressionOrEquation::Expression(exp_lhs),
                NodeOrExpressionOrEquation::Expression(exp_rhs),
            ),
        },
    }
}

impl Add for NodeOrExpressionOrEquation {
    type Output = NodeOrExpressionOrEquation;
    fn add(self, other: NodeOrExpressionOrEquation) -> NodeOrExpressionOrEquation {
        match_over_equation(
            self,
            other,
            |lhs: NodeOrExpressionOrEquation,
             rhs: NodeOrExpressionOrEquation|
             -> NodeOrExpressionOrEquation {
                let mut result = Expression::new();
                result
                    .products
                    .push(Product::new(Sign::Positive, vec![lhs], vec![]));
                result
                    .products
                    .push(Product::new(Sign::Positive, vec![rhs], vec![]));
                NodeOrExpressionOrEquation::Expression(result)
            },
        )
    }
}

impl Sub for NodeOrExpressionOrEquation {
    type Output = NodeOrExpressionOrEquation;
    fn sub(self, other: NodeOrExpressionOrEquation) -> NodeOrExpressionOrEquation {
        match_over_equation(
            self,
            other,
            |lhs: NodeOrExpressionOrEquation,
             rhs: NodeOrExpressionOrEquation|
             -> NodeOrExpressionOrEquation {
                let mut result = Expression::new();
                result
                    .products
                    .push(Product::new(Sign::Positive, vec![lhs], vec![]));
                result
                    .products
                    .push(Product::new(Sign::Negative, vec![rhs], vec![]));
                NodeOrExpressionOrEquation::Expression(result)
            },
        )
    }
}

impl Mul for NodeOrExpressionOrEquation {
    type Output = NodeOrExpressionOrEquation;
    fn mul(self, other: NodeOrExpressionOrEquation) -> NodeOrExpressionOrEquation {
        // println!("{} * {}", self, other);
        match_over_equation(
            self,
            other,
            |lhs: NodeOrExpressionOrEquation,
             rhs: NodeOrExpressionOrEquation|
             -> NodeOrExpressionOrEquation {
                let mut result = Expression::new();
                result
                    .products
                    .push(Product::new(Sign::Positive, vec![lhs, rhs], vec![]));
                NodeOrExpressionOrEquation::Expression(result)
            },
        )
    }
}

impl Div for NodeOrExpressionOrEquation {
    type Output = NodeOrExpressionOrEquation;
    fn div(self, other: NodeOrExpressionOrEquation) -> NodeOrExpressionOrEquation {
        match_over_equation(
            self,
            other,
            |lhs: NodeOrExpressionOrEquation,
             rhs: NodeOrExpressionOrEquation|
             -> NodeOrExpressionOrEquation {
                let mut result = Expression::new();
                result
                    .products
                    .push(Product::new(Sign::Positive, vec![lhs], vec![rhs]));
                NodeOrExpressionOrEquation::Expression(result)
            },
        )
    }
}

impl Neg for NodeOrExpressionOrEquation {
    type Output = NodeOrExpressionOrEquation;
    fn neg(self) -> NodeOrExpressionOrEquation {
        let mut result = Expression::new();
        result
            .products
            .push(Product::new(Sign::Negative, vec![self], vec![]));
        NodeOrExpressionOrEquation::Expression(result)
    }
}

#[derive(Debug)]
pub struct Product {
    pub sign: Sign,
    pub top: Vec<NodeOrExpression>,
    pub bottom: Vec<NodeOrExpression>,
}

impl Product {
    pub fn new(sign: Sign, top: Vec<NodeOrExpression>, bottom: Vec<NodeOrExpression>) -> Product {
        Product { sign, top, bottom }
    }
}

impl IsTimesVisible for Product {
    fn is_times_visible(&self, last: &NodeOrExpression) -> bool {
        if self.top.len() >= 1 {
            self.top[0].is_times_visible(last)
        } else {
            true
        }
    }
}

impl Display for Product {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();

        let mut top_side = true;
        if self.top.is_empty() {
            result.push('1');
        }

        let mut last: Option<&NodeOrExpression> = None;

        for side in [&self.top, &self.bottom] {
            if side.len() > 1 {
                result.push('(');
            }

            for node_or_expression in side {
                if let Some(last) = last {
                    if node_or_expression.is_times_visible(last) {
                        result += " * ";
                    }
                }
                result += &node_or_expression.to_string();
                last = Some(node_or_expression);
            }

            if side.len() > 1 {
                result.push(')');
            }

            if top_side && !self.bottom.is_empty() {
                result += "/";
                top_side = false;
            }
        }

        // println!("Product: {}", result);
        write!(f, "{}", result)
    }
}

#[derive(Debug)]
pub enum NodeOrExpressionOrEquation {
    Node(Node),
    Expression(Expression),
    Equation(Equation),
}

impl Display for NodeOrExpressionOrEquation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NodeOrExpressionOrEquation::Node(node) => write!(f, "{}", node),
            NodeOrExpressionOrEquation::Expression(expression) => {
                let mut result = expression.to_string();

                let mut min = usize::MAX;
                let mut counter = 0;

                if !result.is_empty()
                    && result.chars().next().unwrap() == '('
                    && result.chars().last().unwrap() == ')'
                {
                    for (pos, c) in result.chars().enumerate() {
                        match c {
                            '(' => {
                                counter += 1;
                                min = min.min(counter);
                            }
                            ')' => {
                                counter -= 1;
                                if pos != result.len() - 1 {
                                    min = min.min(counter);
                                }
                            }
                            _ => (),
                        }
                    }

                    result = result
                        .chars()
                        .enumerate()
                        .filter_map(|(pos, c)| {
                            if pos >= min && (result.len() - pos) > min {
                                Some(c)
                            } else {
                                None
                            }
                        })
                        .collect();
                }

                write!(f, "{}", result)
            }
            NodeOrExpressionOrEquation::Equation(equation) => write!(f, "{}", equation),
        }
    }
}
