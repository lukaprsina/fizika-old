use crate::{
    expression::ast::match_over_node_or_expression,
    tokenizer::{Number, Operation, Token},
    Expression, Node, NodeOrExpression, Product, Sign,
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
                        Operation::Mod => match_over_node_or_expression(
                            left,
                            right,
                            |lhs: NodeOrExpression, rhs: NodeOrExpression| -> NodeOrExpression {
                                let mut result = Expression::new();
                                result.products.push(Product::new(
                                    Sign::Positive,
                                    vec![NodeOrExpression::Node(Node::Modulo {
                                        lhs: Box::new(lhs),
                                        rhs: Box::new(rhs),
                                    })],
                                    vec![],
                                ));
                                NodeOrExpression::Expression(result)
                            },
                        ),
                        Operation::Power => match_over_node_or_expression(
                            left,
                            right,
                            |lhs: NodeOrExpression, rhs: NodeOrExpression| -> NodeOrExpression {
                                let mut result = Expression::new();
                                result.products.push(Product::new(
                                    Sign::Positive,
                                    vec![NodeOrExpression::Node(Node::Power {
                                        base: Box::new(lhs),
                                        power: Box::new(rhs),
                                    })],
                                    vec![],
                                ));
                                NodeOrExpression::Expression(result)
                            },
                        ),
                        _ => unimplemented!(),
                        /* Operation::Equal => todo!(),
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
