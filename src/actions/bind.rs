use itertools::Itertools;

use crate::{
    ast::{product::Product, Element, Node, NodeOrExpression},
    tokenizer::Number,
};

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum BindResult {
    Multiply(Element),
    NotOk,
    Ok,
}

pub trait Bind {
    fn bind(&self, instructions: &Self) -> BindResult;
}

impl Bind for Element {
    fn bind(&self, instructions: &Element) -> BindResult {
        let sign = self.sign * instructions.sign;

        match &instructions.node_or_expression {
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
                match &self.node_or_expression {
                    NodeOrExpression::Node(_) => todo!(),
                    NodeOrExpression::Expression(self_expr) => {
                        let strings = i_expr
                            .products
                            .iter()
                            .map(|i_product| {
                                self_expr
                                    .products
                                    .iter()
                                    .map(|self_product| {
                                        let result = self_product.bind(i_product);
                                        (
                                            format!(
                                                "instructions: {}, self: {}",
                                                i_product.to_string(),
                                                self_product.to_string()
                                            ),
                                            result,
                                        )
                                    })
                                    .collect_vec()
                            })
                            .collect_vec();

                        let a = self_expr.products.len();
                        let b = i_expr.products.len();

                        let indices2d = (0..a)
                            .collect_vec()
                            .into_iter()
                            .permutations(b)
                            .collect_vec();

                        /* println!("{:#?}\n", strings);
                        println!("{:#?}\n", indices2d);

                        for indices in indices2d {
                            for (index_pos, index) in indices.into_iter().enumerate() {
                                println!("{}", strings[index_pos][index]);
                            }

                            println!("\n{}\n", "-".repeat(80));
                        } */
                    }
                };

                BindResult::Ok
            }
        }
    }
}

impl Bind for Product {
    fn bind(&self, instructions: &Self) -> BindResult {
        println!("{}\t\t\t{}", self, instructions);

        for self_elem in self.numerator.iter() {
            for instr_elem in instructions.numerator.iter() {
                self_elem.bind(instr_elem);
            }
        }

        BindResult::Ok
    }
}
