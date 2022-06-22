use super::{
    element::{IsTimesVisible, ShouldBeParenthesized},
    Element, Sign,
};

#[derive(Debug, Clone)]
pub struct Product {
    pub numerator: Vec<Element>,
    pub denominator: Vec<Element>,
}

impl Product {
    // TODO: don't accept empty products
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
