use crate::{ast::NodeOrExpression, tokenizer::Operation};

#[derive(Debug)]
pub struct Equation {
    pub expressions: Vec<(NodeOrExpression, Option<Operation>)>,
}
