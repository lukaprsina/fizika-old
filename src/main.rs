use math_eval::{tokenizer::parser::tokenize, Expression, Node, NodeOrExpression, Product, Sign};

fn main() {
    let _a = Expression {
        products: vec![
            Product {
                sign: Sign::Negative,
                top: vec![],
                bottom: vec![NodeOrExpression::Node(Node::Variable("x".to_string()))],
            },
            Product {
                sign: Sign::Positive,
                top: vec![NodeOrExpression::Expression(Expression {
                    products: vec![Product {
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
                                    top: vec![
                                        NodeOrExpression::Node(Node::Number(12.2)),
                                        NodeOrExpression::Node(Node::Variable("x".to_string())),
                                        NodeOrExpression::Node(Node::Number(6.)),
                                        NodeOrExpression::Node(Node::Number(2.)),
                                    ],
                                    bottom: vec![NodeOrExpression::Expression(Expression {
                                        products: vec![
                                            Product {
                                                sign: Sign::Positive,
                                                top: vec![NodeOrExpression::Node(Node::Variable(
                                                    "a".to_string(),
                                                ))],
                                                bottom: vec![],
                                            },
                                            Product {
                                                sign: Sign::Positive,
                                                top: vec![NodeOrExpression::Node(Node::Variable(
                                                    "b".to_string(),
                                                ))],
                                                bottom: vec![],
                                            },
                                        ],
                                    })],
                                }],
                            },
                        })],
                        bottom: vec![],
                    }],
                })],
                bottom: vec![],
            },
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

    /* println!("{}", _a);
    println!("(- 1/x + 2^(x * 6 * 2/(a + b)) - 3^2 + 1/(2 + x))"); */

    let cases = vec![
        // "1/x + 2^(x * 6 * 2/(a + b)) - 3^2 + 1/(2 + x)",
        "1*m", "a/b",
    ];

    for case in cases {
        println!(
            "Case: {}\n{:?}\n\n{}\n",
            case,
            tokenize(case),
            "-".repeat(80)
        );
    }
}
