use std::fmt::Display;

#[derive(Debug)]
pub struct Context {
    pub expressions: Vec<Expression>,
    pub equations: Vec<Equation>,
}

#[derive(Debug)]
pub enum EqualSign {
    Equal,
    NotEqual,
    LessThan,
    LessThanOrEqual,
    GreaterThanOrEqual,
    GreaterThan,
}

impl Display for EqualSign {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let result = match self {
            EqualSign::Equal => "=",
            EqualSign::NotEqual => "!=",
            EqualSign::LessThan => "<",
            EqualSign::LessThanOrEqual => "<=",
            EqualSign::GreaterThanOrEqual => ">=",
            EqualSign::GreaterThan => ">",
        };

        // println!("Equal sign: {}", result);
        write!(f, "{}", result)
    }
}

#[derive(Debug)]
pub struct Equation {
    pub lhs: Expression,
    pub sign: EqualSign,
    pub rhs: Expression,
}

impl Display for Equation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();

        result += &format!("{} {} {}", self.lhs, self.sign, self.rhs);

        // println!("Equation: {}", result);
        write!(f, "{}", result)
    }
}

trait ShouldBeParenthesized {
    fn should_be_parenthesized(&self) -> bool;
}

#[derive(Debug)]
pub struct Expression {
    pub products: Vec<Product>,
}

impl ShouldBeParenthesized for Expression {
    fn should_be_parenthesized(&self) -> bool {
        // self.products.len() > 1
        true
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();

        if self.products.len() > 1 {
            result.push('(');
        }

        for (position, product) in self.products.iter().enumerate() {
            let mut open = false;
            match product.sign {
                Sign::Positive => {
                    if position != 0 {
                        result += "+ ";
                    }
                }
                Sign::Negative => {
                    result += "- ";
                    if product.top.len() > 1 {
                        open = true;
                        result.push('(');
                    }
                }
            }

            result += &product.to_string();

            if open {
                result.push(')');
            }

            if position != self.products.len() - 1 {
                result.push(' ');
            }
        }

        if self.products.len() > 1 {
            result.push(')');
        }

        // println!("Expression: {}", result);
        write!(f, "{}", result)
    }
}

#[derive(Debug)]
pub enum Sign {
    Positive,
    Negative,
}

#[derive(Debug)]
pub enum EquationSide {
    Left,
    Right,
}

#[derive(Debug)]
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

impl ShouldBeParenthesized for Node {
    fn should_be_parenthesized(&self) -> bool {
        match self {
            Node::Power { .. } => true,
            _ => false,
        }
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();
        match self {
            Node::Number(number) => {
                result += &number.to_string();
            }
            Node::Variable(variable) => {
                result += &variable;
            }
            Node::Power { base, power } => {
                let mut parenethesis = false;

                for expression in [&base.products, &power.products] {
                    if parenethesis {
                        continue;
                    }

                    if expression.len() > 1 {
                        parenethesis = true;
                    } else if expression.len() == 1 {
                        for side in [&expression[0].top, &expression[0].bottom] {
                            if side.len() > 1 {
                                parenethesis = true;
                            } else if side.len() == 1 {
                                match &side[0] {
                                    NodeOrExpression::Node(_) => (),
                                    NodeOrExpression::Expression(expression) => {
                                        parenethesis |= expression.should_be_parenthesized();
                                    }
                                }
                            }
                        }
                    }
                }

                result += &format!(
                    "{}^{}{}{}",
                    base,
                    if parenethesis { "(" } else { "" },
                    power,
                    if parenethesis { ")" } else { "" }
                );
            }
            Node::Function { name, arguments } => {
                result += &format!("{}(", name);
                for (index, argument) in arguments.iter().enumerate() {
                    result += &format!("{}", argument);
                    if index < arguments.len() - 1 {
                        result += ", ";
                    }
                }
                result += ")";
            }
        }

        // println!("Node: {}", result);
        write!(f, "{}", result)
    }
}

#[derive(Debug)]
pub enum NodeOrExpression {
    Node(Node),
    Expression(Expression),
}

impl Display for NodeOrExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let result = match self {
            NodeOrExpression::Node(node) => node.to_string(),
            NodeOrExpression::Expression(expression) => expression.to_string(),
        };

        // println!("NodeOrExpression: {}", result);
        write!(f, "{}", result)
    }
}

#[derive(Debug)]
pub struct Product {
    pub sign: Sign,
    pub top: Vec<NodeOrExpression>,
    pub bottom: Vec<NodeOrExpression>,
}

impl Display for Product {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();

        let mut top_side = true;
        if self.top.is_empty() {
            result.push('1');
        }

        for side in [&self.top, &self.bottom] {
            if side.len() > 1 {
                result.push('(');
            }

            for node_or_expression in side {
                result += &node_or_expression.to_string();
            }

            if side.len() > 1 {
                result.push('(');
            }

            if top_side && !self.bottom.is_empty() {
                result += "/";
                top_side = false;
            }
        }

        // println!("Product: {}", result);
        write!(f, "{}", result)
    }
}
