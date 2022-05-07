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

impl ReversePolishNotation {
    pub fn new_from_tokens(tokens: Vec<Token>) -> Self {
        Self { tokens }
    }
}

impl TryFrom<TokenizedString> for ReversePolishNotation {
    type Error = TokenParseError;

    fn try_from(
        tokenized_string: TokenizedString,
    ) -> Result<ReversePolishNotation, TokenParseError> {
        let mut stack: Vec<Token> = Vec::new();
        let mut output: Vec<Token> = Vec::new();
        let mut skip_n = 0;
        // println!("{:#?}", tokenized_string);

        for (pos, token) in tokenized_string.tokens.iter().enumerate() {
            // TODO: maybe filter
            if skip_n != 0 {
                skip_n -= 1;
                continue;
            }
            // println!("stack:{:?}\noutput:{:#?}\n", stack, output);

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
                    skip_n = counter - 1;
                    output.push(Token::Function {
                        name: name.to_string(),
                        num_of_args: Some(which_arg + 1),
                        arguments: arguments.clone(),
                    })
                }
                Token::Binary(_) => {
                    let pa1 = token.get_precedence_and_associativity();

                    while !stack.is_empty() {
                        let pa2 = stack.last().unwrap().get_precedence_and_associativity();
                        match (pa1, pa2) {
                            ((i, Associativity::Left), (j, _)) if i <= j => {
                                output.push(stack.pop().unwrap());
                            }
                            ((i, Associativity::Right), (j, _)) if i < j => {
                                output.push(stack.pop().unwrap());
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
                            _ => output.push(token),
                        }
                    }
                    if !found {
                        return Err(TokenParseError::MismatchedParenthesis);
                    }
                }
                Token::Comma => {
                    while let Some(token) = stack.pop() {
                        match token {
                            Token::LeftParenthesis => {
                                return Err(TokenParseError::UnexpectedComma);
                            }
                            _ => output.push(token),
                        }
                    }
                }
            }
        }

        // println!("stack:{:?}\noutput:{:#?}", stack, output);

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
        /* let mut n_operands = 0isize;
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
        } */

        output.shrink_to_fit();

        Ok(ReversePolishNotation { tokens: output })
    }
}
