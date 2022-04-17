use num::{BigInt, BigRational, Complex, Rational64 };
use nom::{
    branch::alt, bytes::complete::{tag, tag_no_case}, character::complete::{multispace0, one_of, char}, error::ParseError,
    sequence::{delimited, preceded, terminated}, IResult, combinator::{map_res, recognize}, multi::{many1, many0},
};

// use duplicate

#[derive(Debug)]
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

#[derive(Debug, PartialEq)]
pub enum Unit {
    Degree,
    Second,
    Minute,
    Radian,
    Gradian,
    Custom(String),
}

#[derive(Debug)]
pub enum Number {
    BigInt,
    BigRational,
    Complex,
    Rational64
}

#[derive(Debug)]
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

fn parse_unit(input: &str) -> IResult<&str, Unit> {
    alt((
        map_res(ws(tag_no_case("°")), |_| Ok::<Unit, ()>(Unit::Degree)),
        map_res(ws(tag_no_case("\'")), |_| Ok::<Unit, ()>(Unit::Second)),
        map_res(ws(tag_no_case("\'\'")), |_| Ok::<Unit, ()>(Unit::Minute)),
        map_res(ws(tag_no_case("rad")), |_| Ok::<Unit, ()>(Unit::Radian)),
        map_res(ws(tag_no_case("grad")), |_| Ok::<Unit, ()>(Unit::Gradian)),
        map_res(ws(tag_no_case("custom")), |s: &str| Ok::<Unit, ()>(Unit::Custom(s.to_string()))),
    ))(input)
}


// TODO: function to convert to Token::Number
fn parse_hexadecimal(input: &str) -> IResult<&str, i64> {
    map_res(
      preceded(
        alt((tag("0x"), tag("0X"))),
        recognize(
          many1(
            terminated(one_of("0123456789abcdefABCDEF"), many0(char('_')))
          )
        )
      ),
      |out: &str| i64::from_str_radix(&str::replace(&out, "_", ""), 16)
    )(input)
  }

fn parse_number(input: &str) -> IResult<&str, &str> {
    tag("a")(input)
}

fn parse_variable(input: &str) -> IResult<&str, &str> {
    tag("a")(input)
}

fn parse_function(input: &str) -> IResult<&str, &str> {
    tag("a")(input)
}

fn parse_token(input: &str) -> IResult<&str, &str> {
    alt((
        ws(tag("(")),
        ws(tag(")")),
        ws(tag(",")),
    ))(input)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_operation() {
        assert_eq!(parse_operation("    \n\t+  a  "), Ok(("a  ", "+")));
    }

    #[test]
    fn test_unit() {
        assert_eq!(parse_unit("    \n\t\'    "), Ok(("", Unit::Second)));
    }
}
