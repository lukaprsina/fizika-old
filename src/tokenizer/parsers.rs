use super::Number;
use crate::tokenizer::{Operation, Token, Unit};
use std::num::NonZeroUsize;

use nom::{
    branch::alt,
    bytes::complete::{is_not, tag, tag_no_case, take_until},
    character::complete::{char, multispace0, one_of},
    combinator::{complete, map_res, opt, recognize, value},
    error::{Error, ErrorKind, ParseError},
    multi::{many0, many1},
    sequence::{delimited, pair, preceded, terminated, tuple},
    Err, IResult, Needed,
};

// TODO: add whitespace
fn parse_eol_comment<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, (), E> {
    value(
        (), // Output is thrown away.
        pair(tag(r"//"), is_not("\n\r")),
    )(i)
}

fn parse_inline_comment<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, (), E> {
    value(
        (), // Output is thrown away.
        tuple((tag("/*"), take_until("*/"), tag("*/"))),
    )(i)
}

fn trim<'a, F: 'a, O, E: ParseError<&'a str>>(
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
    map_res(trim(tag_no_case(name)), function)
}

fn parse_operation(input: &str) -> IResult<&str, Operation> {
    alt((
        // same as map_res(ws(tag("+")), |_| Ok::<Operation, ()>(Operation::Add)),
        // longest operations first for corect parsing
        parse_and_map("=", |_| Ok::<Operation, ()>(Operation::Equal)),
        parse_and_map("!=", |_| Ok::<Operation, ()>(Operation::NotEqual)),
        parse_and_map("<=", |_| Ok::<Operation, ()>(Operation::LessThanOrEqual)),
        parse_and_map(">=", |_| Ok::<Operation, ()>(Operation::GreaterThanOrEqual)),
        parse_and_map("<", |_| Ok::<Operation, ()>(Operation::LessThan)),
        parse_and_map(">", |_| Ok::<Operation, ()>(Operation::GreaterThan)),
        parse_and_map("+", |_| Ok::<Operation, ()>(Operation::Add)),
        parse_and_map("-", |_| Ok::<Operation, ()>(Operation::Subtract)),
        parse_and_map("*", |_| Ok::<Operation, ()>(Operation::Multiply)),
        parse_and_map("/", |_| Ok::<Operation, ()>(Operation::Divide)),
        parse_and_map("%", |_| Ok::<Operation, ()>(Operation::Mod)),
        parse_and_map("^", |_| Ok::<Operation, ()>(Operation::Power)),
        parse_and_map("!", |_| Ok::<Operation, ()>(Operation::Factorial)),
    ))(input)
}

fn parse_unit(input: &str) -> IResult<&str, Unit> {
    alt((
        // add a custom unit
        parse_and_map("grad", |_| Ok::<Unit, ()>(Unit::Gradian)),
        parse_and_map("rad", |_| Ok::<Unit, ()>(Unit::Radian)),
        parse_and_map("\'\'", |_| Ok::<Unit, ()>(Unit::Minute)),
        parse_and_map("\'", |_| Ok::<Unit, ()>(Unit::Second)),
        parse_and_map("°", |_| Ok::<Unit, ()>(Unit::Second)),
    ))(input)
}

fn parse_hexadecimal(input: &str) -> IResult<&str, Token> {
    map_res(
        preceded(
            alt((tag("0x"), tag("0X"))),
            recognize(many1(terminated(
                one_of("0123456789abcdefABCDEF"),
                many0(char('_')),
            ))),
        ),
        |out: &str| -> Result<Token, std::num::ParseIntError> {
            let number = i64::from_str_radix(&str::replace(out, "_", ""), 16)?;
            Ok(Token::Number(Number::Int(number), None))
        },
    )(input)
}

fn parse_octal(input: &str) -> IResult<&str, Token> {
    map_res(
        preceded(
            alt((tag("0o"), tag("0O"))),
            recognize(many1(terminated(one_of("01234567"), many0(char('_'))))),
        ),
        |out: &str| -> Result<Token, std::num::ParseIntError> {
            let number = i64::from_str_radix(&str::replace(out, "_", ""), 8)?;
            Ok(Token::Number(Number::Int(number), None))
        },
    )(input)
}

fn parse_binary(input: &str) -> IResult<&str, Token> {
    map_res(
        preceded(
            alt((tag("0b"), tag("0B"))),
            recognize(many1(terminated(one_of("01"), many0(char('_'))))),
        ),
        |out: &str| -> Result<Token, std::num::ParseIntError> {
            let number = i64::from_str_radix(&str::replace(out, "_", ""), 2)?;
            Ok(Token::Number(Number::Int(number), None))
        },
    )(input)
}

fn parse_decimal(input: &str) -> IResult<&str, Token> {
    map_res(
        recognize(many1(terminated(one_of("0123456789"), many0(char('_'))))),
        |out: &str| -> Result<Token, std::num::ParseIntError> {
            let number = str::replace(out, "_", "").parse::<i64>()?;
            Ok(Token::Number(Number::Int(number), None))
        },
    )(input)
}

fn parse_float(input: &str) -> IResult<&str, Token> {
    alt((
        // Case one: .42
        map_res(
            recognize(tuple((
                char('.'),
                parse_decimal,
                opt(tuple((one_of("eE"), opt(one_of("+-")), parse_decimal))),
            ))),
            |out: &str| -> Result<Token, std::num::ParseFloatError> {
                let number = str::replace(out, "_", "").parse::<f64>()?;
                Ok(Token::Number(Number::Float(number), None))
            },
        ), // Case two: 42e42 and 42.42e42
        map_res(
            recognize(tuple((
                parse_decimal,
                opt(preceded(char('.'), parse_decimal)),
                one_of("eE"),
                opt(one_of("+-")),
                parse_decimal,
            ))),
            |out: &str| -> Result<Token, std::num::ParseFloatError> {
                let number = str::replace(out, "_", "").parse::<f64>()?;
                Ok(Token::Number(Number::Float(number), None))
            },
        ), // Case three: 42. and 42.42
        map_res(
            recognize(tuple((parse_decimal, char('.'), opt(parse_decimal)))),
            |out: &str| -> Result<Token, std::num::ParseFloatError> {
                let number = str::replace(out, "_", "").parse::<f64>()?;
                Ok(Token::Number(Number::Float(number), None))
            },
        ),
    ))(input)
}

fn parse_number(input: &str) -> IResult<&str, Token> {
    alt((
        trim(parse_hexadecimal),
        trim(parse_octal),
        trim(parse_binary),
        trim(parse_decimal),
        trim(parse_float),
    ))(input)
}

fn parse_idenifier(input: &str) -> IResult<&str, &str> {
    let mut iter = input.chars();
    if let Some(first_char) = iter.next() {
        match first_char {
            'a'..='z' | 'A'..='Z' | '_' => {
                let num_of_chars = iter
                    .take_while(|c| matches!(c, 'a'..='z' | 'A'..='Z' | '_' | '0'..='9'))
                    .count();

                let (out, rest) = input.split_at(num_of_chars + 1);
                Ok((rest, out))
            }
            _ => IResult::Err(Err::Error(Error {
                code: ErrorKind::Char,
                input,
            })),
        }
    } else {
        IResult::Err(Err::Incomplete(Needed::Size(NonZeroUsize::new(1).unwrap())))
    }
}

fn parse_variable(input: &str) -> IResult<&str, Token> {
    map_res(complete(parse_idenifier), |s| -> Result<Token, ()> {
        Ok(Token::Variable(s.to_string()))
    })(input)
}

fn parse_function(input: &str) -> IResult<&str, Token> {
    map_res(
        terminated(
            complete(parse_idenifier),
            preceded(multispace0, complete(tag("("))),
        ),
        |s: &str| -> Result<Token, ()> { Ok(Token::Function(s.to_string(), None)) },
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eol_comment() {
        let input = "// This is a 1 + 2 / x comment\n";
        let expected: Result<(&str, ()), nom::Err<nom::error::Error<&str>>> = Ok(("\n", ()));
        assert_eq!(expected, parse_eol_comment(input));
    }

    #[test]
    fn test_inline_comment() {
        let input = "/* alphabet
        
        should not be parsed x / 3
        */\n";
        let expected: Result<(&str, ()), nom::Err<nom::error::Error<&str>>> = Ok(("\n", ()));
        assert_eq!(expected, parse_inline_comment(input));
    }

    #[test]
    fn test_trim() {
        let input = " \t\n 1 + 2 \r\t\t      \n";
        let expected: Result<(&str, &str), nom::Err<nom::error::Error<&str>>> = Ok(("", "1 + 2"));
        assert_eq!(expected, trim(tag("1 + 2"))(input));
    }

    #[test]
    fn test_parse_and_map() {
        let input = "+";
        let expected: Result<(&str, ()), nom::Err<nom::error::Error<&str>>> = Ok(("", ()));
        assert_eq!(
            expected,
            parse_and_map("+", |_| -> Result<(), ()> { Ok(()) })(input)
        );
    }

    #[test]
    fn test_parse_operation() {
        let cases = [
            ("+", Operation::Add),
            ("-", Operation::Subtract),
            ("*", Operation::Multiply),
            ("/", Operation::Divide),
            ("%", Operation::Mod),
            ("^", Operation::Power),
            ("!", Operation::Factorial),
            ("=", Operation::Equal),
            ("!=", Operation::NotEqual),
            ("<", Operation::LessThan),
            ("<=", Operation::LessThanOrEqual),
            (">=", Operation::GreaterThanOrEqual),
            (">", Operation::GreaterThan),
        ];

        for case in cases {
            assert_eq!(case.1, parse_operation(case.0).unwrap().1);
        }
    }

    #[test]
    fn test_parse_unit() {
        // TODO
    }

    #[test]
    fn test_parse_hexadecimal() {
        // TODO
    }

    #[test]
    fn test_parse_octal() {
        // TODO
    }

    #[test]
    fn test_parse_binary() {
        // TODO
    }

    #[test]
    fn test_parse_decimal() {
        // TODO
    }

    #[test]
    fn test_parse_float() {
        // TODO
    }

    #[test]
    fn test_parse_number() {
        // TODO
    }

    #[test]
    fn test_parse_identifier() {
        // TODO
    }

    #[test]
    fn test_parse_variable() {
        // TODO
    }

    #[test]
    fn test_parse_function() {
        // TODO
    }
}
