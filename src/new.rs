use std::fmt;

#[derive(Debug)]
pub enum EquationOperators {
    LessThan,
    LessThanOrEqual,
    Equal,
    GreaterThan,
    GreaterThanOrEqual,
    None,
}

#[derive(Debug)]
pub struct Equation {
    pub expressions: Vec<(Expression, EquationOperators)>,
}

#[derive(Debug)]
pub struct Expression {
    pub children: Vec<Product>,
}

#[derive(Debug)]
pub enum Sign {
    Plus,
    Minus,
}

#[derive(Debug)]
pub enum EquationSide {
    Left,
    Right,
}

#[derive(Debug)]
pub struct Product {
    pub sign: Sign,
    pub side: EquationSide,
    pub top: Vec<Expression>,
    pub bottom: Vec<Expression>,
}

#[derive(Debug)]
pub enum Item {
    Number(f64),
    Variable(String),
    Power { base: Expression, power: Expression },
    Function { name: String, arguments: Vec<Item> },
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut product_string = String::new();
        match self {
            Item::Number(number) => product_string += &number.to_string(),
            Item::Variable(variable_name) => product_string += &variable_name,
            Item::Power { base, power } => product_string += &format!("{}^{}", base, power),
            Item::Function { name, arguments } => {
                product_string += &format!("{}(", name);
                for (index, argument) in arguments.iter().enumerate() {
                    product_string += &format!("{}", argument);
                    if index < arguments.len() - 1 {
                        product_string += ", ";
                    }
                }
                product_string += ")";
            }
        };

        write!(f, "{}", product_string)
    }
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut left: Vec<&Product> = Vec::new();
        let mut right: Vec<&Product> = Vec::new();

        let mut result_string = String::new();

        for product in self.children.iter() {
            match product.side {
                EquationSide::Left => left.push(product),
                EquationSide::Right => right.push(product),
            }
        }

        for side in [left, right] {
            if side.len() > 1 {
                result_string += "(";
            }

            for (position, product) in side.iter().enumerate() {
                match product.sign {
                    Sign::Plus => {
                        if position != 0 {
                            result_string += " + "
                        }
                    }
                    Sign::Minus => result_string += " - ",
                }

                for expression in [&product.top, &product.bottom] {
                    if expression.len() > 1 {
                        result_string += "(";
                    }

                    for item in expression.iter() {
                        result_string += &item.to_string();
                    }

                    if expression.len() > 1 {
                        result_string += ")";
                    }
                }
            }

            if side.len() > 1 {
                result_string += ")";
            }

            match self.expression_type {
                ExpressionType::Equation => result_string += " = ",
                ExpressionType::Ineqatuon => result_string += " > ",
                ExpressionType::Expression => (),
            }
        }

        write!(f, "{}", result_string)
    }
}
