use crate::tokenizer::Number;

use super::{
    element::{IsTimesVisible, NodeOrExpression, ShouldBeParenthesized},
    Element,
};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Node {
    Number(Number),
    Variable(String),
    Unit(String),
    Power {
        base: Box<Element>,
        power: Box<Element>,
    },
    Modulo {
        lhs: Box<Element>,
        rhs: Box<Element>,
    },
    Factorial {
        child: Box<Element>,
    },
    Function {
        name: String,
        arguments: Vec<Element>,
    },
}

impl ShouldBeParenthesized for Node {
    fn should_be_parenthesized(&self) -> bool {
        false
    }
}

impl IsTimesVisible for Node {
    fn is_times_visible(&self, last: &Element) -> bool {
        match self {
            Node::Number(_)
            | Node::Power { .. }
            | Node::Function { .. }
            | Node::Modulo { .. }
            | Node::Factorial { .. } => true,
            Node::Variable(_) | Node::Unit(_) => match &last.node_or_expression {
                NodeOrExpression::Node(var_node) => !matches!(
                    var_node,
                    Node::Number(_) | Node::Variable(_) | Node::Unit(_)
                ),
                NodeOrExpression::Expression(_) => false,
            },
        }
    }
}
