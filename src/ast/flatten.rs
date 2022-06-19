use super::{expression::Element, Expression, NodeOrExpression, Product, Sign};

pub enum FlattenResult {
    Monomial,
    Polynomial,
}

fn conditional_fill(return_full: bool, element: &Element) -> Vec<Element> {
    if return_full {
        vec![element.clone()]
    } else {
        vec![]
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
                for (element_pos, element) in side.iter_mut().enumerate() {
                    // println!("Element:\n{:#?}\n\n", &element);

                    match &mut element.node_or_expression {
                        NodeOrExpression::Expression(expression) => match &mut expression.flatten()
                        {
                            FlattenResult::Polynomial => match element.sign {
                                Sign::Positive => {
                                    if side_len == 1 {
                                        new_products.append(
                                            &mut expression.products.iter().cloned().collect(),
                                        );
                                    } else {
                                        match side_pos {
                                            0 => new_product.numerator.push(element.clone()),
                                            1 => new_product.denominator.push(element.clone()),
                                            _ => unreachable!(),
                                        }
                                    }
                                }
                                Sign::Negative => match side_pos {
                                    0 => new_product.numerator.push(element.clone()),
                                    1 => new_product.denominator.push(element.clone()),
                                    _ => unreachable!(),
                                },
                            },
                            FlattenResult::Monomial => match element.sign {
                                Sign::Positive => match side_pos {
                                    0 => new_product.numerator.push(element.clone()),
                                    1 => new_product.denominator.push(element.clone()),
                                    _ => unreachable!(),
                                },
                                Sign::Negative => match side_pos {
                                    0 => new_product.numerator.push(element.clone()),
                                    1 => new_product.denominator.push(element.clone()),
                                    _ => unreachable!(),
                                },
                            },
                        },
                        NodeOrExpression::Node(..) => match side_pos {
                            0 => new_product.numerator.push(element.clone()),
                            1 => new_product.denominator.push(element.clone()),
                            _ => unreachable!(),
                        },
                    }

                    // println!("New product:\n{:#?}\n\n", &new_product);
                }

                if !new_product.numerator.is_empty() || !new_product.denominator.is_empty() {
                    new_products.push(new_product);
                    // println!("New products:\n{:#?}\n\n", &new_products);
                }
            }
        }

        // println!("New products:\n{:#?}\n\n", &new_products);
        self.products = new_products;

        if self.products.len() <= 1 {
            FlattenResult::Monomial
        } else {
            FlattenResult::Polynomial
        }
    }
}
