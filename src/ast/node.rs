use crate::{
    ast::{
        expression::{IsTimesVisible, ShouldBeParenthesized},
        NodeOrExpression,
    },
    tokenizer::Number,
};

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
        false
    }
}

impl IsTimesVisible for Node {
    fn is_times_visible(&self, last: &NodeOrExpression) -> bool {
        match self {
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
        }
    }
}
