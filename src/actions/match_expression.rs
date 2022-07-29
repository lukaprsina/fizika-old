use super::is_same::IsSame;
use crate::ast::{Element, Node, NodeOrExpression};
use crate::tokenizer::Number;

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum BindResult {
    Ok,
    Inverse,
    NotOk,
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
                NodeOrExpression::Node(node_self) => {
                    if Node::is_same(node_instr, node_self) {
                        if self.sign == instructions.sign {
                            BindResult::Ok
                        } else {
                            BindResult::Inverse
                        }
                    } else {
                        BindResult::NotOk
                    }
                }
                NodeOrExpression::Expression(expr_self) => {
                    // a
                    BindResult::Ok
                }
            },
            NodeOrExpression::Expression(expr_instr) => {
                // a
                BindResult::Ok
            }
        }
    }
}
