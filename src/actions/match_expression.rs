#![allow(unused_variables)]

use crate::ast::{product::Product, Element, NodeOrExpression};

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
                    /* if Node::is_same(node_instr, node_self, names) {
                        if self.sign == instructions.sign {
                            BindResult::Ok
                        } else {
                            BindResult::Inverse
                        }
                    } else {
                        BindResult::NotOk
                    } */
                    BindResult::NotOk
                }
                NodeOrExpression::Expression(expr_self) => BindResult::NotOk,
            },
            NodeOrExpression::Expression(expr_instr) => match &self.node_or_expression {
                NodeOrExpression::Node(node_self) => BindResult::NotOk,
                NodeOrExpression::Expression(expr_self) => {
                    for product_instr in expr_instr.products.iter() {
                        for product_self in expr_self.products.iter() {
                            let result = product_self.bind(product_instr);
                        }
                    }
                    BindResult::Ok
                }
            },
        }
    }
}

impl Match for Product {
    type Instructions = Product;

    fn bind(&self, instructions: &Self::Instructions) -> BindResult {
        fn side(side_self: &Vec<Element>, side_instr: &Vec<Element>) {}

        side(&self.numerator, &instructions.numerator);
        side(&self.denominator, &instructions.denominator);

        BindResult::Ok
    }
}
