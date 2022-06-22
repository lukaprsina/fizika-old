use super::{
    element::{IsTimesVisible, ShouldBeParenthesized},
    product::Product,
    Element,
};

#[derive(Debug, Clone)]
pub struct Expression {
    pub products: Vec<Product>,
}

impl Expression {
    pub fn new() -> Self {
        Expression {
            products: Vec::new(),
        }
    }
}

impl Default for Expression {
    fn default() -> Self {
        Self::new()
    }
}

impl IsTimesVisible for Expression {
    fn is_times_visible(&self, last: &Element) -> bool {
        if !self.products.is_empty() {
            self.products[0].is_times_visible(last)
        } else {
            true
        }
    }
}

impl ShouldBeParenthesized for Expression {
    fn should_be_parenthesized(&self) -> bool {
        if self.products.len() == 1 {
            self.products[0].should_be_parenthesized()
        } else {
            true
        }
    }
}
