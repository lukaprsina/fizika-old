use crate::tokenizer::{parser::TokenizedString, token::Associativity, Token};

#[derive(Debug)]
pub enum TokenParseError {
    MismatchedParenthesis,
    UnexpectedComma,
    NotEnoughOperands,
    TooManyOperands,
}

pub struct ReversePolishNotation {
    pub tokens: Vec<Token>,
}

impl TryFrom<TokenizedString> for ReversePolishNotation {
    type Error = TokenParseError;

    fn try_from(tokens: TokenizedString) -> Result<ReversePolishNotation, TokenParseError> {
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
                                println!("Left: {:?}", prev_token);
                                output.push(prev_token.clone());
                            }
                            ((i, Associativity::Right), (j, _)) if i < j => {
                                println!("Right: {:?}", prev_token);
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

        // add the last operation at the end
        while let Some(token) = stack.pop() {
            match token {
                Token::Unary(_) | Token::Binary(_) => output.push(token),
                Token::LeftParenthesis | Token::Function(..) => {
                    return Err(TokenParseError::MismatchedParenthesis)
                }
                _ => panic!("Unexpected token on stack."),
            }
        }

        // verify RPN
        let mut n_operands = 0isize;
        for (_index, token) in output.iter().enumerate() {
            match *token {
                Token::Identifier { .. } | Token::Number(_) => n_operands += 1,
                Token::Unary(_) => (),
                Token::Binary(_) => n_operands -= 1,
                Token::Function(_, Some(n_args)) => n_operands -= n_args as isize - 1,
                _ => panic!("Nothing else should be here"),
            }
            if n_operands <= 0 {
                return Err(TokenParseError::NotEnoughOperands);
            }
        }

        if n_operands > 1 {
            return Err(TokenParseError::TooManyOperands);
        }

        output.shrink_to_fit();

        println!("{:#?}", &output);
        Ok(ReversePolishNotation { tokens: output })
    }
}
