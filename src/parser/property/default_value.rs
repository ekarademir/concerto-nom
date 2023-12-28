use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::space0,
    error::{context, ContextError, ParseError},
    sequence::{preceded, tuple},
    IResult, Parser,
};

use crate::parser::common::{keywords, string_parser as string_default_value};

#[derive(Debug, PartialEq, Clone)]
pub enum DefaultValue {
    StringDefaultValue(String),
    BooleanDefaultValue(bool),
    LongDefaultValue(i64),
    DoubleDefaultValue(f64),
    IntegerDefaultValue(i64),
    DateTimeDefaultValue(String),
}

impl From<String> for DefaultValue {
    fn from(value: String) -> Self {
        Self::StringDefaultValue(value)
    }
}

fn default_value_parser<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, DefaultValue, E> {
    Parser::into(context("DefaultValue", alt((string_default_value,)))).parse(input)
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
    fn test_default_metaproperty_string() {
        assert_eq!(
            super::default_metaproperty_parser::<VerboseError<&str>>("default=\"Hello World\""),
            Ok((
                "",
                super::DefaultValue::StringDefaultValue("Hello World".to_string())
            )),
            "Should parse default value of a String, delimited by double quotes"
        );

        assert_eq!(
            super::default_metaproperty_parser::<VerboseError<&str>>("default=\'Hello World\'"),
            Ok((
                "",
                super::DefaultValue::StringDefaultValue("Hello World".to_string())
            )),
            "Should parse default value of a String, delimited by single quotes"
        );
    }
}
