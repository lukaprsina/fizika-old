use super::{Expression, NodeOrExpression, Sign};

pub enum FlattenResult {
    Monomial,
    Polynomial,
    NotPossible,
}

impl Expression {
    pub fn flatten(&mut self) -> FlattenResult {
        let mut new_products = Vec::new();

        for product in self.products.iter().cloned() {
            // let only_one = product.numerator.len() <= 1;
            println!("Flatten:\n{}", &product);
            // println!("{:#?}", &product);
            println!();
            let mut changed = false;

            for (pos, node_or_expression) in product.numerator.iter().cloned().enumerate() {
                println!("NodeOrExpression:\n{}", &node_or_expression);
                // println!("{:#?}", &node_or_expression);
                println!();
                if let NodeOrExpression::Expression(mut expression) = node_or_expression {
                    match expression.flatten() {
                        FlattenResult::Polynomial => {
                            // println!("Polynomial");

                            /* only_one && */
                            if matches!(product.sign, Sign::Positive) {
                                new_products.append(&mut expression.products);
                                changed = true;
                                // println!("If hit");
                            }
                        }
                        FlattenResult::Monomial => {
                            // println!("Monomial");
                            let first_product = expression.products[0].clone();
                            let mut new_product = product.clone();

                            new_product.numerator.remove(pos);
                            for (pos2, node_or_expression) in
                                first_product.numerator.into_iter().enumerate()
                            {
                                new_product.numerator.insert(pos + pos2, node_or_expression);
                            }

                            new_products.push(new_product);
                            changed = true;
                        }
                        FlattenResult::NotPossible => {
                            // println!("NotPossible");
                        }
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
        } /*  else {
              return FlattenResult::NotPossible;
          } */

        let mut can_flatten = FlattenResult::Polynomial;

        /* for product in self.products.iter() {
            println!("Product:\n{:#?}", product);
            for node_or_expression in product.numerator.iter() {
                if matches!(node_or_expression, NodeOrExpression::Expression(_)) {
                    println!("Will return NotPossible:\n{:#?}", node_or_expression);
                    can_flatten = FlattenResult::NotPossible;
                    break;
                }
            }
        } */

        can_flatten
    }
}
