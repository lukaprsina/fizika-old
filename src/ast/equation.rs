use crate::{ast::Expression, tokenizer::Operation};

#[derive(Debug)]
pub struct Equation {
    pub expressions: Vec<(Expression, Operation)>,
}
