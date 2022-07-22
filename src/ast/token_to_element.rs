use thiserror::Error;

use crate::{
    ast::{Element, NodeOrExpression},
    tokenizer::{parser::TokenizedString, token::Associativity, Operation, Token},
};

use super::{
    equation::{EquationSide, NoContextEquation},
    Node, Sign,
};

#[derive(Debug, Error)]
pub enum TokenParseError {
    #[error("Mismatched parenthesis at location {0}")]
    MismatchedParenthesis(usize),
    #[error("Unexpected comma at location ")]
    UnexpectedComma(usize),
    #[error("Not enough operands at location {0}")]
    NotEnoughOperands(usize),
    #[error("Too many operands")]
    TooManyOperands,
}

fn tokens_to_rpn<'a, I>(
    iterator: &mut I,
) -> Result<(Vec<Token>, Option<Operation>), TokenParseError>
where
    I: Iterator<Item = &'a Token>,
{
    let mut stack: Vec<(Token, usize)> = Vec::new();
    let mut output: Vec<Token> = Vec::new();
    let mut equal_sign = None;

    for (pos, token) in iterator.enumerate() {
        let token = token.clone();
        match token {
            Token::Number(_) | Token::Identifier { .. } => output.push(token),
            Token::Unary(_) => stack.push((token, pos)),
            Token::Binary(ref operation) => {
                if operation.is_comparison_sign() {
                    equal_sign = Some(operation.clone());
                    break;
                }

                let pa1 = token.get_precedence_and_associativity();

                while !stack.is_empty() {
                    let pa2 = stack.last().unwrap().0.get_precedence_and_associativity();
                    match (pa1, pa2) {
                        ((i, Associativity::Left), (j, _)) if i <= j => {
                            output.push(stack.pop().unwrap().0);
                        }
                        ((i, Associativity::Right), (j, _)) if i < j => {
                            output.push(stack.pop().unwrap().0);
                        }
                        _ => {
                            break;
                        }
                    }
                }

                stack.push((token, pos));
            }
            Token::LeftParenthesis => stack.push((token, pos)),
            Token::RightParenthesis => {
                let mut found = false;
                while let Some((t, _)) = stack.pop() {
                    match t {
                        Token::LeftParenthesis => {
                            found = true;
                            break;
                        }
                        Token::Function { name, num_of_args } => {
                            found = true;
                            output.push(Token::Function {
                                name,
                                num_of_args: Some(num_of_args.unwrap_or(0) + 1),
                            });
                            break;
                        }
                        _ => output.push(t),
                    }
                }

                if !found {
                    return Err(TokenParseError::MismatchedParenthesis(pos));
                }
            }
            Token::Comma => {
                let mut found = false;
                while let Some((t, i)) = stack.pop() {
                    match t {
                        Token::LeftParenthesis => {
                            return Err(TokenParseError::UnexpectedComma(pos));
                        }
                        Token::Function { name, num_of_args } => {
                            found = true;
                            stack.push((
                                Token::Function {
                                    name,
                                    num_of_args: Some(num_of_args.unwrap_or(0) + 1),
                                },
                                i,
                            ));
                            break;
                        }
                        _ => output.push(t),
                    }
                }

                if !found {
                    return Err(TokenParseError::UnexpectedComma(pos));
                }
            }
            Token::Function { .. } => stack.push((token, pos)),
        }
    }

    while let Some((token, index)) = stack.pop() {
        match token {
            Token::Unary(_) | Token::Binary(_) => output.push(token),
            Token::LeftParenthesis | Token::Function { .. } => {
                return Err(TokenParseError::MismatchedParenthesis(index));
            }
            _ => panic!("Unexpected token on stack."),
        }
    }

    let mut n_operands = 0isize;
    for (index, token) in output.iter().enumerate() {
        match *token {
            Token::Identifier { .. } | Token::Number(_) => n_operands += 1,
            Token::Unary(_) => (),
            Token::Binary(_) => n_operands -= 1,
            Token::Function {
                name: _,
                num_of_args: Some(n_args),
            } => n_operands -= n_args as isize - 1,
            _ => panic!("Nothing else should be here"),
        }
        if n_operands <= 0 {
            return Err(TokenParseError::NotEnoughOperands(index));
        }
    }

    if n_operands > 1 {
        return Err(TokenParseError::TooManyOperands);
    }

    output.shrink_to_fit();

    Ok((output, equal_sign))
}

#[derive(Debug, Error)]
pub enum AbstractSyntaxTreeError {
    #[error("Unary AST error")]
    Unary,
    #[error("Binary AST error")]
    Binary,
}

fn rpn_to_ast(tokens: &[Token]) -> Result<Element, AbstractSyntaxTreeError> {
    let mut stack: Vec<Element> = Vec::new();

    for token in tokens.iter() {
        let token = token.clone();

        match token {
            Token::Number(number) => stack.push(Element::new(
                Sign::Positive,
                NodeOrExpression::Node(Node::Number(number)),
            )),
            Token::Identifier {
                name,
                could_be_unit: _,
            } => {
                // TODO: need context to determine if this is a unit
                /* let node = if could_be_unit {
                    Node::Unit(name)
                } else {
                    Node::Variable(name)
                }; */
                let node = Node::Variable(name);

                stack.push(Element::new(Sign::Positive, NodeOrExpression::Node(node)));
            }
            Token::Unary(operation) => {
                let mut child = stack.pop().expect("Expected a token in the stack");
                let result = match operation {
                    Operation::Add => child,
                    Operation::Subtract => {
                        child.invert_sign();
                        child
                    }
                    _ => return Err(AbstractSyntaxTreeError::Unary),
                };
                stack.push(result);
            }
            Token::Binary(operation) => {
                let right = stack.pop().expect("Expected a token in the stack");
                let left = stack.pop().expect("Expected a token in the stack");

                let result = match operation {
                    Operation::Add => left + right,
                    Operation::Subtract => left - right,
                    Operation::Multiply => left * right,
                    Operation::Divide => left / right,
                    Operation::Mod => Element::new(
                        Sign::Positive,
                        NodeOrExpression::Node(Node::Modulo {
                            lhs: Box::new(left),
                            rhs: Box::new(right),
                        }),
                    ),
                    Operation::Power => Element::new(
                        Sign::Positive,
                        NodeOrExpression::Node(Node::Power {
                            base: Box::new(left),
                            power: Box::new(right),
                        }),
                    ),
                    Operation::Equal
                    | Operation::NotEqual
                    | Operation::LessThan
                    | Operation::LessThanOrEqual
                    | Operation::GreaterThanOrEqual
                    | Operation::GreaterThan => unreachable!(),
                    _ => unreachable!(),
                };
                stack.push(result);
            }
            Token::Function { name, num_of_args } => {
                let num_of_args = num_of_args.expect("Expected a number of arguments");

                let arguments = stack.drain(0..num_of_args).collect::<Vec<_>>();
                let function = Element::new(
                    Sign::Positive,
                    NodeOrExpression::Node(Node::Function { name, arguments }),
                );

                stack.push(function);
            }
            _ => (),
        }
    }

    // println!("{:?}", stack);

    assert!(stack.len() == 1);
    Ok(stack.pop().unwrap())
}

#[derive(Debug, Error)]
pub enum TokensToEquationError {
    #[error("Token parse error: {0}")]
    TokenParseError(TokenParseError),
    #[error("AST error: {0}")]
    AbstractSyntaxTreeError(AbstractSyntaxTreeError),
}

impl TryFrom<TokenizedString> for NoContextEquation {
    type Error = TokensToEquationError;

    fn try_from(
        tokenized_string: TokenizedString,
    ) -> Result<NoContextEquation, TokensToEquationError> {
        let mut sides: Vec<EquationSide> = Vec::new();
        let mut token_iter = tokenized_string.iter();
        let mut should_continue = true;
        while should_continue {
            let result = tokens_to_rpn(&mut token_iter);

            if let Ok(rpn) = result {
                if rpn.1.is_none() {
                    should_continue = false;
                }
                if let Ok(expression) = rpn_to_ast(&rpn.0) {
                    sides.push(EquationSide::new(expression, rpn.1));
                }
            }
        }
        Ok(NoContextEquation { sides })
    }
}
