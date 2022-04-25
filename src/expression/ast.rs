use std::fmt::Display;

#[derive(Debug)]
pub struct Context {
    pub expressions: Vec<Expression>,
    pub equations: Vec<Equation>,
}

#[derive(Debug)]
pub enum ComparisonSign {
    Equal,
    NotEqual,
    LessThan,
    LessThanOrEqual,
    GreaterThanOrEqual,
    GreaterThan,
}

impl Display for ComparisonSign {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let result = match self {
            ComparisonSign::Equal => "=",
            ComparisonSign::NotEqual => "!=",
            ComparisonSign::LessThan => "<",
            ComparisonSign::LessThanOrEqual => "<=",
            ComparisonSign::GreaterThanOrEqual => ">=",
            ComparisonSign::GreaterThan => ">",
        };

        // println!("Equal sign: {}", result);
        write!(f, "{}", result)
    }
}

#[derive(Debug)]
pub struct Equation {
    pub lhs: Expression,
    pub sign: ComparisonSign,
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

pub struct AnalyzedEquation {
    pub equation: Equation,
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
        self.products.len() > 1
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

impl Expression {
    pub fn new() -> Expression {
        Expression { products: vec![] }
    }
}

impl Default for Expression {
    fn default() -> Expression {
        Expression::new()
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
            Node::Power { base, power } => {
                base.should_be_parenthesized() || power.should_be_parenthesized()
            }
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
                result += variable;
            }
            Node::Power { base, power } => {
                let mut parenethesis = false;

                for expression in [&base.products, &power.products] {
                    match expression.len() {
                        2.. => {
                            parenethesis = true;
                            break;
                        }
                        1 => {
                            for side in [&expression[0].top, &expression[0].bottom] {
                                match side.len() {
                                    2.. => {
                                        parenethesis = true;
                                        break;
                                    }
                                    1 => {
                                        if let NodeOrExpression::Expression(expression) = &side[0] {
                                            if expression.should_be_parenthesized() {
                                                parenethesis = true;
                                                break;
                                            }
                                        }
                                    }
                                    _ => (),
                                }
                            }
                        }
                        _ => (),
                    };
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

impl NodeOrExpression {
    fn is_times_visible(&self, last: &NodeOrExpression) -> bool {
        match self {
            // last * thing
            NodeOrExpression::Node(node) => match node {
                Node::Number(_) => true,
                Node::Variable(_) => match last {
                    NodeOrExpression::Node(var_node) => {
                        !matches!(var_node, Node::Number(_) | Node::Variable(_))
                    }
                    // TODO: get first from expression
                    NodeOrExpression::Expression(_) => false,
                },
                Node::Power { .. } => true,
                Node::Function { .. } => true,
            },
            // TODO
            NodeOrExpression::Expression(_) => false,
        }
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

        let mut last: Option<&NodeOrExpression> = None;

        for side in [&self.top, &self.bottom] {
            if side.len() > 1 {
                result.push('(');
            }

            for node_or_expression in side {
                if let Some(last) = last {
                    if node_or_expression.is_times_visible(last) {
                        result += " * ";
                    }
                }
                result += &node_or_expression.to_string();
                last = Some(node_or_expression);
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

pub enum ExpressionOrEquation {
    Expression(Expression),
    Equation(Equation),
}
