use math_eval::{Node, Operator};

fn main() {
    // (6 + a)^2
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

    // f(x) = log(2, x) + 3x - 2
    let _ = Node::Binary {
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

    println!("{}", a.eval());
}
