use math_eval::{
    new::Item, EquationSide, Expression, ExpressionType, Node, Operator, Product, Sign,
};

fn main() {
    // (6 + a)^2
    let _a = Node::Binary {
        op: Operator::Power,
        lhs: Box::new(Node::Binary {
            op: Operator::Plus,
            lhs: Box::new(Node::Number(6.)),
            rhs: Box::new(Node::Number(1.)),
            // rhs: Box::new(Node::Variable("a".to_string())),
        }),
        rhs: Box::new(Node::Number(2.)),
    };
    // println!("{}", a.eval());

    // f(x) = log(2, x) + 3x - 2
    let _b = Node::Binary {
        op: Operator::Equals,
        lhs: Box::new(Node::Function {
            name: "f".to_string(),
            args: vec![Node::Variable("x".to_string())],
        }),
        rhs: Box::new(Node::Binary {
            op: Operator::Minus,
            lhs: Box::new(Node::Binary {
                op: Operator::Plus,
                lhs: Box::new(Node::Function {
                    name: "log".to_string(),
                    args: vec![Node::Number(2.), Node::Variable("x".to_string())],
                }),
                rhs: Box::new(Node::Binary {
                    op: Operator::Times,
                    lhs: Box::new(Node::Number(3.)),
                    rhs: Box::new(Node::Variable("x".to_string())),
                }),
            }),
            rhs: Box::new(Node::Number(2.)),
        }),
    };

    // println!("{}", b.eval());

    // (6 + a)^2
    /* let _c = Expression {
        expression_type: ExpressionType::Expression,
        children: vec![Product {
            sign: Sign::Plus,
            side: EquationSide::Left,
            top: vec![Item::Power {
                base: Expression {
                    expression_type: ExpressionType::Expression,
                    children: vec![
                        Product {
                            sign: Sign::Plus,
                            side: EquationSide::Left,
                            top: vec![Item::Number(6.)],
                            bottom: vec![],
                        },
                        Product {
                            sign: Sign::Plus,
                            side: EquationSide::Left,
                            top: vec![Item::Variable("a".to_string())],
                            bottom: vec![],
                        },
                    ],
                },
                power: Expression {
                    expression_type: ExpressionType::Expression,
                    children: vec![Product {
                        sign: Sign::Plus,
                        side: EquationSide::Left,
                        top: vec![Item::Number(2.)],
                        bottom: vec![],
                    }],
                },
            }],
            bottom: vec![],
        }],
    }; */

    // f(x) = log(2, x)/(5-x) - (3/4)x - 2
    let d = Expression {
        expression_type: ExpressionType::Equation,
        children: vec![
            Product {
                sign: Sign::Plus,
                side: EquationSide::Left,
                top: vec![Item::Function {
                    name: "f".to_string(),
                    arguments: vec![Item::Variable("x".to_string())],
                }],
                bottom: vec![],
            },
            Product {
                sign: Sign::Minus,
                side: EquationSide::Right,
                top: vec![Item::Function {
                    name: "log".to_string(),
                    arguments: vec![Item::Number(2.), Item::Variable("x".to_string())],
                }],
                bottom: vec![Item::Number(5.), Item::Variable("x".to_string())],
            },
        ],
    };

    println!("{}", d);
}
