use nom::{
    branch::alt, bytes::complete::{tag, tag_no_case}, character::complete::multispace0, error::ParseError,
    sequence::delimited, IResult,
};

pub enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
    Mod,
    Power,
    Factorial,
    Equal,
    NotEqual,
    LessThan,
    LessThanOrEqual,
    GreaterThanOrEqual,
    GreaterThan,
}

pub enum Unit {
    Degree,
    Second,
    Minute,
    Radian,
    Gradian,
    Custom(String),
}

pub enum Token {
    Binary(Operation),
    Unary(Operation),
    LeftParenthesis,
    RightParenthesis,
    Comma,
    Number(f64, Option<Unit>),
    Variable(String),
    Function(String, Option<usize>),
}

fn ws<'a, F: 'a, O, E: ParseError<&'a str>>(
    inner: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    F: Fn(&'a str) -> IResult<&'a str, O, E>,
{
    delimited(multispace0, inner, multispace0)
}

fn parse_operation(input: &str) -> IResult<&str, &str> {
    alt((
        ws(tag("+")),
        ws(tag("-")),
        ws(tag("*")),
        ws(tag("/")),
        ws(tag("%")),
        ws(tag("^")),
        ws(tag("!")),
        ws(tag("=")),
        ws(tag("!=")),
        ws(tag("<")),
        ws(tag("<=")),
        ws(tag(">=")),
        ws(tag(">")),
    ))(input)
}

fn parse_unit(input: &str) -> IResult<&str, &str> {
    alt((
        ws(tag_no_case("°")),
        ws(tag_no_case("s")),
        ws(tag_no_case("m")),
        ws(tag_no_case("rad")),
        ws(tag_no_case("grad")),
        ws(tag_no_case("custom")),
    ))(input)
}

fn parse_token(input: &str) -> IResult<&str, &str> {
    alt((
        ws(parse_operation),
        ws(parse_unit),
        ws(tag("(")),
        ws(tag(")")),
        ws(tag(",")),
        // TODO
        ws(tag("number")),
        ws(tag("variable")),
        ws(tag("function")),
    ))(input)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_operation() {
        assert_eq!(parse_operation("    \n\t+  a  "), Ok(("a  ", "+")));
    }
}
