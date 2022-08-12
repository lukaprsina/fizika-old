use crate::ast::{product::Product, Element, Expression, NodeOrExpression, Sign};

pub enum FlattenResult {
    Monomial,
    Polynomial,
}

fn move_element_to_product(element: Element, new_product: &mut Product, side_pos: usize) {
    match side_pos {
        0 => new_product.numerator.push(element.clone()),
        1 => new_product.denominator.push(element.clone()),
        _ => unreachable!(),
    }
}

impl Expression {
    pub fn flatten(self) -> (Expression, FlattenResult) {
        let mut new_expression = Expression::new(vec![]);

        for product in self.products.into_iter() {
            let mut new_product = Product::new(vec![], vec![]);

            for (side_pos, side) in [product.numerator, product.denominator]
                .into_iter()
                .enumerate()
            {
                let side_len = side.len();
                for element in side.into_iter() {
                    match element.node_or_expression {
                        NodeOrExpression::Expression(expression) => {
                            let new_expr = match_expression(expression, element.sign, side_len);

                            let new_elem =
                                Element::new(element.sign, NodeOrExpression::Expression(new_expr));

                            move_element_to_product(new_elem, &mut new_product, side_pos)
                        }
                        NodeOrExpression::Node(_) => {
                            move_element_to_product(element, &mut new_product, side_pos)
                        }
                    }
                }
            }

            new_expression.products.push(new_product);
        }

        (new_expression, FlattenResult::Polynomial)
    }
}

fn match_expression(expression: Expression, sign: Sign, side_len: usize) -> Expression {
    let (new_expr, result) = expression.flatten();

    match sign {
        Sign::Positive => match side_len {
            0 => unreachable!(),
            1 => (),
            // > 1
            _ => (),
        },
        Sign::Negative => (),
    }

    new_expr
}
