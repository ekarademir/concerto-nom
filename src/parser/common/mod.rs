use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric0},
    combinator::{recognize, value},
    error::context,
    sequence::pair,
};

mod numeric;
mod string;

pub(crate) mod concerto;
pub(crate) mod keywords;
pub(crate) use numeric::integer_value;
pub(crate) use numeric::long_value;
pub(crate) use string::string_value;

use crate::parser::CResult;

/// A `token` starts with a letter and includes alphanumerical characters
pub(crate) fn boolean_value<'a>(input: &'a str) -> CResult<&'a str, bool> {
    context(
        "Boolean",
        alt((value(true, tag("true")), value(false, tag("false")))),
    )(input)
}

/// A `token` starts with a letter and includes alphanumerical characters
pub(crate) fn token<'a>(input: &'a str) -> CResult<&'a str, &'a str> {
    context("Token", recognize(pair(alpha1, alphanumeric0)))(input)
}

#[cfg(test)]
mod test {
    #[test]
    fn test_token() {
        assert_eq!(
            super::token("a123"),
            Ok(("", "a123")),
            "Should parse token starting with a letter"
        );
        assert_eq!(
            super::token("foo"),
            Ok(("", "foo")),
            "Should parse token with just letters"
        );
        assert!(
            super::token("1foo").is_err(),
            "Should not parse token starting with number"
        );
    }

    #[test]
    fn test_boolean() {
        assert_eq!(
            super::boolean_value("true"),
            Ok(("", true)),
            "Should parse `true` value"
        );
        assert_eq!(
            super::boolean_value("false"),
            Ok(("", false)),
            "Should parse `false` value"
        );
        assert!(
            super::boolean_value("unknown").is_err(),
            "Should not parse values other than true or false"
        );
    }
}
