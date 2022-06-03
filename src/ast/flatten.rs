use super::{Expression, NodeOrExpression};

pub enum FlattenResult {
    Monomial,
    Polynomial,
    NotPossible,
}

impl Expression {
    pub fn flatten(&mut self) -> FlattenResult {
        let mut new_products = Vec::new();

        for product in self.products.iter().cloned() {
            let only_one = product.numerator.len() <= 1;
            let mut changed = false;

            for (pos, node_or_expression) in product.numerator.iter().cloned().enumerate() {
                if let NodeOrExpression::Expression(mut expression) = node_or_expression {
                    match expression.flatten() {
                        FlattenResult::Polynomial => {
                            if only_one {
                                new_products.append(&mut expression.products);
                                changed = true;
                            }
                        }
                        FlattenResult::Monomial => {
                            let first_product = expression.products[0].clone();
                            let mut new_product = product.clone();

                            new_product.numerator.remove(pos);
                            for (pos2, node_or_expression) in
                                first_product.numerator.iter().cloned().enumerate()
                            {
                                new_product.numerator.insert(pos + pos2, node_or_expression);
                            }

                            new_products.push(new_product);
                            changed = true;
                        }
                        FlattenResult::NotPossible => (),
                    }
                }
            }

            if !changed {
                new_products.push(product);
            }
        }

        self.products = new_products;

        if self.products.len() <= 1 {
            return FlattenResult::Monomial;
        }

        let mut can_flatten = FlattenResult::Polynomial;

        for product in self.products.iter() {
            for node_or_expression in product.numerator.iter() {
                if matches!(node_or_expression, NodeOrExpression::Expression(_)) {
                    can_flatten = FlattenResult::NotPossible;
                    break;
                }
            }
        }

        can_flatten
    }
}
