use crate::{
    tokenizer::{Number, Operation, Token},
    Expression, Node, NodeOrExpression,
};

use super::{ast::ExpressionOrEquation, token_to_rpn::ReversePolishNotation};

impl From<ReversePolishNotation> for ExpressionOrEquation {
    fn from(rpn: ReversePolishNotation) -> Self {
        let result = Expression::new();
        let mut stack: Vec<NodeOrExpression> = Vec::new();

        for token in rpn.tokens.into_iter() {
            match token {
                Token::Binary(operation) => {
                    let left = stack.pop().unwrap();
                    let right = stack.pop().unwrap();
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
        ExpressionOrEquation::Expression(result)
    }
}

/*
           Token::Number(number) => Node::Number(number.clone()),
           Token::Identifier {
               name,
               could_be_unit,
           } => {
               if *could_be_unit {
                   Node::Unit(name.clone())
               } else {
                   Node::Variable(name.clone())
               }
           }
           Token::Function(name, _) => Node::Function {
               name: name.clone(),
               arguments: vec![],
           },
*/
