use std::collections::HashSet;

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
    pub fn analyze(mut self, context: &Context) -> AnalyzedElement {
        let mut info = ExpressionInfo::default();
        let mut is_number = false;

        self.element
            .node_or_expression
            .analyze(context, &mut info, &mut is_number);

        println!("{:#?}", info);

        AnalyzedElement {
            element: self.element,
            info,
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct ExpressionInfo {
    variables: HashSet<String>,
    functions: HashSet<String>,
}

pub trait Analyze {
    fn analyze(&mut self, context: &Context, info: &mut ExpressionInfo, is_number: &mut bool);
}

impl Analyze for NodeOrExpression {
    fn analyze(&mut self, context: &Context, info: &mut ExpressionInfo, is_number: &mut bool) {
        match self {
            NodeOrExpression::Node(node) => node.analyze(context, info, is_number),
            NodeOrExpression::Expression(expression) => {
                expression.analyze(context, info, is_number)
            }
        }
    }
}

impl Analyze for Node {
    fn analyze(&mut self, context: &Context, info: &mut ExpressionInfo, is_number: &mut bool) {
        match self {
            Node::Number(_) => *is_number = true,
            Node::Variable(variable) => {
                info.variables.insert(variable.to_string());
            }
            Node::Power { base, power } => {
                base.analyze(context, info, is_number);
                power.analyze(context, info, is_number);
            }
            Node::Modulo { lhs, rhs } => {
                lhs.analyze(context, info, is_number);
                rhs.analyze(context, info, is_number);
            }
            Node::Factorial { child } => {
                child.analyze(context, info, is_number);
            }
            Node::Function { name, arguments } => {
                info.functions.insert(name.to_string());
                arguments
                    .iter_mut()
                    .for_each(|arg| arg.analyze(context, info, is_number))
            }
            _ => (),
        }
    }
}

impl Analyze for Expression {
    fn analyze(&mut self, context: &Context, info: &mut ExpressionInfo, is_number: &mut bool) {
        for product in self.products.iter_mut() {
            product.analyze(context, info, is_number);
        }
    }
}

impl Analyze for Product {
    fn analyze(&mut self, context: &Context, info: &mut ExpressionInfo, is_number: &mut bool) {
        for side in [&mut self.numerator, &mut self.denominator].into_iter() {
            for element in side {
                element.analyze(context, info, is_number);
            }
        }
    }
}

impl Analyze for Element {
    fn analyze(&mut self, context: &Context, info: &mut ExpressionInfo, is_number: &mut bool) {
        let mut is_number = false;
        self.node_or_expression
            .analyze(context, info, &mut is_number);
        self.is_number = is_number;
    }
}
