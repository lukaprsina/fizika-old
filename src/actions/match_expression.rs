use crate::ast::{Element, NodeOrExpression};

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum BindResult {
    Multiply(Element),
    Ok,
}

pub trait Match {
    type Instructions;
    fn bind(&self, instructions: &Self::Instructions) -> BindResult;
}

impl Match for Element {
    type Instructions = Element;

    fn bind(&self, instructions: &Element) -> BindResult {
        let sign = self.sign * instructions.sign;

        match &instructions.node_or_expression {
            NodeOrExpression::Node(node_instr) => match &self.node_or_expression {
                NodeOrExpression::Node(node_self) => todo!(),
                NodeOrExpression::Expression(expr_self) => todo!(),
            },
            NodeOrExpression::Expression(expr_instr) => todo!(),
        }

        BindResult::Ok
    }
}
