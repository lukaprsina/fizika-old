use itertools::Itertools;

use crate::{
    ast::{product::Product, Element, Node, NodeOrExpression},
    tokenizer::Number,
};

pub trait Bind {
    type Instructions;
    fn bind(&self, other: &Self::Instructions) -> BindResult;
}

pub enum BindResult {
    Multiply(Element),
    Ok,
}

impl Bind for Element {
    type Instructions = Element;

    fn bind(&self, instructions: &Element) -> BindResult {
        let sign = self.sign * instructions.sign;
        println!("Bind!:\nSelf:\n{}\nInstructions:\n{}\n", self, instructions);

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
                for products in i_expr.products.iter().permutations(i_expr.products.len()) {
                    println!("New iteration:\n{:#?}\n\n", products);

                    let result = match &self.node_or_expression {
                        NodeOrExpression::Node(self_node) => {
                            BindResult::Multiply(instructions.clone() / self.clone())
                        }
                        NodeOrExpression::Expression(self_expr) => {
                            for (self_product, i_product) in self_expr.products.iter().zip(products)
                            {
                                self_product.bind(&i_product);
                            }
                            BindResult::Ok
                        }
                    };
                }
                BindResult::Ok
            }
        }
    }
}

impl Bind for Product {
    type Instructions = Product;

    fn bind(&self, instr: &Product) -> BindResult {
        // TODO: each side should be compared, not by enumerate
        println!("Bind product (self, instr): {} {}", self, instr);

        for (self_side, instr_side) in [&self.numerator, &self.denominator]
            .into_iter()
            .zip([&instr.numerator, &instr.denominator])
        {
            for zipped_side in self_side
                .iter()
                .zip(instr_side)
                .permutations(self_side.len().max(instr_side.len()))
            {
                println!("Zipped [self, instructions]:\n{:#?}\n\n", zipped_side);

                for (self_perm, instr_perm) in zipped_side {
                    //
                    self_perm.bind(instr_perm);
                }
            }
        }

        BindResult::Ok
    }
}
