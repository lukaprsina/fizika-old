pub struct Context {
    pub expressions: Vec<Expression>,
    pub equations: Vec<Equation>,
}

pub struct Equation {
    pub items: Expression,
}

pub struct Expression {
    pub products: Vec<Product>,
}

pub enum Sign {
    Positive,
    Negative,
}

pub enum EquationSide {
    Left,
    Right,
}

pub enum Node {
    Number(f64),
    Variable(String),
    Power {
        base: Expression,
        power: Expression,
    },
    Function {
        name: String,
        arguments: Vec<Expression>,
    },
}

pub enum NodeOrExpression {
    Node(Node),
    Expression(Expression),
}

pub struct Product {
    pub sign: Sign,
    pub side: Option<EquationSide>,
    pub top: Vec<NodeOrExpression>,
    pub bottom: Vec<NodeOrExpression>,
}
