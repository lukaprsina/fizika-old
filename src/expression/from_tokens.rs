use crate::{
    tokenizer::{parser::TokenizedString, token::Associativity, Token},
    Expression,
};

use super::ast::ExpressionOrEquation;

#[derive(Debug)]
pub enum TokenParseError {
    MismatchedParenthesis,
    UnexpectedComma,
}

impl TryFrom<TokenizedString> for ExpressionOrEquation {
    type Error = TokenParseError;

    fn try_from(tokens: TokenizedString) -> Result<ExpressionOrEquation, TokenParseError> {
        let result = Expression::new();
        let mut stack: Vec<Token> = Vec::new();
        let mut output: Vec<Token> = Vec::new();

        for token in tokens.tokens.iter().cloned() {
            match token {
                Token::Number(_) | Token::Identifier { .. } => output.push(token),
                Token::Unary(_) | Token::Function(..) => stack.push(token),
                Token::Binary(_) => {
                    let pa1 = token.get_precedence_and_associativity();

                    for prev_token in stack.iter_mut().rev() {
                        let pa2 = prev_token.get_precedence_and_associativity();
                        match (pa1, pa2) {
                            ((i, Associativity::Left), (j, _)) if i <= j => {
                                output.push(prev_token.clone());
                            }
                            ((i, Associativity::Right), (j, _)) if i < j => {
                                output.push(prev_token.clone());
                            }
                            _ => {
                                break;
                            }
                        }
                    }

                    stack.push(token);
                }
                Token::LeftParenthesis => stack.push(token),
                Token::RightParenthesis => {
                    let mut found = false;
                    while let Some(token) = stack.pop() {
                        match token {
                            Token::LeftParenthesis => {
                                found = true;
                                break;
                            }
                            Token::Function(name, nargs) => {
                                found = true;
                                output.push(Token::Function(name, Some(nargs.unwrap_or(0) + 1)));
                                break;
                            }
                            _ => output.push(token),
                        }
                    }
                    if !found {
                        return Err(TokenParseError::MismatchedParenthesis);
                    }
                }
                Token::Comma => {
                    let mut found = false;
                    while let Some(token) = stack.pop() {
                        match token {
                            Token::LeftParenthesis => {
                                return Err(TokenParseError::UnexpectedComma);
                            }
                            Token::Function(name, nargs) => {
                                found = true;
                                stack.push(Token::Function(name, Some(nargs.unwrap_or(0) + 1)));
                                break;
                            }
                            _ => output.push(token),
                        }
                    }
                    if !found {
                        return Err(TokenParseError::MismatchedParenthesis);
                    }
                }
            }
        }
        println!("{:?}", &output);
        Ok(ExpressionOrEquation::Expression(result))
    }
}
