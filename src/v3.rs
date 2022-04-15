use std::fmt::Display;

#[derive(Debug)]
pub struct Context {
    pub expressions: Vec<Expression>,
    pub equations: Vec<Equation>,
}

#[derive(Debug)]
pub struct Equation {
    pub items: Expression,
}

#[derive(Debug)]
pub struct Expression {
    pub products: Vec<Product>,
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();
        let mut left: Vec<&Product> = Vec::new();
        let mut right: Vec<&Product> = Vec::new();

        for product in &self.products {
            if let Some(side) = &product.side {
                match side {
                    EquationSide::Left => left.push(product),
                    EquationSide::Right => right.push(product),
                }
            }
        }

        let mut add_products_from_side = |side: &[&Product]| {
            for (position, product) in side.iter().enumerate() {
                match product.sign {
                    Sign::Positive => {
                        if position != 0 {
                            result += " + "
                        }
                    }
                    Sign::Negative => result += " - ",
                }

                result.push_str(&product.to_string());
            }
        };

        if left.len() == 0 && right.len() == 0 {
            self.products.iter().for_each(|product| {
                left.push(product);
            });
            add_products_from_side(left.as_slice());
        } else {
            add_products_from_side(left.as_slice());
            add_products_from_side(right.as_slice());
        }

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
                result.push_str(&number.to_string());
            }
            Node::Variable(variable) => {
                result.push_str(&variable);
            }
            Node::Power { base, power } => {
                result.push_str(&base.to_string());
                result.push_str("^");
                result.push_str(&power.to_string());
            }
            Node::Function { name, arguments } => {
                result.push_str(&name);
                result.push_str("(");
                for argument in arguments {
                    result.push_str(&argument.to_string());
                    result.push_str(",");
                }
                result.pop();
                result.push_str(")");
            }
        }
        write!(f, "{}", result)
    }
}

#[derive(Debug)]
pub enum NodeOrExpression {
    Node(Node),
    Expression(Expression),
}

#[derive(Debug)]
pub struct Product {
    pub sign: Sign,
    pub side: Option<EquationSide>,
    pub top: Vec<NodeOrExpression>,
    pub bottom: Vec<NodeOrExpression>,
}

impl Display for Product {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();

        for side in [&self.top, &self.bottom] {
            // let guard = ParenthesesGuard::new(side.len(), &result);
            for node_or_expression in side {
                // let guard = ParenthesesGuard::new(side.len(), &result);

                match node_or_expression {
                    NodeOrExpression::Node(node) => {
                        result.push_str(&node.to_string());
                    }
                    NodeOrExpression::Expression(expression) => {
                        result.push_str(&expression.to_string());
                    }
                }
            }

            if !self.bottom.is_empty() {
                result.push_str("/");
            }
        }

        write!(f, "{}", result)
    }
}
