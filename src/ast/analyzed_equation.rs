use std::collections::HashMap;

use super::{
    context::Context, equation::EquationSide, product::Product, Element, Expression, Node,
    NodeOrExpression,
};

#[derive(Debug, Clone)]
pub struct AnalyzedElement {
    pub element: Element,
    pub variables: HashMap<String, usize>,
}

impl EquationSide {
    // TODO: ignores operations
    pub fn analyze(self, context: &Context) -> AnalyzedElement {
        let mut variables: HashMap<String, usize> = HashMap::new();

        self.element
            .node_or_expression
            .analyze(context, &mut variables);

        AnalyzedElement {
            element: self.element,
            variables,
        }
    }
}

pub trait Analyze {
    fn analyze(&self, context: &Context, variables: &mut HashMap<String, usize>);
}

impl Analyze for NodeOrExpression {
    fn analyze(&self, context: &Context, variables: &mut HashMap<String, usize>) {
        match self {
            NodeOrExpression::Node(node) => node.analyze(context, variables),
            NodeOrExpression::Expression(expression) => expression.analyze(context, variables),
        }
    }
}

impl Analyze for Node {
    fn analyze(&self, _: &Context, variables: &mut HashMap<String, usize>) {
        if let Node::Variable(name) = self {
            match variables.get_mut(name) {
                Some(size) => *size += 1,
                None => {
                    variables.insert(name.to_string(), 1);
                }
            }
        }
    }
}

impl Analyze for Expression {
    fn analyze(&self, context: &Context, variables: &mut HashMap<String, usize>) {
        for product in self.products.iter() {
            product.analyze(context, variables);
        }
    }
}

impl Analyze for Product {
    fn analyze(&self, context: &Context, variables: &mut HashMap<String, usize>) {
        for side in [&self.numerator, &self.denominator].into_iter() {
            for element in side {
                element.analyze(context, variables);
            }
        }
    }
}

impl Analyze for Element {
    fn analyze(&self, context: &Context, variables: &mut HashMap<String, usize>) {
        self.node_or_expression.analyze(context, variables);
    }
}
