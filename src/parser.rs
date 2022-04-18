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
// use num::{BigInt, BigRational, Complex, Rational64};

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
    Number(f64, Option<Unit>),
    Variable(String),
    Function(String, Option<usize>),
}

pub fn peol_comment<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, (), E> {
    value(
        (), // Output is thrown away.
        pair(tag(r"\/\/"), is_not("\n\r")),
    )(i)
}

pub fn pinline_comment<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, (), E> {
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

/* fn parse_test<ParserInput, P, F, FuncInput, FuncOutput, FuncErr, RetInput, RetOutput, RetError>(
    parser: P,
    function: F
) -> impl FnMut(RetInput) -> IResult<RetInput, RetOutput, RetError>
where
    RetInput: Clone,
    P: Parser<ParserInput, FuncInput, RetError>,
    F: FnMut(FuncInput) -> Result<FuncOutput, FuncErr>
{
    map_res(ws(parser), function)
}

pub fn parse_test<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, (), E> {
    value(
        (), // Output is thrown away.
        tuple((tag("/*"), take_until("*/
"), tag("*)"))),
    )(i)
}

fn test() {
    let _a = alt((parse_test(), parse_test()))("a");
} */

fn parse_operation(input: &str) -> IResult<&str, Operation> {
    alt((
        map_res(ws(tag("+")), |_| Ok::<Operation, ()>(Operation::Add)),
        map_res(ws(tag("-")), |_| Ok::<Operation, ()>(Operation::Subtract)),
        map_res(ws(tag("*")), |_| Ok::<Operation, ()>(Operation::Multiply)),
        map_res(ws(tag("/")), |_| Ok::<Operation, ()>(Operation::Divide)),
        map_res(ws(tag("%")), |_| Ok::<Operation, ()>(Operation::Mod)),
        map_res(ws(tag("^")), |_| Ok::<Operation, ()>(Operation::Power)),
        map_res(ws(tag("!")), |_| Ok::<Operation, ()>(Operation::Factorial)),
        map_res(ws(tag("=")), |_| Ok::<Operation, ()>(Operation::Equal)),
        map_res(ws(tag("!=")), |_| Ok::<Operation, ()>(Operation::NotEqual)),
        map_res(ws(tag("<")), |_| Ok::<Operation, ()>(Operation::LessThan)),
        map_res(ws(tag("<=")), |_| {
            Ok::<Operation, ()>(Operation::LessThanOrEqual)
        }),
        map_res(ws(tag(">=")), |_| {
            Ok::<Operation, ()>(Operation::GreaterThanOrEqual)
        }),
        map_res(ws(tag(">")), |_| {
            Ok::<Operation, ()>(Operation::GreaterThan)
        }),
    ))(input)
}

fn parse_unit(input: &str) -> IResult<&str, Unit> {
    alt((
        map_res(ws(tag_no_case("°")), |_| Ok::<Unit, ()>(Unit::Second)),
        map_res(ws(tag_no_case("\'")), |_| Ok::<Unit, ()>(Unit::Second)),
        map_res(ws(tag_no_case("\'\'")), |_| Ok::<Unit, ()>(Unit::Minute)),
        map_res(ws(tag_no_case("rad")), |_| Ok::<Unit, ()>(Unit::Radian)),
        map_res(ws(tag_no_case("grad")), |_| Ok::<Unit, ()>(Unit::Gradian)),
        map_res(ws(tag_no_case("custom")), |s: &str| {
            Ok::<Unit, ()>(Unit::Custom(s.to_string()))
        }),
    ))(input)
}

// TODO: function to convert to Token::Number
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

fn octal(input: &str) -> IResult<&str, &str> {
    preceded(
        alt((tag("0o"), tag("0O"))),
        recognize(many1(terminated(one_of("01234567"), many0(char('_'))))),
    )(input)
}

fn binary(input: &str) -> IResult<&str, &str> {
    preceded(
        alt((tag("0b"), tag("0B"))),
        recognize(many1(terminated(one_of("01"), many0(char('_'))))),
    )(input)
}

fn decimal(input: &str) -> IResult<&str, &str> {
    recognize(many1(terminated(one_of("0123456789"), many0(char('_')))))(input)
}

fn float(input: &str) -> IResult<&str, &str> {
    alt((
        // Case one: .42
        recognize(tuple((
            char('.'),
            decimal,
            opt(tuple((one_of("eE"), opt(one_of("+-")), decimal))),
        ))), // Case two: 42e42 and 42.42e42
        recognize(tuple((
            decimal,
            opt(preceded(char('.'), decimal)),
            one_of("eE"),
            opt(one_of("+-")),
            decimal,
        ))), // Case three: 42. and 42.42
        recognize(tuple((decimal, char('.'), opt(decimal)))),
    ))(input)
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
    alt((ws(tag("(")), ws(tag(")")), ws(tag(","))))(input)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_operation() {
        assert_eq!(
            parse_operation("    \n\t+  a  "),
            Ok(("a  ", Operation::Add))
        );
    }

    #[test]
    fn test_unit() {
        assert_eq!(parse_unit("    \n\t\'    "), Ok(("", Unit::Second)));
    }
}
