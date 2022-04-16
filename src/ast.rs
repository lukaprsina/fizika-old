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

#[derive(Debug)]
pub struct Expression {
    pub products: Vec<Product>,
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();

        for (position, product) in self.products.iter().enumerate() {
            let mut open = false;
            // result += " | ";
            match product.sign {
                Sign::Positive => {
                    if position != 0 {
                        result += " + ";
                    }
                }
                // TODO: - / when top empty, bottom not
                Sign::Negative => {
                    result += " - ";
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
                let mut open = false;
                // TODO: check for parentheses around both base and power
                // and theirs top and bottom

                if base.products.len() > 1 {
                    result.push('(');
                    open = true;
                } else if base.products.len() == 1 {
                    // match base.products[0].bottom {}
                }

                result += &format!("{}^{}", base, power);

                if open {
                    result.push(')');
                }
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
