use math_eval::{
    new::Item, EquationSide, Expression, ExpressionType, Node, Operator, Product, Sign,
};

fn main() {
    /* // (6 + a)^2
    let a = Node::Binary {
        op: Operator::Power,
        lhs: Box::new(Node::Binary {
            op: Operator::Plus,
            lhs: Box::new(Node::Number(6.)),
            rhs: Box::new(Node::Number(1.)),
            // rhs: Box::new(Node::Variable("a".to_string())),
        }),
        rhs: Box::new(Node::Number(2.)),
    };
    println!("{}", a.eval());

    // f(x) = log(2, x) + 3x - 2
    let b = Node::Binary {
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

    println!("{}", b.eval()); */

    // (6 + a)^2
    let c = Expression {
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
    };

    println!("{}", c);
}
