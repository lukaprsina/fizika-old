use crate::ast::Equation;

#[derive(Debug)]
pub struct Context {
    pub equations: Vec<Equation>,
}
