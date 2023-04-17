use core::panic;
use std::collections::HashMap;

use itertools::Itertools;
use once_cell::sync::Lazy;
use tracing::debug;

use crate::{
    ast::{product::Product, Element, Equation, Expression, Node, NodeOrExpression, Sign},
    output::equation_to_rpn::ReversePolishNotation,
};

use super::strategy::Strategy;

// imply that it has been analysed
fn apply_inverse(equation: &mut Equation) -> Vec<Equation> {
    if equation.equation_sides.len() != 2 {
        return vec![];
    }

    let constraints = vec![];

    let mut inverse = None;

    for side_element in &mut equation.equation_sides {
        debug!("{}", side_element.rpn());
        match &mut side_element.cache {
            Some(cache) => {
                if cache.variables.len() >= 1 {
                    inverse = get_element_inverse(side_element);
                    break;
                }
            }
            None => panic!("Equation has not been analyzed, cannot simplify"),
        }
    }

    constraints
}

pub fn get_apply_inverse() -> Strategy {
    Strategy {
        equation: Some(Box::new(apply_inverse)),
    }
}

static INVERSE_FUNCTIONS: Lazy<HashMap<String, (Element, Vec<String>)>> = Lazy::new(|| {
    let map: HashMap<&str, (&str, Vec<&str>)> = HashMap::from([
        ("sin", ("arcsin", vec![])),
        ("cos", ("arccos", vec![])),
        ("tan", ("arctan", vec![])),
        ("cot", ("arccot", vec![])),
    ]);

    let mut new_map: HashMap<String, (Element, Vec<String>)> = HashMap::new();
    for (key, value) in map.into_iter() {
        let new_key = key.to_string();

        let new_value = (
            Element::new(
                Sign::Positive,
                NodeOrExpression::Node(Node::Function {
                    name: value.0.to_string(),
                    arguments: vec![],
                }),
            ),
            value
                .1
                .into_iter()
                .map(|constraint| constraint.to_string())
                .collect_vec(),
        );

        new_map.insert(new_key, new_value);
    }
    new_map
});

fn get_element_inverse(element: &mut Element) -> Option<(Element, Vec<String>)> {
    let mut constraints: Vec<String> = vec![];

    let inverse = match &element.node_or_expression {
        NodeOrExpression::Node(node) => match node {
            Node::Power { base, power } => {
                if let (Some(b_cache), Some(p_cache)) = (&base.cache, &power.cache) {
                    if b_cache.variables.len() == 1 {
                        None
                    } else if p_cache.variables.len() == 1 {
                        None
                    } else {
                        None
                    }
                } else {
                    panic!("Not analyzed when getting the inverse")
                }
            }
            Node::Function { name, arguments: _ } => {
                if let Some(value) = INVERSE_FUNCTIONS.get(name) {
                    constraints.extend(value.1.clone());
                    Some(value.0.clone())
                } else {
                    let negative_one = Element::new(
                        Sign::Positive,
                        NodeOrExpression::Node(Node::Number(num::BigRational::from_integer(
                            (-1).into(),
                        ))),
                    );

                    let inverse_func = Element::new(
                        Sign::Positive,
                        NodeOrExpression::Node(Node::Power {
                            base: Box::new(element.clone()),
                            power: Box::new(negative_one),
                        }),
                    );
                    Some(inverse_func)
                }
            }
            _ => None,
        },
        NodeOrExpression::Expression(expression) => match expression.products.len() {
            0 => None,
            1 => one_product(expression.products.first().unwrap()),
            _ => multiple_products(expression),
        },
    };

    match inverse {
        Some(element) => Some((element, constraints)),
        None => None,
    }
}

fn one_product(product: &Product) -> Option<Element> {
    for side in [&product.numerator, &product.denominator] {
        for pr_elem in side {
            match &pr_elem.cache {
                Some(cache) => {
                    let a = cache.variables.len() == 1;
                }
                None => panic!("Element should be analyzed when applying inverse"),
            }
        }
    }

    None
}

fn multiple_products(expression: &Expression) -> Option<Element> {
    let mut new_expression = Expression::new(vec![]);

    for product in &expression.products {
        let mut skip_product = false;

        for side in [&product.numerator, &product.denominator] {
            for pr_elem in side {
                skip_product = match &pr_elem.cache {
                    Some(cache) => cache.variables.len() == 1,
                    None => panic!("Element should be analyzed when applying inverse"),
                };

                if skip_product {
                    break;
                }
            }
        }

        if !skip_product {
            let mut new_product = product.clone();
            if let Some(pr_elem) = new_product.numerator.first_mut() {
                pr_elem.sign = pr_elem.sign * Sign::Negative;
                new_expression.products.push(new_product);
            }
        }
    }

    debug!("{new_expression}");

    None
}
