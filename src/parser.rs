use crate::ast::{Node, Operator};
use pest_consume::{match_nodes, Error, Parser};

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct ExpressionParser;

pub type Result<T> = std::result::Result<T, Error<Rule>>;
pub type Input<'i> = pest_consume::Node<'i, Rule, ()>;

#[pest_consume::parser]
impl ExpressionParser {
    fn EOI(_input: Input) -> Result<()> {
        Ok(())
    }

    fn Expr(input: Input) -> Result<Node> {
        Ok(match_nodes!(input.into_children();
            [UnaryExpr] => Node::Unary { op: Operator::Plus, child: Box::new(Node::Number(1.0)) },
            [BinaryExpr] => Node::Binary { op: Operator::Plus, lhs: Box::new(Node::Number(1.0)), rhs: Box::new(Node::Number(1.0)) },
        ))
    }

    fn Program(input: Input) -> Result<Node> {
        Ok(crate::parser::Rule::Expr(input))
    }

    fn Term(input: Input) -> Result<Node> {
        Ok(match_nodes!(input.into_children();
            [Int] => Node::Number(1.0),
            [Expr] => Node::Number(1.0),
        ))
    }

    fn UnaryExpr(input: Input) -> Result {}

    fn BinaryExpr(input: Input) -> Result {}

    /* fn String(input: Input) -> Result<&str> {}

    fn FunctionArg(input: Input) -> Result {}

    fn Function(input: Input) -> Result {} */

    fn Operator(input: Input) -> Result<&str> {
        Ok(input.as_str())
    }

    fn Int(input: Input) -> Result<f64> {
        input
            .as_str()
            .parse::<f64>()
            // `input.error` links the error to the location in the input file where it occurred.
            .map_err(|e| input.error(e))
    }
}
