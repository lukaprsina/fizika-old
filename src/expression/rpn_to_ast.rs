use core::panic;

use crate::{
    expression::ast::match_over_equation,
    tokenizer::{parser::TokenizedString, Number, Operation, Token},
    Equation, Expression, Node, NodeOrExpression, Product, Sign,
};

use super::{ast::NodeOrExpressionOrEquation, token_to_rpn::ReversePolishNotation};

impl From<ReversePolishNotation> for NodeOrExpressionOrEquation {
    fn from(rpn: ReversePolishNotation) -> Self {
        let mut stack: Vec<NodeOrExpressionOrEquation> = Vec::new();
        // println!("{:?}", rpn);

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
                        Operation::Mod => match_over_equation(
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
                        Operation::Power => match_over_equation(
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
                        Operation::Equal => match_over_equation(
                            left,
                            right,
                            |lhs: NodeOrExpression, rhs: NodeOrExpression| -> NodeOrExpression {
                                Equation {
                                    lhs,
                                    sign: Operation::Equal,
                                    rhs,
                                }
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
                // TODO
                Token::Function {
                    name,
                    num_of_args: _,
                    arguments,
                } => {
                    let mut result = Vec::new();
                    for argument in arguments.into_iter() {
                        let tokenized_string = TokenizedString::new_from_tokens(argument);
                        if let Ok(rpn) = ReversePolishNotation::try_from(tokenized_string) {
                            result.push(match NodeOrExpressionOrEquation::from(rpn) {
                                NodeOrExpressionOrEquation::Node(node) => {
                                    NodeOrExpression::Node(node)
                                }
                                NodeOrExpressionOrEquation::Expression(expression) => {
                                    NodeOrExpression::Expression(expression)
                                }
                                NodeOrExpressionOrEquation::Equation(_) => {
                                    panic!("Can't have equations in functions")
                                }
                            });
                        }
                    }

                    stack.push(NodeOrExpression::Node(Node::Function {
                        name: name.clone(),
                        arguments: result,
                    }));
                }
                _ => (),
            }
        }

        // println!("{:#?}", stack);
        assert!(stack.len() == 1);
        stack.pop().unwrap()
    }
}
