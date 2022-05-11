use crate::ast::Node;
pub(crate) trait ShouldBeParenthesized {
    fn should_be_parenthesized(&self) -> bool;
}

pub(crate) trait IsTimesVisible {
    fn is_times_visible(&self, last: &NodeOrExpression) -> bool;
}

#[derive(Debug)]
pub enum NodeOrExpression {
    Node(Node),
    Expression(Expression),
}

impl IsTimesVisible for NodeOrExpression {
    fn is_times_visible(&self, last: &NodeOrExpression) -> bool {
        match self {
            NodeOrExpression::Node(node) => node.is_times_visible(last),
            NodeOrExpression::Expression(expression) => expression.is_times_visible(last),
        }
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

#[derive(Debug)]
pub enum Sign {
    Positive,
    Negative,
}

#[derive(Debug)]
pub struct Product {
    pub sign: Sign,
    pub top: Vec<NodeOrExpression>,
    pub bottom: Vec<NodeOrExpression>,
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

#[derive(Debug)]
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

impl IsTimesVisible for Expression {
    fn is_times_visible(&self, last: &NodeOrExpression) -> bool {
        if self.products.len() >= 1 {
            self.products[0].is_times_visible(last)
        } else {
            true
        }
    }
}

impl ShouldBeParenthesized for Expression {
    fn should_be_parenthesized(&self) -> bool {
        self.products.len() > 1
    }
}
