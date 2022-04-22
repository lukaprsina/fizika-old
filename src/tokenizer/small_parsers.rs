use super::Number;
use crate::tokenizer::{Operation, Token, Unit};
use std::{cmp::Ordering, collections::HashMap, num::NonZeroUsize};

use lazy_static::lazy_static;
use nom::{
    branch::alt,
    bytes::complete::{is_not, tag, tag_no_case, take_until},
    character::complete::{char, multispace0, one_of},
    combinator::{complete, map_res, opt, recognize, value},
    error::{self, Error, ErrorKind, ParseError},
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

fn add_str_token_pairs(
    pairs: &'static [(&str, &Token)],
) -> Vec<HashMap<&'static str, &'static Token>> {
    let mut result: Vec<HashMap<&'static str, &Token>> = vec![];

    for (name, token) in pairs {
        let index = name.len() - 1;

        if let Some(map) = result.get_mut(index) {
            map.insert(name, &token);
        } else {
            let mut map = HashMap::new();
            map.insert(*name, *token);
            result.resize(index, HashMap::new());
            result.insert(index, map);
        }
    }
    result
}

lazy_static! {
    static ref BINARY_EXPRESSION_MAPS: Vec<HashMap<&'static str, &'static Token>> = {
        add_str_token_pairs(&[
            ("+", &Token::Binary(Operation::Add)),
            ("-", &Token::Binary(Operation::Subtract)),
            ("*", &Token::Binary(Operation::Multiply)),
            ("/", &Token::Binary(Operation::Divide)),
            ("%", &Token::Binary(Operation::Mod)),
            ("^", &Token::Binary(Operation::Power)),
            ("!", &Token::Binary(Operation::Factorial)),
            ("=", &Token::Binary(Operation::Equal)),
            ("<", &Token::Binary(Operation::LessThan)),
            (">", &Token::Binary(Operation::GreaterThan)),
            ("<=", &Token::Binary(Operation::LessThanOrEqual)),
            (">=", &Token::Binary(Operation::GreaterThanOrEqual)),
            ("!=", &Token::Binary(Operation::NotEqual)),
        ])
    };
}

fn parse_and_map<'a, OutputType, FuncError>(
    name: &'a str,
    function: impl FnMut(&'a str) -> Result<OutputType, FuncError>,
) -> impl FnMut(&'a str) -> IResult<&'a str, OutputType> {
    map_res(trim(tag_no_case(name)), function)
}

fn parse_binary_expressions(input: &str) -> IResult<&str, Token> {
    for (position, map) in BINARY_EXPRESSION_MAPS.iter().enumerate().rev() {
        let length = position + 1;

        let work_string = match input.len().cmp(&length) {
            Ordering::Less => continue,
            Ordering::Equal => input,
            Ordering::Greater => &input[..length],
        };

        if let Some(&token) = map.get(work_string) {
            return Ok((&input[length..], token.clone()));
        }
    }

    Err(nom::Err::Error(error::Error {
        input,
        code: ErrorKind::LengthValue,
    }))
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
        trim(parse_float),
        trim(parse_hexadecimal),
        trim(parse_octal),
        trim(parse_binary),
        trim(parse_decimal),
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

/*
    All of the following functions are used directly for parsing.
*/

fn parse_unary_sign(input: &str) -> IResult<&str, Token> {
    alt((
        parse_and_map("+", |_| Ok::<Token, ()>(Token::Unary(Operation::Add))),
        parse_and_map("-", |_| Ok::<Token, ()>(Token::Unary(Operation::Subtract))),
    ))(input)
}

fn parse_factorial(input: &str) -> IResult<&str, Token> {
    parse_and_map("!", |_| Ok::<Token, ()>(Token::Unary(Operation::Factorial)))(input)
}

fn parse_left_parenthesis(input: &str) -> IResult<&str, Token> {
    parse_and_map("(", |_| Ok::<Token, ()>(Token::LeftParenthesis))(input)
}

fn parse_right_parenthesis(input: &str) -> IResult<&str, Token> {
    parse_and_map(")", |_| Ok::<Token, ()>(Token::RightParenthesis))(input)
}

fn parse_comma(input: &str) -> IResult<&str, Token> {
    parse_and_map(",", |_| Ok::<Token, ()>(Token::Comma))(input)
}

pub(crate) fn parse_left_expression(input: &str) -> IResult<&str, Token> {
    alt((
        trim(parse_number),
        trim(parse_function),
        trim(parse_variable),
        trim(parse_unary_sign),
        trim(parse_left_parenthesis),
    ))(input)
}

pub(crate) fn parse_right_expression(input: &str) -> IResult<&str, Token> {
    alt((
        trim(parse_factorial),
        trim(parse_binary_expressions),
        trim(parse_right_parenthesis),
    ))(input)
}

pub(crate) fn parse_right_expression_no_parenthesis(input: &str) -> IResult<&str, Token> {
    alt((trim(parse_factorial), trim(parse_binary_expressions)))(input)
}

pub(crate) fn parse_right_expression_with_comma(input: &str) -> IResult<&str, Token> {
    alt((
        trim(parse_factorial),
        trim(parse_binary_expressions),
        trim(parse_right_parenthesis),
        trim(parse_comma),
    ))(input)
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
    fn test_binary_expressions() {
        let cases = [
            ("+", Token::Binary(Operation::Add)),
            ("-", Token::Binary(Operation::Subtract)),
            ("*", Token::Binary(Operation::Multiply)),
            ("/", Token::Binary(Operation::Divide)),
            ("%", Token::Binary(Operation::Mod)),
            ("^", Token::Binary(Operation::Power)),
            ("!", Token::Binary(Operation::Factorial)),
            ("=", Token::Binary(Operation::Equal)),
            ("!=", Token::Binary(Operation::NotEqual)),
            ("<", Token::Binary(Operation::LessThan)),
            ("<=", Token::Binary(Operation::LessThanOrEqual)),
            (">=", Token::Binary(Operation::GreaterThanOrEqual)),
            (">", Token::Binary(Operation::GreaterThan)),
        ];

        for case in cases {
            assert_eq!(case.1, parse_binary_expressions(case.0).unwrap().1);
        }
    }

    #[test]
    fn test_parse_unit() {
        let cases = [
            ("grad", Unit::Gradian),
            ("rad", Unit::Radian),
            ("\'\'", Unit::Minute),
            ("\'", Unit::Second),
            ("°", Unit::Second),
        ];

        for case in cases {
            assert_eq!(case.1, parse_unit(case.0).unwrap().1);
        }
    }

    #[test]
    fn test_parse_hexadecimal() {
        assert_eq!(
            Ok(("", Token::Number(Number::Int(0x1A), None))),
            parse_hexadecimal("0x1A")
        );
    }

    #[test]
    fn test_parse_octal() {
        assert_eq!(
            Ok(("", Token::Number(Number::Int(0o73), None))),
            parse_octal("0o73")
        );
    }

    #[test]
    fn test_parse_binary() {
        assert_eq!(
            Ok(("", Token::Number(Number::Int(0b011001), None))),
            parse_binary("0b011001")
        );
    }

    #[test]
    fn test_parse_decimal() {
        assert_eq!(
            Ok(("", Token::Number(Number::Int(297), None))),
            parse_decimal("297")
        );
    }

    #[test]
    fn test_parse_float() {
        assert_eq!(
            Ok(("", Token::Number(Number::Float(0.42), None))),
            parse_float(".42")
        );

        assert_eq!(
            Ok(("", Token::Number(Number::Float(10e3), None))),
            parse_float("10e3")
        );

        assert_eq!(
            Ok(("", Token::Number(Number::Float(10.1e3), None))),
            parse_float("10.1e3")
        );

        assert_eq!(
            Ok(("", Token::Number(Number::Float(297.42), None))),
            parse_float("297.42")
        );
    }

    #[test]
    fn test_parse_number() {
        let cases = [
            ("0x1A", Token::Number(Number::Int(0x1A), None)),
            ("0o73", Token::Number(Number::Int(0o73), None)),
            ("0b011001", Token::Number(Number::Int(0b011001), None)),
            ("297", Token::Number(Number::Int(297), None)),
            (".42", Token::Number(Number::Float(0.42), None)),
            ("10e3", Token::Number(Number::Float(10e3), None)),
            ("10.1e3", Token::Number(Number::Float(10.1e3), None)),
            ("297.42", Token::Number(Number::Float(297.42), None)),
        ];

        for case in cases {
            assert_eq!(case.1, parse_number(case.0).unwrap().1);
        }
    }

    #[test]
    fn test_parse_identifier() {
        assert_eq!(Ok(("", "abc")), parse_idenifier("abc"));
        assert_eq!(Ok(("", "Abc")), parse_idenifier("Abc"));
        assert_eq!(Ok(("", "_abc")), parse_idenifier("_abc"));
        assert_eq!(Ok(("", "a_Bc")), parse_idenifier("a_Bc"));
    }

    #[test]
    fn test_parse_variable() {
        assert_eq!(
            Ok(("", Token::Variable("abc".to_string()))),
            parse_variable("abc")
        );
        assert_eq!(
            Ok(("", Token::Variable("Abc".to_string()))),
            parse_variable("Abc")
        );
        assert_eq!(
            Ok(("", Token::Variable("_abc".to_string()))),
            parse_variable("_abc")
        );
        assert_eq!(
            Ok(("", Token::Variable("a_Bc".to_string()))),
            parse_variable("a_Bc")
        );
    }

    #[test]
    fn test_parse_function() {
        assert_eq!(
            Ok((")", Token::Function("abc".to_string(), None))),
            parse_function("abc()")
        );
        assert_eq!(
            Ok(("x)", Token::Function("Abc".to_string(), None))),
            parse_function("Abc(x)")
        );
        assert_eq!(
            Ok(("1)", Token::Function("_abc".to_string(), None))),
            parse_function("_abc(1)")
        );
        assert_eq!(
            Ok(("", Token::Function("a_Bc".to_string(), None))),
            parse_function("a_Bc(")
        );
    }
}
