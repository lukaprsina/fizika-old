use math_eval::{Expression, Node, NodeOrExpression, Product, Sign};

fn main() {
    // x + 2^(x/(a+b)) - 3^2 + 1/(2 + x)
    let a = Expression {
        products: vec![
            Product {
                sign: Sign::Positive,
                top: vec![NodeOrExpression::Expression(Expression {
                    products: vec![
                        Product {
                            sign: Sign::Positive,
                            top: vec![NodeOrExpression::Node(Node::Variable("x".to_string()))],
                            bottom: vec![],
                        },
                        Product {
                            sign: Sign::Positive,
                            top: vec![NodeOrExpression::Node(Node::Power {
                                base: Expression {
                                    products: vec![Product {
                                        sign: Sign::Positive,
                                        top: vec![NodeOrExpression::Node(Node::Number(2.0))],
                                        bottom: vec![],
                                    }],
                                },
                                power: Expression {
                                    products: vec![Product {
                                        sign: Sign::Positive,
                                        top: vec![NodeOrExpression::Node(Node::Variable(
                                            "x".to_string(),
                                        ))],
                                        bottom: vec![NodeOrExpression::Expression(Expression {
                                            products: vec![
                                                Product {
                                                    sign: Sign::Positive,
                                                    top: vec![NodeOrExpression::Node(
                                                        Node::Variable("a".to_string()),
                                                    )],
                                                    bottom: vec![],
                                                },
                                                Product {
                                                    sign: Sign::Positive,
                                                    top: vec![NodeOrExpression::Node(
                                                        Node::Variable("b".to_string()),
                                                    )],
                                                    bottom: vec![],
                                                },
                                            ],
                                        })],
                                    }],
                                },
                            })],
                            bottom: vec![],
                        },
                    ],
                })],
                bottom: vec![],
            },
            // x + 2^(x/(a+b)) - 3^2 + 1/(2 + x)
            Product {
                sign: Sign::Negative,
                top: vec![NodeOrExpression::Node(Node::Power {
                    base: Expression {
                        products: vec![Product {
                            sign: Sign::Positive,
                            top: vec![NodeOrExpression::Node(Node::Number(3.0))],
                            bottom: vec![],
                        }],
                    },
                    power: Expression {
                        products: vec![Product {
                            sign: Sign::Positive,
                            top: vec![NodeOrExpression::Node(Node::Number(2.0))],
                            bottom: vec![],
                        }],
                    },
                })],
                bottom: vec![],
            },
            // x + 2^(x/(a+b)) - 3^2 + 1/(2 + x)
            Product {
                sign: Sign::Positive,
                top: vec![NodeOrExpression::Node(Node::Number(1.0))],
                bottom: vec![NodeOrExpression::Expression(Expression {
                    products: vec![Product {
                        sign: Sign::Positive,
                        top: vec![NodeOrExpression::Expression(Expression {
                            products: vec![
                                Product {
                                    sign: Sign::Positive,
                                    top: vec![NodeOrExpression::Node(Node::Number(2.0))],
                                    bottom: vec![],
                                },
                                Product {
                                    sign: Sign::Positive,
                                    top: vec![NodeOrExpression::Node(Node::Variable(
                                        "x".to_string(),
                                    ))],
                                    bottom: vec![],
                                },
                            ],
                        })],
                        bottom: vec![],
                    }],
                })],
            },
        ],
    };

    println!("{}", a);
    println!("x + 2^(x/(a+b)) - 3^2 + 1/(2 + x)");
}
