use super::{expression::Element, Expression, NodeOrExpression, Product, Sign};

pub enum FlattenResult {
    Monomial(Product),
    Polynomial,
}

fn copy_element_to_product(element: &Element, new_product: &mut Product, side_pos: usize) {
    match side_pos {
        0 => new_product.numerator.push(element.clone()),
        1 => new_product.denominator.push(element.clone()),
        _ => unreachable!(),
    }
}

impl Expression {
    pub fn flatten(&mut self) -> FlattenResult {
        let mut new_products: Vec<Product> = Vec::new();

        for product in self.products.iter_mut() {
            for (side_pos, side) in [&mut product.numerator, &mut product.denominator]
                .into_iter()
                .enumerate()
            {
                let mut new_product = Product::new(vec![], vec![]);

                let side_len = side.len();
                for element in side.iter_mut() {
                    // println!("Element:\n{:#?}\n\n", &element);

                    match &mut element.node_or_expression {
                        NodeOrExpression::Expression(expression) => {
                            let result = expression.flatten();
                            match element.sign {
                                Sign::Positive => match side_len {
                                    1 => {
                                        new_products.append(
                                            &mut expression.products.iter().cloned().collect(),
                                        );
                                    }
                                    0 => unreachable!(),

                                    // > 1
                                    _ => match result {
                                        FlattenResult::Monomial(mut nested_product) => {
                                            for (nested_side_pos, mut nested_side) in [
                                                &mut nested_product.numerator,
                                                &mut nested_product.denominator,
                                            ]
                                            .into_iter()
                                            .enumerate()
                                            {
                                                match nested_side_pos {
                                                    0 => new_product
                                                        .numerator
                                                        .append(&mut nested_side),

                                                    1 => new_product
                                                        .denominator
                                                        .append(&mut nested_side),
                                                    _ => unreachable!(),
                                                }
                                            }
                                        }
                                        FlattenResult::Polynomial => copy_element_to_product(
                                            element,
                                            &mut new_product,
                                            side_pos,
                                        ),
                                    },
                                },
                                Sign::Negative => {
                                    copy_element_to_product(element, &mut new_product, side_pos)
                                }
                            }
                        }
                        NodeOrExpression::Node(..) => {
                            copy_element_to_product(element, &mut new_product, side_pos)
                        }
                    }

                    // println!("New product:\n{:#?}\n\n", &new_product);
                }

                if !new_product.numerator.is_empty() || !new_product.denominator.is_empty() {
                    new_products.push(new_product);
                    // println!("New products:\n{:#?}\n\n", &new_products);
                }
            }
        }

        // println!("Switching products:\n{:#?}\n\n", &new_products);
        self.products = new_products;

        match self.products.first() {
            Some(product) => {
                if self.products.len() == 1 {
                    FlattenResult::Monomial(product.clone())
                } else {
                    FlattenResult::Polynomial
                }
            }
            None => unreachable!(),
        }
    }
}
