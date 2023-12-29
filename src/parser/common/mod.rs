use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric0},
    combinator::{recognize, value},
    error::{context, ContextError, ParseError},
    sequence::pair,
    IResult,
};

mod numeric;
mod string;

pub(crate) mod concerto;
pub(crate) mod keywords;
pub(crate) use numeric::integer_parser;
pub(crate) use numeric::long_parser;
pub(crate) use string::string_parser;

/// A `token` starts with a letter and includes alphanumerical characters
pub(crate) fn boolean_parser<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, bool, E> {
    context(
        "Boolean",
        alt((value(true, tag("true")), value(false, tag("false")))),
    )(input)
}

/// A `token` starts with a letter and includes alphanumerical characters
pub(crate) fn token_parser<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, &'a str, E> {
    context("Token", recognize(pair(alpha1, alphanumeric0)))(input)
}

#[cfg(test)]
mod test {
    use nom::error::VerboseError;

    #[test]
    fn test_token() {
        assert_eq!(
            super::token_parser::<VerboseError<&str>>("a123"),
            Ok(("", "a123")),
            "Should parse token starting with a letter"
        );
        assert_eq!(
            super::token_parser::<VerboseError<&str>>("foo"),
            Ok(("", "foo")),
            "Should parse token with just letters"
        );
        assert!(
            super::token_parser::<VerboseError<&str>>("1foo").is_err(),
            "Should not parse token starting with number"
        );
    }

    #[test]
    fn test_boolean() {
        assert_eq!(
            super::boolean_parser::<VerboseError<&str>>("true"),
            Ok(("", true)),
            "Should parse `true` value"
        );
        assert_eq!(
            super::boolean_parser::<VerboseError<&str>>("false"),
            Ok(("", false)),
            "Should parse `false` value"
        );
        assert!(
            super::boolean_parser::<VerboseError<&str>>("unknown").is_err(),
            "Should not parse values other than true or false"
        );
    }
}
