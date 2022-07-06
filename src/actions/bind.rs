use crate::{
    ast::{
        context::Context, equation::EquationSide, product::Product, Element, Equation, Expression,
        Node, NodeOrExpression, Sign,
    },
    tokenizer::{Number, Operation},
};

pub trait Bind {
    type Instructions;
    fn bind(&self, other: &Self::Instructions) -> BindResult;
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
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
                match &self.node_or_expression {
                    NodeOrExpression::Node(self_node) => todo!(),
                    NodeOrExpression::Expression(self_expr) => {
                        for i_product in i_expr.products.iter() {
                            for self_product in self_expr.products.iter() {
                                self_product.bind(i_product);
                            }
                        }
                    }
                };

                BindResult::Ok
            }
        }
    }
}

impl Bind for Product {
    type Instructions = Product;

    fn bind(&self, instr: &Product) -> BindResult {
        println!("self: {}\ninstr: {}\n", self, instr);

        let mut context = Context::new();

        let equation = Equation::new(vec![
            EquationSide::new(
                Element::new(
                    Sign::Positive,
                    NodeOrExpression::Expression(Expression::new(vec![self.clone()])),
                ),
                Some(Operation::Equal),
            ),
            EquationSide::new(
                Element::new(
                    Sign::Positive,
                    NodeOrExpression::Expression(Expression::new(vec![instr.clone()])),
                ),
                Some(Operation::Equal),
            ),
        ]);

        let reference = context.add_equation(equation);

        BindResult::Ok
    }
}

/* for products in i_expr.products.iter().permutations(i_expr.products.len()) {
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
} */
