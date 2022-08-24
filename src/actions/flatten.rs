use std::rc::Rc;

use crate::ast::{product::Product, Element, Equation, Expression, NodeOrExpression, Sign};

#[derive(Debug)]
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

impl Equation {
    // #[tracing::instrument(skip_all)]
    pub fn flatten(self) -> Equation {
        let mut new_equation = Equation {
            eq_sides: vec![],
            app: Rc::clone(&self.app),
            context: self.context,
            cache: self.cache,
        };

        // ANATODO
        for element in self.eq_sides {
            new_equation.eq_sides.push(element.flatten());
        }

        new_equation
    }
}

impl Element {
    pub fn flatten(self) -> Element {
        match self.node_or_expression {
            NodeOrExpression::Expression(expression) => Element::new(
                self.sign,
                NodeOrExpression::Expression(expression.flatten()),
            ),
            NodeOrExpression::Node(_) => self,
        }
    }
}

impl Expression {
    #[tracing::instrument(skip_all)]
    pub fn flatten(self) -> Expression {
        // info!("Flatten: {}", self);

        let mut result_expression = Expression::new(vec![]);

        for product in self.products.into_iter() {
            // debug!("New product: {}", product);
            let mut result_product = Product::new(vec![], vec![]);

            let is_surrounded = (product.numerator.len() + product.denominator.len()) >= 2;

            for (side_pos, side) in [product.numerator, product.denominator]
                .into_iter()
                .enumerate()
            {
                // debug!("Side {}", side_pos);
                // println!("{:#?}", side);

                for element in side.into_iter() {
                    // debug!("Element: {}", element);
                    match element.node_or_expression {
                        NodeOrExpression::Expression(expression) => {
                            let mut new_expr = expression.flatten();

                            let create_new_element = match element.sign {
                                Sign::Positive => {
                                    if is_surrounded {
                                        // info!("num_elements_in_expr > 1");
                                        if new_expr.products.len() == 1 {
                                            let only_product = new_expr.products.remove(0);

                                            result_product.numerator.extend(only_product.numerator);

                                            result_product
                                                .denominator
                                                .extend(only_product.denominator);

                                            false
                                        } else {
                                            true
                                        }
                                    } else {
                                        result_expression
                                            .products
                                            .extend(new_expr.products.clone());
                                        false
                                    }
                                }
                                Sign::Negative => true,
                            };

                            if create_new_element {
                                let new_elem = Element::new(
                                    element.sign,
                                    NodeOrExpression::Expression(new_expr),
                                );

                                move_element_to_product(new_elem, &mut result_product, side_pos)
                            }
                        }
                        NodeOrExpression::Node(_) => {
                            move_element_to_product(element, &mut result_product, side_pos)
                        }
                    }
                }
            }

            if !result_product.numerator.is_empty() || !result_product.denominator.is_empty() {
                result_expression.products.push(result_product);
            }
        }

        result_expression
    }
}
