use std::collections::HashMap;

use super::{
    context::Context, product::Product, Element, Equation, Expression, Node, NodeOrExpression,
};

#[derive(Debug)]
pub struct AnalyzedExpression {
    pub node_or_expression: NodeOrExpression,
    pub variables: HashMap<String, usize>,
}

pub trait Analyze {
    fn analyze(&self, context: &Context, variables: &mut HashMap<String, usize>) {}
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
    fn analyze(&self, context: &Context, variables: &mut HashMap<String, usize>) {
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
        for product in self.products {
            product.analyze(context, variables);
        }
    }
}

impl Analyze for Product {
    fn analyze(&self, context: &Context, variables: &mut HashMap<String, usize>) {
        for side in [self.numerator, self.denominator] {
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
