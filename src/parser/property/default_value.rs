use nom::{
    branch::alt,
    bytes::complete::{tag, take_while},
    character::complete::space0,
    error::{context, ContextError, ParseError},
    sequence::{delimited, preceded, tuple},
    IResult, Parser,
};

use crate::parser::common::keywords;

#[derive(Debug, PartialEq, Clone)]
pub enum DefaultValue {
    StringDefaultValue(String),
    BooleanDefaultValue(bool),
    LongDefaultValue(i64),
    DoubleDefaultValue(f64),
    IntegerDefaultValue(i64),
    DateTimeDefaultValue(String),
}

fn string_value_parser<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, &'a str, E> {
    // Doesn't handle unicode escapes yet.
    let invalid = "\n\r\"\'";
    // TODO: Add punctuation
    let valid =
        take_while(|c: char| !invalid.contains(c) || c.is_alphanumeric() || c.is_whitespace());
    context("StringValue", valid)(input)
}

pub(super) fn default_value_parser<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, DefaultValue, E> {
    let string_default_value = context(
        "StringDefaultValue",
        alt((
            delimited(tag("\""), string_value_parser::<E>, tag("\"")),
            delimited(tag("'"), string_value_parser::<E>, tag("'")),
        )),
    )
    .map(|v| DefaultValue::StringDefaultValue(v.to_string()));

    context("DefaultValue", alt((string_default_value,)))(input)
}

pub(super) fn default_metaproperty_parser<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, DefaultValue, E> {
    context(
        "DefaultMetaProperty",
        preceded(
            tuple((keywords::default, space0, tag("="), space0)),
            default_value_parser,
        ),
    )(input)
}

#[cfg(test)]
mod test {
    use nom::error::VerboseError;

    #[test]
    fn test_default_metaproperty() {
        assert_eq!(
            super::default_metaproperty_parser::<VerboseError<&str>>("default=\"Hello World\""),
            Ok((
                "",
                super::DefaultValue::StringDefaultValue("Hello World".to_string())
            )),
        );
    }

    #[test]
    fn test_string_value() {
        assert_eq!(
            super::string_value_parser::<VerboseError<&str>>("lorem ipsum\t123"),
            Ok(("", "lorem ipsum\t123")),
            "Should parse letters, numbers, and spaces"
        );
    }
}
