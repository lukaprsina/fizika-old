use crate::{
    ast::Equation,
    tokenizer::parser::{ParseError, TokenizedString},
};

use super::token_to_element::TokensToEquationError;

#[derive(Debug)]
pub struct Context {
    pub equations: Vec<Equation>,
}

#[derive(Debug)]
pub enum AddEquationError {
    ParseError(ParseError),
    TokensToEquationError(TokensToEquationError),
}

impl Context {
    pub fn new() -> Self {
        Context { equations: vec![] }
    }

    pub fn get_nth(&self, reference: &EquationReference) -> &Equation {
        self.equations
            .get(reference.index)
            .expect("Invalid equation reference")
    }

    pub fn add_equation(&mut self, equation: &str) -> Result<EquationReference, AddEquationError> {
        let tokens =
            TokenizedString::try_new(equation).map_err(|err| AddEquationError::ParseError(err))?;

        let mut ast = Equation::try_from(tokens)
            .map_err(|err| AddEquationError::TokensToEquationError(err))?;

        ast.flatten();

        self.equations.push(ast);

        Ok(EquationReference::new(self.equations.len() - 1))
    }
}

#[derive(Debug)]
pub struct EquationReference {
    index: usize,
}

impl EquationReference {
    pub fn new(index: usize) -> Self {
        EquationReference { index }
    }

    pub fn get_equation<'a>(&self, context: &'a Context) -> &'a Equation {
        context.get_nth(&self)
    }
}
