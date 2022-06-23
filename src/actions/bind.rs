use itertools::Itertools;

use crate::{
    ast::{Element, Node, NodeOrExpression},
    tokenizer::Number,
};

pub trait Bind {
    fn bind(&self, other: &Element) -> BindResult;
}

pub enum BindResult {
    Multiply(Element),
    Ok,
}

impl Bind for Element {
    fn bind(&self, instructions: &Element) -> BindResult {
        let sign = self.sign * instructions.sign;

        match &self.node_or_expression {
            NodeOrExpression::Node(instructions_sign) => match &self.node_or_expression {
                NodeOrExpression::Node(self_node) => {
                    if instructions_sign == self_node {
                        if self.sign == instructions.sign {
                            BindResult::Ok
                        } else {
                            BindResult::Multiply(Element::new(
                                sign,
                                NodeOrExpression::Node(Node::Number(Number::Int(1))),
                            ))
                        }
                    } else {
                        BindResult::Multiply(instructions.clone() / self.clone())
                    }
                }
                NodeOrExpression::Expression(..) => {
                    BindResult::Multiply(instructions.clone() / self.clone())
                }
            },
            NodeOrExpression::Expression(i_expr) => {
                for products in i_expr.products.iter().permutations(i_expr.products.len()) {
                    println!("New iteration:\n{:#?}\n\n", products);
                }
                BindResult::Ok
            }
        }
    }
}
