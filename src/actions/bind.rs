#![allow(unused_variables)]

use crate::ast::{product::Product, Element, Node, NodeOrExpression};

pub trait Bind {
    fn bind(&self, instructions: &Self);
}

// assume element is analyzed
impl Bind for Element {
    fn bind(&self, instructions: &Element) {
        if let (Some(i_cache), Some(s_cache)) = (&instructions.cache, &self.cache) {
            let contains_functions = i_cache
                .functions
                .iter()
                .all(|item| s_cache.functions.contains(item));

            let contains_variables = i_cache
                .variables
                .iter()
                .all(|item| s_cache.variables.contains(item));

            if !contains_functions || !contains_variables {
                return;
            }

            match &instructions.node_or_expression {
                NodeOrExpression::Node(i_node) => match &self.node_or_expression {
                    NodeOrExpression::Node(s_node) => {
                        if !matches!(s_node, i_node) {
                            return;
                        }

                        match (&s_node, &i_node) {
                            (Node::Number(s_number), Node::Number(i_number)) => todo!(),
                            (Node::Variable(s_variable), Node::Variable(i_variable)) => todo!(),
                            (
                                Node::Power {
                                    base: s_base,
                                    power: s_power,
                                },
                                Node::Power {
                                    base: i_base,
                                    power: i_power,
                                },
                            ) => {
                                todo!()
                            }
                            (
                                Node::Modulo {
                                    lhs: s_lhs,
                                    rhs: s_rhs,
                                },
                                Node::Modulo {
                                    lhs: i_lhs,
                                    rhs: i_rhs,
                                },
                            ) => {
                                todo!()
                            }
                            (
                                Node::Factorial { child: s_child },
                                Node::Factorial { child: i_child },
                            ) => {
                                todo!()
                            }
                            (
                                Node::Function {
                                    name: s_name,
                                    arguments: s_arguments,
                                },
                                Node::Function {
                                    name: i_name,
                                    arguments: i_arguments,
                                },
                            ) => todo!(),
                            _ => panic!("Nodes do not match, but should (when binding)"),
                        }
                    }
                    NodeOrExpression::Expression(s_expression) => {}
                },
                NodeOrExpression::Expression(i_expr) => match &self.node_or_expression {
                    NodeOrExpression::Node(s_node) => todo!(),
                    NodeOrExpression::Expression(s_expression) => todo!(),
                },
            }
        } else {
            panic!("Tried to bind element which is not analyzed");
        }
    }
}

impl Bind for Product {
    fn bind(&self, instructions: &Self) {}
}
