#[derive(Debug)]
pub enum ExpressionType {
    Expression,
    Equation,
    Ineqatuon,
}

#[derive(Debug)]
pub struct Expression {
    pub expression_type: ExpressionType,
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
    pub top: Vec<Item>,
    pub bottom: Vec<Item>,
}

#[derive(Debug)]
pub enum Item {
    Number(f64),
    Variable(String),
    Power { base: Expression, power: Expression },
    Function { name: String, arguments: Vec<Item> },
}

impl Expression {
    pub fn foo(&self) {
        let mut result: String;

        let left: Vec<Product>;
        let right: Vec<Product>;

        let product_str: String;

        for (position, product) in self.children.iter().enumerate() {
            // println!("Position {}\n{:#?}", position, child);
            match product.side {
                EquationSide::Left => (),
                EquationSide::Right => (),
            }
        }

        let equation_char = match self.expression_type {
            ExpressionType::Expression => None,
            ExpressionType::Equation => Some('='),
            ExpressionType::Ineqatuon => Some('<'),
        };

        println!("Eq char: {:?}\n\n", equation_char);
    }
}
