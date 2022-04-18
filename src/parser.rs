use nom::{
    branch::alt,
    bytes::complete::{is_not, tag, tag_no_case, take_until},
    character::complete::{char, multispace0, one_of},
    combinator::{map_res, opt, recognize, value},
    error::ParseError,
    multi::{many0, many1},
    sequence::{delimited, pair, preceded, terminated, tuple},
    IResult,
};
use num::{BigInt, BigRational, Complex, Rational64};

#[derive(Debug, PartialEq)]
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
    Rational64,
}

#[derive(Debug)]
pub enum Token {
    Binary(Operation),
    Unary(Operation),
    LeftParenthesis,
    RightParenthesis,
    Comma,
    Number(Number, Option<Unit>),
    Variable(String),
    Function(String, Option<usize>),
}

fn parse_eol_comment<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, (), E> {
    value(
        (), // Output is thrown away.
        pair(tag(r"\/\/"), is_not("\n\r")),
    )(i)
}

fn parse_inline_comment<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, (), E> {
    value(
        (), // Output is thrown away.
        tuple((tag("/*"), take_until("*/"), tag("*)"))),
    )(i)
}

fn ws<'a, F: 'a, O, E: ParseError<&'a str>>(
    inner: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    F: Fn(&'a str) -> IResult<&'a str, O, E>,
{
    delimited(multispace0, inner, multispace0)
}

fn parse_and_map<'a, OutputType, FuncError>(
    name: &'a str,
    function: impl FnMut(&'a str) -> Result<OutputType, FuncError>,
) -> impl FnMut(&'a str) -> IResult<&'a str, OutputType> {
    map_res(ws(tag_no_case(name)), function)
}

fn parse_operation(input: &str) -> IResult<&str, Operation> {
    alt((
        parse_and_map("+", |_| Ok::<Operation, ()>(Operation::Add)),
        // map_res(ws(tag("+")), |_| Ok::<Operation, ()>(Operation::Add)),
        parse_and_map("-", |_| Ok::<Operation, ()>(Operation::Subtract)),
        parse_and_map("*", |_| Ok::<Operation, ()>(Operation::Multiply)),
        parse_and_map("/", |_| Ok::<Operation, ()>(Operation::Divide)),
        parse_and_map("%", |_| Ok::<Operation, ()>(Operation::Mod)),
        parse_and_map("^", |_| Ok::<Operation, ()>(Operation::Power)),
        parse_and_map("!", |_| Ok::<Operation, ()>(Operation::Factorial)),
        parse_and_map("=", |_| Ok::<Operation, ()>(Operation::Equal)),
        parse_and_map("!=", |_| Ok::<Operation, ()>(Operation::NotEqual)),
        parse_and_map("<", |_| Ok::<Operation, ()>(Operation::LessThan)),
        parse_and_map("<=", |_| Ok::<Operation, ()>(Operation::LessThanOrEqual)),
        parse_and_map(">=", |_| Ok::<Operation, ()>(Operation::GreaterThanOrEqual)),
        parse_and_map(">", |_| Ok::<Operation, ()>(Operation::GreaterThan)),
    ))(input)
}

fn parse_unit(input: &str) -> IResult<&str, Unit> {
    alt((
        parse_and_map("°", |_| Ok::<Unit, ()>(Unit::Second)),
        parse_and_map("\'", |_| Ok::<Unit, ()>(Unit::Second)),
        parse_and_map("\'\'", |_| Ok::<Unit, ()>(Unit::Minute)),
        parse_and_map("rad", |_| Ok::<Unit, ()>(Unit::Radian)),
        parse_and_map("grad", |_| Ok::<Unit, ()>(Unit::Gradian)),
        parse_and_map("custom", |s: &str| {
            Ok::<Unit, ()>(Unit::Custom(s.to_string()))
        }),
    ))(input)
}

fn parse_hexadecimal(input: &str) -> IResult<&str, i64> {
    map_res(
        preceded(
            alt((tag("0x"), tag("0X"))),
            recognize(many1(terminated(
                one_of("0123456789abcdefABCDEF"),
                many0(char('_')),
            ))),
        ),
        |out: &str| i64::from_str_radix(&str::replace(&out, "_", ""), 16),
    )(input)
}

fn parse_octal(input: &str) -> IResult<&str, i64> {
    map_res(
        preceded(
            alt((tag("0o"), tag("0O"))),
            recognize(many1(terminated(one_of("01234567"), many0(char('_'))))),
        ),
        |out: &str| i64::from_str_radix(&str::replace(&out, "_", ""), 8),
    )(input)
}

fn parse_binary(input: &str) -> IResult<&str, i64> {
    map_res(
        preceded(
            alt((tag("0b"), tag("0B"))),
            recognize(many1(terminated(one_of("01"), many0(char('_'))))),
        ),
        |out: &str| i64::from_str_radix(&str::replace(&out, "_", ""), 2),
    )(input)
}

fn parse_decimal(input: &str) -> IResult<&str, i64> {
    map_res(
        recognize(many1(terminated(one_of("0123456789"), many0(char('_'))))),
        |out: &str| i64::from_str_radix(&str::replace(&out, "_", ""), 10),
    )(input)
}

fn parse_float(input: &str) -> IResult<&str, f64> {
    alt((
        // Case one: .42
        map_res(
            recognize(tuple((
                char('.'),
                parse_decimal,
                opt(tuple((one_of("eE"), opt(one_of("+-")), parse_decimal))),
            ))),
            |out: &str| out.parse::<f64>(),
        ), // Case two: 42e42 and 42.42e42
        map_res(
            recognize(tuple((
                parse_decimal,
                opt(preceded(char('.'), parse_decimal)),
                one_of("eE"),
                opt(one_of("+-")),
                parse_decimal,
            ))),
            |out: &str| out.parse::<f64>(),
        ), // Case three: 42. and 42.42
        map_res(
            recognize(tuple((parse_decimal, char('.'), opt(parse_decimal)))),
            |out: &str| out.parse::<f64>(),
        ),
    ))(input)
}

/* fn parse_number(input: &str) -> IResult<&str, i64> {
    alt((parse_binary(input),))(input)
} */

fn parse_variable(input: &str) -> IResult<&str, &str> {
    tag("a")(input)
}

fn parse_function(input: &str) -> IResult<&str, &str> {
    tag("a")(input)
}

fn parse_token(input: &str) -> IResult<&str, &str> {
    alt((ws(tag("(")), ws(tag(")")), ws(tag(","))))(input)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_operation() {
        assert_eq!(
            parse_operation("    \n\t/  a  "),
            Ok(("a  ", Operation::Divide))
        );

        assert_eq!(parse_operation("+"), Ok(("", Operation::Add)));
    }

    #[test]
    fn test_unit() {
        assert_eq!(parse_unit("    \n\t\'    "), Ok(("", Unit::Second)));
    }
}
