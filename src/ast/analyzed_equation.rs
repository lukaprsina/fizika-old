use std::{collections::HashMap, hash::Hash, ops::AddAssign};

use super::{
    context::Context, equation::EquationSide, product::Product, Element, Expression, Node,
    NodeOrExpression,
};

#[derive(Debug, Clone)]
pub struct AnalyzedElement {
    pub element: Element,
    pub info: ExpressionInfo,
}

impl EquationSide {
    // TODO: ignores operations
    pub fn analyze(self, context: &Context) -> AnalyzedElement {
        let mut info = ExpressionInfo::default();
        self.element.node_or_expression.analyze(context, &mut info);
        println!("{:#?}", info);

        AnalyzedElement {
            element: self.element,
            info,
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct ExpressionInfo {
    variables: HashMap<String, usize>,
    functions: HashMap<String, usize>,
}

pub trait Analyze {
    fn analyze(&self, context: &Context, info: &mut ExpressionInfo);
}

impl Analyze for NodeOrExpression {
    fn analyze(&self, context: &Context, info: &mut ExpressionInfo) {
        match self {
            NodeOrExpression::Node(node) => node.analyze(context, info),
            NodeOrExpression::Expression(expression) => expression.analyze(context, info),
        }
    }
}

impl Analyze for Node {
    fn analyze(&self, _: &Context, info: &mut ExpressionInfo) {
        // Analyze everything
        match self {
            Node::Variable(name) => match info.variables.get_mut(name) {
                Some(size) => *size += 1,
                None => {
                    info.variables.insert(name.to_string(), 1);
                }
            },
            Node::Unit(name) => match info.variables.get_mut(name) {
                Some(size) => *size += 1,
                None => {
                    info.variables.insert(name.to_string(), 1);
                }
            },
            Node::Function { name, arguments } => match info.functions.get_mut(name) {
                Some(size) => *size += 1,
                None => {
                    info.functions.insert(name.to_string(), 1);
                }
            },
            _ => (),
        }
    }
}

impl Analyze for Expression {
    fn analyze(&self, context: &Context, info: &mut ExpressionInfo) {
        for product in self.products.iter() {
            product.analyze(context, info);
        }
    }
}

impl Analyze for Product {
    fn analyze(&self, context: &Context, info: &mut ExpressionInfo) {
        for side in [&self.numerator, &self.denominator].into_iter() {
            for element in side {
                element.analyze(context, info);
            }
        }
    }
}

impl Analyze for Element {
    fn analyze(&self, context: &Context, info: &mut ExpressionInfo) {
        self.node_or_expression.analyze(context, info);
    }
}
