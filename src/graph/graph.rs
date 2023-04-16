use petgraph::graph::UnGraph;

use crate::ast::Equation;

#[derive(Debug, Clone)]
pub struct EquationGraph {
    pub graph: UnGraph<Equation, Vec<Equation>>,
}

impl EquationGraph {
    pub fn new() -> EquationGraph {
        EquationGraph {
            graph: Default::default(),
        }
    }
}
