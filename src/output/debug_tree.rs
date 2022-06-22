use std::borrow::Cow;

use ptree::{Style, TreeItem};

use crate::ast::{equation::EquationSide, product::Product, Element, Equation, NodeOrExpression};

impl TreeItem for Equation {
    type Child = EquationSide;

    fn write_self<W: std::io::Write>(&self, f: &mut W, style: &Style) -> std::io::Result<()> {
        write!(f, "{}", style.paint("Equation"))
    }

    fn children(&self) -> std::borrow::Cow<[Self::Child]> {
        Cow::from(&self.sides)
    }
}

impl TreeItem for EquationSide {
    type Child = Product;

    fn write_self<W: std::io::Write>(&self, f: &mut W, style: &Style) -> std::io::Result<()> {
        match &self.operation {
            Some(operation) => write!(f, "{}", style.paint(format!("Side: {}", operation))),
            None => write!(f, "{}", style.paint("Side")),
        }
    }

    fn children(&self) -> Cow<[Self::Child]> {
        let v = match &self.element.node_or_expression {
            NodeOrExpression::Node(..) => {
                vec![Product::new(vec![self.element.clone()], vec![])]
            }
            NodeOrExpression::Expression(expression) => expression.products.clone(),
        };

        Cow::from(v)
    }
}

#[derive(Clone, Debug)]
pub struct RatioSides(pub String, pub Vec<Element>);

impl TreeItem for Product {
    type Child = RatioSides;

    fn write_self<W: std::io::Write>(&self, f: &mut W, style: &Style) -> std::io::Result<()> {
        write!(f, "{}", style.paint("Product"))
    }

    fn children(&self) -> std::borrow::Cow<[Self::Child]> {
        let v = vec![
            RatioSides("Numerator".to_string(), self.numerator.clone()),
            RatioSides("Denominator".to_string(), self.denominator.clone()),
        ];
        Cow::from(v)
    }
}

impl TreeItem for RatioSides {
    type Child = Element;

    fn write_self<W: std::io::Write>(&self, f: &mut W, style: &Style) -> std::io::Result<()> {
        write!(f, "{}", style.paint(&self.0))
    }

    fn children(&self) -> std::borrow::Cow<[Self::Child]> {
        Cow::from(&self.1)
    }
}

impl TreeItem for Element {
    type Child = Product;

    fn write_self<W: std::io::Write>(&self, f: &mut W, style: &Style) -> std::io::Result<()> {
        match &self.node_or_expression {
            NodeOrExpression::Node(node) => {
                write!(
                    f,
                    "{}",
                    style.paint(format!("Element: {} {}", self.sign, node))
                )
            }
            NodeOrExpression::Expression(_) => {
                write!(f, "{}", style.paint(format!("Element: {}", self.sign)))
            }
        }
    }

    fn children(&self) -> std::borrow::Cow<[Self::Child]> {
        let v = match &self.node_or_expression {
            NodeOrExpression::Node(..) => {
                vec![]
            }
            NodeOrExpression::Expression(expression) => expression.products.clone(),
        };

        Cow::from(v)
    }
}

/* impl TreeItem for NodeOrExpression {
    type Child = Product;

    fn write_self<W: std::io::Write>(&self, f: &mut W, style: &Style) -> std::io::Result<()> {
        todo!()
    }

    fn children(&self) -> std::borrow::Cow<[Self::Child]> {
        todo!()
    }
} */
