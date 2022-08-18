use crate::ast::{
    analyzed_expression::AnalyzedElement, product::Product, Element, Equation, Expression, Node,
    NodeOrExpression,
};

pub trait IsSame {
    fn is_same(lhs: &Self, rhs: &Self) -> bool;
}

impl<T: PartialOrd + Clone + IsSame> IsSame for Vec<T> {
    fn is_same(lhs: &Self, rhs: &Self) -> bool {
        if lhs.len() != rhs.len() {
            return false;
        }

        if lhs.is_empty() || rhs.is_empty() {
            return true;
        }

        let mut result = false;
        for left in lhs {
            for right in rhs {
                let are_same = T::is_same(left, right);
                result |= are_same;
                if result {
                    break;
                }
            }
        }

        result
    }
}

impl IsSame for Equation {
    fn is_same(lhs: &Self, rhs: &Self) -> bool {
        if lhs.uuids.len() != rhs.uuids.len() {
            return false;
        }
        // TODO: not true
        let mut result = true;
        let borrowed_app = lhs.app.borrow();
        let contex = borrowed_app.get_context(lhs.context).unwrap();

        for (left_uuid, right_uuid) in lhs.uuids.iter().zip(&rhs.uuids) {
            let a = contex.get_element(*left_uuid).unwrap();
            let b = contex.get_element(*right_uuid).unwrap();
            let are_same = AnalyzedElement::is_same(a, b);
            result &= are_same;
            if !result {
                break;
            }
        }
        result
    }
}

impl IsSame for AnalyzedElement {
    fn is_same(lhs: &Self, rhs: &Self) -> bool {
        Element::is_same(&lhs.element, &rhs.element)
    }
}

impl IsSame for Element {
    fn is_same(lhs: &Self, rhs: &Self) -> bool {
        lhs.sign == rhs.sign
            && NodeOrExpression::is_same(&lhs.node_or_expression, &rhs.node_or_expression)
    }
}

impl IsSame for NodeOrExpression {
    fn is_same(lhs: &Self, rhs: &Self) -> bool {
        match lhs {
            NodeOrExpression::Node(l_node) => match rhs {
                NodeOrExpression::Node(r_node) => Node::is_same(l_node, r_node),
                NodeOrExpression::Expression(_) => false,
            },
            NodeOrExpression::Expression(l_expr) => match rhs {
                NodeOrExpression::Node(_) => false,
                NodeOrExpression::Expression(r_expr) => Expression::is_same(l_expr, r_expr),
            },
        }
    }
}

impl IsSame for Node {
    fn is_same(lhs: &Self, rhs: &Self) -> bool {
        match lhs {
            Node::Number(left_number) => {
                if let Node::Number(right_number) = rhs {
                    left_number == right_number
                } else {
                    false
                }
            }
            Node::Variable(left_name) => {
                if let Node::Variable(right_name) = rhs {
                    left_name == right_name
                } else {
                    false
                }
            }
            Node::Unit(left_name) => {
                if let Node::Unit(right_name) = rhs {
                    left_name == right_name
                } else {
                    false
                }
            }
            Node::Power {
                base: left_base,
                power: left_power,
            } => {
                if let Node::Power {
                    base: right_base,
                    power: right_power,
                } = rhs
                {
                    Element::is_same(left_base, right_base)
                        && Element::is_same(left_power, right_power)
                } else {
                    false
                }
            }
            Node::Modulo {
                lhs: left_lhs,
                rhs: left_rhs,
            } => {
                if let Node::Modulo {
                    lhs: right_lhs,
                    rhs: right_rhs,
                } = rhs
                {
                    Element::is_same(left_lhs, right_lhs) && Element::is_same(left_rhs, right_rhs)
                } else {
                    false
                }
            }
            Node::Factorial { child: left_child } => {
                if let Node::Factorial { child: right_child } = rhs {
                    Element::is_same(left_child, right_child)
                } else {
                    false
                }
            }
            Node::Function {
                name: left_name,
                arguments: left_arguments,
            } => {
                if let Node::Function {
                    name: right_name,
                    arguments: right_arguments,
                } = rhs
                {
                    left_name == right_name && Vec::is_same(left_arguments, right_arguments)
                } else {
                    false
                }
            }
        }
    }
}

impl IsSame for Expression {
    fn is_same(lhs: &Self, rhs: &Self) -> bool {
        let result = Vec::is_same(&lhs.products, &rhs.products);
        result
    }
}

impl IsSame for Product {
    fn is_same(lhs: &Self, rhs: &Self) -> bool {
        let mut result = Vec::is_same(&lhs.numerator, &rhs.numerator);
        result &= Vec::is_same(&lhs.denominator, &rhs.denominator);
        result
    }
}
