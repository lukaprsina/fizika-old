use super::{Expression, NodeOrExpression, Product, Sign};

pub enum FlattenResult {
    Monomial,
    Polynomial,
}

fn test(return_full: bool, node_or_expression: &NodeOrExpression) -> Vec<NodeOrExpression> {
    if return_full {
        vec![node_or_expression.clone()]
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
                for node_or_expression in side.iter_mut() {
                    println!("NodeOrExpression:\n{:#?}\n\n", &node_or_expression);

                    // minus are expressions, plus are nodes
                    match node_or_expression {
                        NodeOrExpression::Expression(expression) => match expression.flatten() {
                            FlattenResult::Polynomial => {
                                if product.sign == Sign::Positive {
                                    new_products
                                        .append(&mut expression.products.iter().cloned().collect());
                                } else {
                                    new_products.push(Product::new(
                                        product.sign,
                                        test(side_pos == 0, &node_or_expression),
                                        test(side_pos == 1, &node_or_expression),
                                    ));
                                }
                            }
                            FlattenResult::Monomial => if product.sign == Sign::Positive {},
                        },
                        NodeOrExpression::Node(..) => {
                            new_products.push(Product::new(
                                product.sign,
                                test(side_pos == 0, &node_or_expression),
                                test(side_pos == 1, &node_or_expression),
                            ));
                        }
                    }
                }
            }
        }

        self.products = new_products;

        if self.products.len() <= 1 {
            FlattenResult::Monomial
        } else {
            FlattenResult::Polynomial
        }
    }
}
