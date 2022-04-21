use crate::tokenizer::Token;

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

    let mut rest = input;

    while !rest.is_empty() {
        let token = match (state, parenthesis_stack.last()) {
            (TokenizerState::LeftExpression, _) => todo!(),
            (TokenizerState::RightExpression, None) => todo!(),
            (TokenizerState::RightExpression, Some(&ParenthesisState::Function)) => todo!(),
            (TokenizerState::RightExpression, Some(&ParenthesisState::Subexpression)) => todo!(),
        };
    }
    Ok(result)
}
