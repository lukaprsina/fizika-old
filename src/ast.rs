pub enum Operator {
    Plus,
    Minus,
    Times,
    Divide,
    Modulo,
    Power,    
}

pub enum Node {
    Int(f64),
    Unary {
        op: Operator,
        child: Box<Node>,
    },
    Binary {
        op: Operator,
        lhs: Box<Node>,
        rhs: Box<Node>,
    },
}