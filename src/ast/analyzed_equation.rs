use std::collections::HashMap;

use super::Equation;

#[derive(Debug)]
pub struct AnalyzedExpression {
    pub expression: Equation,
    pub variables: HashMap<String, usize>,
}

impl Equation {
    pub fn analyze(&self) -> AnalyzedExpression {
        AnalyzedExpression {
            expression: self.clone(),
            variables: HashMap::new(),
        }
    }
}
