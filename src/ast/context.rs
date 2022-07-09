use std::collections::HashMap;

use uuid::{uuid, Uuid};

use crate::{
    ast::{analyzed_equation::AnalyzedExpression, Equation},
    tokenizer::parser::ParseError,
};

use super::token_to_element::TokensToEquationError;

#[derive(Debug)]
pub struct Context {
    pub expressions: HashMap<Uuid, AnalyzedExpression>,
}

#[derive(Debug)]
pub enum CreateEquationError {
    ParseError(ParseError),
    TokensToEquationError(TokensToEquationError),
}

impl Context {
    pub fn new() -> Self {
        Context {
            expressions: HashMap::new(),
        }
    }

    pub fn get_expression(&self, uuid: Uuid) -> &AnalyzedExpression {
        self.expressions.get(&uuid).expect("Invalid equation UUID")
    }

    pub fn get_expression_mut(&self, uuid: Uuid) -> &mut AnalyzedExpression {
        self.expressions
            .get_mut(&uuid)
            .expect("Invalid equation UUID")
    }

    pub fn try_add_equation<T>(&mut self, input: T) -> Result<Uuid, CreateEquationError>
    where
        T: TryInto<Equation, Error = CreateEquationError>,
    {
        let equation: Equation = input.try_into()?;
        Ok(self.add_equation(equation))
    }

    pub fn add_equation<T>(&mut self, input: T) -> Uuid
    where
        T: Into<Equation>,
    {
        let equation: Equation = input.into();

        /* Don't accept equations */
        for a in  equation.uuids {

        }

        self.expressions.insert(k, v)

        uuid!("a")
    }

    pub fn solve(&self) {}
}
