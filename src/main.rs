use math_eval::{parser::ExpressionParser, Node};
use pest::RuleType;

fn parse_csv(input_str: &str) -> math_eval::parser::Result<Node> {
    // Parse the input into `Nodes`
    let inputs = ExpressionParser::parse(Rule::file, input_str)?;
    // There should be a single root node in the parsed tree
    let input = inputs.single()?;
    // Consume the `Node` recursively into the final value
    ExpressionParser::file(input)
}

fn main() {
    let parsed = parse_csv("1+1").unwrap();
}
