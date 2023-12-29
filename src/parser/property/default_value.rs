use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::space0,
    combinator::into,
    error::context,
    sequence::{preceded, tuple},
    Parser,
};

use crate::parser::{
    common::{boolean_value, integer_value, keywords, long_value, string_value},
    CResult,
};

#[derive(Debug, PartialEq, Clone)]
pub enum DefaultValue {
    StringDefaultValue(String),
    BooleanDefaultValue(bool),
    LongDefaultValue(i64),
    DoubleDefaultValue(f64),
    IntegerDefaultValue(i32),
    DateTimeDefaultValue(String),
}

impl From<String> for DefaultValue {
    fn from(value: String) -> Self {
        Self::StringDefaultValue(value)
    }
}

impl From<bool> for DefaultValue {
    fn from(value: bool) -> Self {
        Self::BooleanDefaultValue(value)
    }
}

impl From<i32> for DefaultValue {
    fn from(value: i32) -> Self {
        Self::IntegerDefaultValue(value)
    }
}

impl From<i64> for DefaultValue {
    fn from(value: i64) -> Self {
        Self::LongDefaultValue(value)
    }
}

fn default_value<'a>(input: &'a str) -> CResult<&'a str, DefaultValue> {
    let string_default_value = context("StringDefaultValue", string_value);
    let boolean_default_value = context("BooleanDefaultValue", boolean_value);
    let integer_default_value = context("IntegerDefaultValue", integer_value);
    let long_default_value = context("LongDefaultValue", long_value);

    context(
        "DefaultValue",
        alt((
            into(string_default_value),
            into(boolean_default_value),
            into(integer_default_value),
            into(long_default_value),
        )),
    )
    .parse(input)
}

pub(super) fn default_metaproperty_parser<'a>(input: &'a str) -> CResult<&'a str, DefaultValue> {
    context(
        "DefaultMetaProperty",
        preceded(
            tuple((keywords::default, space0, tag("="), space0)),
            default_value,
        ),
    )(input)
}

#[cfg(test)]
mod test {
    #[test]
    fn test_default_metaproperty_string() {
        assert_eq!(
            super::default_metaproperty_parser("default=\"Hello World\""),
            Ok((
                "",
                super::DefaultValue::StringDefaultValue("Hello World".to_string())
            )),
            "Should parse default value of a String, delimited by double quotes"
        );

        assert_eq!(
            super::default_metaproperty_parser("default=\'Hello World\'"),
            Ok((
                "",
                super::DefaultValue::StringDefaultValue("Hello World".to_string())
            )),
            "Should parse default value of a String, delimited by single quotes"
        );

        assert_eq!(
            super::default_metaproperty_parser("default=true"),
            Ok(("", super::DefaultValue::BooleanDefaultValue(true))),
            "Should parse default value of a Boolean"
        );

        assert_eq!(
            super::default_metaproperty_parser("default=123"),
            Ok(("", super::DefaultValue::IntegerDefaultValue(123))),
            "Should parse default value of an Integer"
        );

        assert_eq!(
            super::default_metaproperty_parser("default=3147483647"),
            Ok(("", super::DefaultValue::LongDefaultValue(3147483647))),
            "Should parse default value of an Long"
        );
    }
}
