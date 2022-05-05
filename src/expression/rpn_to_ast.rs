use crate::{
    tokenizer::{Number, Operation, Token},
    Expression, Node, NodeOrExpression,
};

use super::{ast::NodeOrExpressionOrEquation, token_to_rpn::ReversePolishNotation};

impl From<ReversePolishNotation> for NodeOrExpressionOrEquation {
    fn from(rpn: ReversePolishNotation) -> Self {
        let mut stack: Vec<NodeOrExpression> = Vec::new();

        for token in rpn.tokens.into_iter() {
            match token {
                Token::Binary(operation) => {
                    let right = stack.pop().unwrap();
                    let left = stack.pop().unwrap();
                    let r = match operation {
                        Operation::Add => left + right,
                        Operation::Subtract => left - right,
                        Operation::Multiply => left * right,
                        Operation::Divide => left / right,
                        _ => left,
                        /* Operation::Mod => todo!(),
                        Operation::Power => todo!(),
                        Operation::Factorial => todo!(),
                        Operation::Equal => todo!(),
                        Operation::NotEqual => todo!(),
                        Operation::LessThan => todo!(),
                        Operation::LessThanOrEqual => todo!(),
                        Operation::GreaterThanOrEqual => todo!(),
                        Operation::GreaterThan => todo!(), */
                    };
                    stack.push(r);
                }
                Token::Number(number) => stack.push(NodeOrExpression::Node(Node::Number(number))),
                Token::Identifier {
                    name,
                    could_be_unit,
                } => {
                    let node = if could_be_unit {
                        Node::Unit(name)
                    } else {
                        Node::Variable(name)
                    };

                    stack.push(NodeOrExpression::Node(node));
                }
                Token::Function(name, _) => {
                    stack.push(NodeOrExpression::Node(Node::Function {
                        name: name.clone(),
                        arguments: vec![],
                    }));
                }
                _ => (),
            }
        }
        assert!(stack.len() == 1);
        match stack.pop().unwrap() {
            NodeOrExpression::Node(node) => NodeOrExpressionOrEquation::Node(node),
            NodeOrExpression::Expression(expression) => {
                NodeOrExpressionOrEquation::Expression(expression)
            }
        }
    }
}
