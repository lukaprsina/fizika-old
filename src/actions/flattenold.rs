use tracing::info;

use crate::ast::{product::Product, Element, Expression, NodeOrExpression, Sign};

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
    #[tracing::instrument(skip_all)]
    pub fn flatten(&mut self) -> FlattenResult {
        let mut new_products: Vec<Product> = Vec::new();

        info!("Call flatten: {}", self);

        for product in self.products.iter_mut() {
            let mut new_product = Product::new(vec![], vec![]);

            for (side_pos, side) in [&mut product.numerator, &mut product.denominator]
                .into_iter()
                .enumerate()
            {
                info!("Side pos: {}", side_pos);
                let side_len = side.len();
                for element in side.iter_mut() {
                    info!("Element: {}", element);

                    match &mut element.node_or_expression {
                        NodeOrExpression::Expression(expression) => {
                            let result = expression.flatten();
                            info!("Returned expr: {}", expression);

                            match element.sign {
                                Sign::Positive => match side_len {
                                    1 => {
                                        for pr in new_products.iter() {
                                            println!("\t{}", pr);
                                        }
                                        println!("{}", expression);
                                        let new_element = Element::new(
                                            Sign::Positive,
                                            NodeOrExpression::Expression(Expression::new(
                                                expression.products.clone(),
                                            )),
                                        );
                                        new_products.push(Product::new(vec![new_element], vec![]));
                                    }
                                    0 => unreachable!(),

                                    // > 1
                                    _ => match result {
                                        FlattenResult::Monomial(mut nested_product) => {
                                            for (nested_side_pos, nested_side) in [
                                                &mut nested_product.numerator,
                                                &mut nested_product.denominator,
                                            ]
                                            .into_iter()
                                            .enumerate()
                                            {
                                                match nested_side_pos {
                                                    0 => new_product.numerator.append(nested_side),

                                                    1 => {
                                                        new_product.denominator.append(nested_side)
                                                    }
                                                    _ => unreachable!(),
                                                }
                                            }
                                        }
                                        FlattenResult::Polynomial => {
                                            copy_element_to_product(
                                                element,
                                                &mut new_product,
                                                side_pos,
                                            );
                                            // println!("{element}\n{new_product}\n{side_pos}");
                                        }
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

                    info!("Elem added, np: {}", new_product);
                }
            }

            if !new_product.numerator.is_empty() || !new_product.denominator.is_empty() {
                info!("End side, np: {}", new_product);
                new_products.push(new_product);
            }
        }

        // info!("Both sides done, np: {:#?}", new_products);
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
