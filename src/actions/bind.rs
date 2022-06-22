use crate::{
    ast::{Element, Node, NodeOrExpression, Sign},
    tokenizer::Number,
};

pub trait Bind {
    fn bind(&self, other: &Element) -> BindResult;
}

pub enum BindResult {
    Multiply(Element),
    Ok,
    NotOk,
}

impl Bind for Element {
    fn bind(&self, other: &Element) -> BindResult {
        match &self.node_or_expression {
            NodeOrExpression::Node(node) => match &self.node_or_expression {
                NodeOrExpression::Node(self_node) => {
                    // TODO:
                    if node == self_node {
                        if self.sign == other.sign {
                            BindResult::Ok
                        } else {
                            BindResult::Multiply(Element::new(
                                Sign::Negative,
                                NodeOrExpression::Node(Node::Number(Number::Int(1))),
                            ))
                        }
                    } else {
                        BindResult::NotOk
                    }
                }
                NodeOrExpression::Expression(self_expr) => todo!(),
            },
            NodeOrExpression::Expression(expression) => todo!(),
        }
    }
}
