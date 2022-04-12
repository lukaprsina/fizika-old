pub enum Operator {
    Plus,
    Minus,
    Times,
    Divide,
    Modulo,
    Power,
    Equals,
    NotEquals,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
}

pub enum Node {
    Number(f64),
    Unary {
        op: Operator,
        child: Box<Node>,
    },
    Binary {
        op: Operator,
        lhs: Box<Node>,
        rhs: Box<Node>,
    },
    Variable(String),
    Function {
        name: String,
        args: Vec<Node>,
    },
}

impl Node {
    pub fn eval(&self) -> f64 {
        match self {
            Node::Number(n) => *n,
            Node::Unary { op, child } => match op {
                Operator::Plus => child.eval(),
                Operator::Minus => -child.eval(),
                Operator::Times => panic!("Times operator not implemented"),
                Operator::Divide => panic!("Divide operator not implemented"),
                Operator::Modulo => panic!("Modulo operator not implemented"),
                Operator::Power => panic!("Power operator not implemented"),
                Operator::Equals => todo!(),
                Operator::NotEquals => todo!(),
                Operator::LessThan => todo!(),
                Operator::LessThanOrEqual => todo!(),
                Operator::GreaterThan => todo!(),
                Operator::GreaterThanOrEqual => todo!(),
            },
            Node::Binary { op, lhs, rhs } => match op {
                Operator::Plus => lhs.eval() + rhs.eval(),
                Operator::Minus => lhs.eval() - rhs.eval(),
                Operator::Times => lhs.eval() * rhs.eval(),
                Operator::Divide => lhs.eval() / rhs.eval(),
                Operator::Modulo => lhs.eval() % rhs.eval(),
                Operator::Power => lhs.eval().powf(rhs.eval()),
                Operator::Equals => panic!("Equals operator not implemented"),
                Operator::NotEquals => panic!("NotEquals operator not implemented"),
                Operator::LessThan => panic!("LessThan operator not implemented"),
                Operator::LessThanOrEqual => panic!("LessThanOrEqual operator not implemented"),
                Operator::GreaterThan => panic!("GreaterThan operator not implemented"),
                Operator::GreaterThanOrEqual => {
                    panic!("GreaterThanOrEqual operator not implemented")
                }
            },
            Node::Variable(name) => panic!("Variable {} not implemented", name),
            Node::Function { name, args: _args } => panic!("Function {} not implemented", name),
        }
    }
}
