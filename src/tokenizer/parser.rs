use nom::{IResult, character::complete::multispace0};

use crate::tokenizer::{
    small_parsers::{
        parse_left_expression, parse_right_expression, parse_right_expression_no_parenthesis,
        parse_right_expression_with_comma, trim
    },
    Token,
};

#[derive(Debug)]
pub enum ParseError {
    UnexpectedToken(usize),
    MissingRightParenthesis(usize),
    MissingArgument,
}

enum TokenizerState {
    LeftExpression,
    RightExpression,
}

enum ParenthesisState {
    Subexpression,
    Function,
}

pub fn tokenize(input: &str) -> Result<Vec<Token>, ParseError> {
    let mut result: Vec<Token> = vec![];
    let mut parenthesis_stack: Vec<ParenthesisState> = vec![];
    let mut state = TokenizerState::LeftExpression;

    let mut work_string = input;    

    while !work_string.is_empty() {
        if let Ok(trimmed) = multispace0::<&str, nom::error::Error<_>>(work_string) {
            work_string = trimmed.0;
        }
        
        let parsing_result: IResult<&str, Token> = match (&state, parenthesis_stack.last()) {
            (TokenizerState::LeftExpression, _) => parse_left_expression(work_string),
            (TokenizerState::RightExpression, None) => {
                parse_right_expression_no_parenthesis(work_string)
            }
            (TokenizerState::RightExpression, Some(&ParenthesisState::Function)) => {
                parse_right_expression_with_comma(work_string)
            }
            (TokenizerState::RightExpression, Some(&ParenthesisState::Subexpression)) => {
                parse_right_expression(work_string)
            }
        };

        match parsing_result {
            Ok((rest, token)) => {
                match token {
                    Token::Binary(_) | Token::Comma => state = TokenizerState::LeftExpression,
                    Token::LeftParenthesis => {
                        parenthesis_stack.push(ParenthesisState::Subexpression)
                    }
                    Token::RightParenthesis => {
                        parenthesis_stack.pop().expect("Missing left parenthesis");
                    }
                    Token::Identifier {..} | Token::Number(..) => {
                        state = TokenizerState::RightExpression
                    }
                    Token::Function(..) => parenthesis_stack.push(ParenthesisState::Function),
                    _ => (),
                }

                result.push(token);
                work_string = rest;
            }
            Err(nom::Err::Error(_)) => {
                // TODO: handle error
                return Err(ParseError::UnexpectedToken(1));
            }
            Err(error) => panic!("{}", error),
        }
    }

    match state {
        TokenizerState::LeftExpression => Err(ParseError::MissingArgument),
        _ if !parenthesis_stack.is_empty() => {
            Err(ParseError::MissingRightParenthesis(parenthesis_stack.len()))
        }
        _ => Ok(result),
    }
}
