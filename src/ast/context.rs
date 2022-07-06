use std::collections::HashMap;

use crate::{
    ast::{analyzed_equation::AnalyzedExpression, Equation},
    tokenizer::parser::ParseError,
};

use super::token_to_element::TokensToEquationError;

#[derive(Debug)]
pub struct Context {
    pub equations: HashMap<String, AnalyzedExpression>,
}

#[derive(Debug)]
pub enum CreateEquationError {
    ParseError(ParseError),
    TokensToEquationError(TokensToEquationError),
}

impl Context {
    pub fn new() -> Self {
        Context {
            equations: HashMap::new(),
        }
    }

    pub fn get_nth(&self, reference: &EquationReference) -> &AnalyzedExpression {
        self.equations
            .get(&reference.input)
            .expect("Invalid equation reference")
    }

    pub fn try_add_equation<T>(
        &mut self,
        input: T,
    ) -> Result<EquationReference, CreateEquationError>
    where
        T: TryInto<Equation, Error = CreateEquationError>,
    {
        let equation: Equation = input.try_into()?;
        Ok(self.add_equation(equation))
    }

    pub fn add_equation<T>(&mut self, input: T) -> EquationReference
    where
        T: Into<Equation>,
    {
        let equation: Equation = input.into();
        let eq_str = equation.to_string();

        let analyzed_equation = equation.analyze();

        self.equations.insert(eq_str.clone(), analyzed_equation);

        EquationReference::new(&eq_str)
    }

    pub fn solve(&self) {}
}

#[derive(Debug)]
pub struct EquationReference {
    input: String,
}

impl EquationReference {
    pub fn new(input: &str) -> Self {
        EquationReference {
            input: input.to_string(),
        }
    }

    pub fn get_equation<'a>(&self, context: &'a Context) -> &'a AnalyzedExpression {
        context.get_nth(&self)
    }
}
