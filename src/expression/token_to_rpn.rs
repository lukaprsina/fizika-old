use crate::tokenizer::{parser::TokenizedString, token::Associativity, Token};

#[derive(Debug)]
pub enum TokenParseError {
    MismatchedParenthesis,
    UnexpectedComma,
    NotEnoughOperands,
    TooManyOperands,
}

#[derive(Debug)]
pub struct ReversePolishNotation {
    pub tokens: Vec<Token>,
}

impl TryFrom<TokenizedString> for ReversePolishNotation {
    type Error = TokenParseError;

    fn try_from(
        tokenized_string: TokenizedString,
    ) -> Result<ReversePolishNotation, TokenParseError> {
        let mut stack: Vec<Token> = Vec::new();
        let mut output: Vec<Token> = Vec::new();
        let mut skip_n = 0;

        for (pos, token) in tokenized_string.tokens.iter().enumerate() {
            // TODO: maybe filter
            if skip_n != 0 {
                skip_n -= 1;
                continue;
            }

            let token = token.clone();

            match token {
                Token::Number(_) | Token::Identifier { .. } => output.push(token),
                Token::Unary(_) => stack.push(token),
                Token::Function {
                    name,
                    num_of_args: _,
                    mut arguments,
                } => {
                    // instead of skipping, read the stack
                    let mut counter = 1;
                    let mut which_arg = 0;
                    let mut got_all_args = false;
                    arguments.push(vec![]);

                    while !got_all_args {
                        if let Some(arg_token) = tokenized_string.tokens.iter().nth(pos + counter) {
                            match arg_token {
                                Token::Comma => {
                                    arguments.push(vec![]);
                                    which_arg += 1;
                                }
                                Token::RightParenthesis => {
                                    got_all_args = true;
                                }
                                // TODO: nested functions
                                Token::Function { .. } => todo!(),
                                _ => arguments[which_arg].push(arg_token.clone()),
                            }
                        }
                        counter += 1;
                    }
                    skip_n = counter;
                    output.push(Token::Function {
                        name: name.to_string(),
                        num_of_args: Some(which_arg + 1),
                        arguments: arguments.clone(),
                    })
                }
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
                            Token::Function {
                                name,
                                num_of_args,
                                arguments,
                            } => {
                                found = true;
                                output.push(Token::Function {
                                    name,
                                    num_of_args: Some(num_of_args.unwrap_or(0) + 1),
                                    arguments: vec![],
                                });
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
                            Token::Function {
                                name,
                                num_of_args,
                                arguments,
                            } => {
                                found = true;
                                stack.push(Token::Function {
                                    name,
                                    num_of_args: Some(num_of_args.unwrap_or(0) + 1),
                                    arguments: vec![],
                                });
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

        println!("Stack: {:#?}\nOutput: {:#?}", &stack, &output);

        // add the last operation at the end
        while let Some(token) = stack.pop() {
            match token {
                Token::Unary(_) | Token::Binary(_) => output.push(token),
                Token::LeftParenthesis | Token::Function { .. } => {
                    return Err(TokenParseError::MismatchedParenthesis)
                }
                _ => panic!("Unexpected token on stack."),
            }
        }

        // verify RPN
        // TODO
        let mut n_operands = 0isize;
        for (_index, token) in output.iter().enumerate() {
            match *token {
                Token::Identifier { .. } | Token::Number(_) => n_operands += 1,
                Token::Unary(_) => (),
                Token::Binary(_) => n_operands -= 1,
                Token::Function {
                    name: _,
                    num_of_args: Some(n_args),
                    arguments: _,
                } => n_operands -= n_args as isize - 1,
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

        Ok(ReversePolishNotation { tokens: output })
    }
}
