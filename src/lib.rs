/* pub mod ast;
pub mod new;

pub use crate::{
    ast::{Node, Operator},
    new::{EquationSide, Expression, ExpressionType, Product, Sign},
};
 */

pub mod v3;

pub use crate::v3::{Context, Equation, Expression, Node, NodeOrExpression, Product, Sign};
