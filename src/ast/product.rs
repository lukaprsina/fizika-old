use super::{
    element::{IsTimesVisible, ShouldBeParenthesized},
    Element, Node, NodeOrExpression, Sign,
};

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct Product {
    pub numerator: Vec<Element>,
    pub denominator: Vec<Element>,
}

impl Product {
    pub fn new(numerator: Vec<Element>, denominator: Vec<Element>) -> Product {
        Product {
            numerator,
            denominator,
        }
    }

    pub fn get_sign(&self) -> Sign {
        match self.numerator.first() {
            Some(first) => first.sign,
            None => match self.denominator.first() {
                Some(first) => first.sign,
                None => unreachable!("Empty products are not allowed"),
            },
        }
    }

    pub fn rationalize(&mut self) -> Self {
        let mut new_product = Product::new(vec![], vec![]);

        let mut new_number = num::BigRational::new(
            num::BigInt::new(num::bigint::Sign::NoSign, vec![]),
            num::BigInt::new(num::bigint::Sign::NoSign, vec![]),
        );

        for (side_pos, side) in [&self.numerator, &self.denominator].into_iter().enumerate() {
            for element in side {
                let add_element = match &element.node_or_expression {
                    NodeOrExpression::Node(node) => match node {
                        Node::Number(number) => {
                            if side_pos == 0 {
                                new_number *= number;
                            } else {
                                new_number /= number;
                            }
                            false
                        }
                        _ => true,
                    },
                    NodeOrExpression::Expression(_) => true,
                };

                if add_element {
                    if side_pos == 0 {
                        new_product.numerator.push(element.clone());
                    } else {
                        new_product.denominator.push(element.clone());
                    }
                }
            }
        }

        new_product
    }
}

impl IsTimesVisible for Product {
    fn is_times_visible(&self, last: &Element) -> bool {
        if !self.numerator.is_empty() {
            self.numerator[0].is_times_visible(last)
        } else {
            true
        }
    }
}

impl ShouldBeParenthesized for Product {
    fn should_be_parenthesized(&self) -> bool {
        if self.numerator.len() == 1 {
            self.numerator[0].should_be_parenthesized()
        } else {
            true
        }
    }
}
