pub mod ranged;

use nom::{
    branch::alt,
    character::complete::{char, space0},
    combinator::into,
    error::context,
    sequence::{preceded, tuple},
    Parser,
};

use crate::parser::{
    common::{keywords, regex_value},
    property::{
        default_value::{
            boolean_default, double_default, integer_default, long_default, string_default,
            DefaultValue,
        },
        meta_property::ranged::{
            double_range, integer_range, length_meta_property, long_range, LengthLimits,
            NumberRange,
        },
    },
    CResult,
};

#[derive(Debug, PartialEq, Clone)]
pub enum MetaProperty {
    Default(DefaultValue),
    Regex(String),
    Length(LengthLimits),
    Range(NumberRange),
}

impl From<DefaultValue> for MetaProperty {
    fn from(value: DefaultValue) -> Self {
        Self::Default(value)
    }
}

impl From<NumberRange> for MetaProperty {
    fn from(value: NumberRange) -> Self {
        Self::Range(value)
    }
}

pub(super) fn regex_meta_property<'a>(input: &'a str) -> CResult<&'a str, MetaProperty> {
    context(
        "RegexMetaProperty",
        preceded(
            tuple((keywords::regex, space0, char('='), space0)),
            regex_value,
        )
        .map(|s| MetaProperty::Regex(s)),
    )(input)
}

pub(super) fn string_meta_property<'a>(input: &'a str) -> CResult<&'a str, MetaProperty> {
    context(
        "DefaultStringMetaProperty",
        alt((
            into(string_default),
            regex_meta_property,
            length_meta_property,
        )),
    )(input)
}

pub(super) fn double_meta_property<'a>(input: &'a str) -> CResult<&'a str, MetaProperty> {
    context(
        "DefaultDoubleMetaProperty",
        alt((into(double_default), double_range)),
    )(input)
}

pub(super) fn long_meta_property<'a>(input: &'a str) -> CResult<&'a str, MetaProperty> {
    context(
        "DefaultLongMetaProperty",
        alt((into(long_default), long_range)),
    )(input)
}

pub(super) fn boolean_meta_property<'a>(input: &'a str) -> CResult<&'a str, MetaProperty> {
    context("DefaultBooleanMetaProperty", alt((into(boolean_default),)))(input)
}

pub(super) fn integer_meta_property<'a>(input: &'a str) -> CResult<&'a str, MetaProperty> {
    context(
        "DefaultIntegerMetaProperty",
        alt((into(integer_default), integer_range)),
    )(input)
}

pub(super) fn meta_property<'a>(input: &'a str) -> CResult<&'a str, MetaProperty> {
    context(
        "MetaProperty",
        alt((
            string_meta_property,
            boolean_meta_property,
            integer_meta_property,
            long_meta_property,
            double_meta_property,
        )),
    )(input)
}

#[cfg(test)]
mod test {
    use crate::parser::property::meta_property::ranged::{DoubleRange, LongRange};
    use crate::parser::property::meta_property::LengthLimits;

    #[test]
    fn test_double_meta() {
        assert_eq!(
            super::double_meta_property("default = 3.14"),
            Ok((
                "",
                super::MetaProperty::Default(super::DefaultValue::DoubleDefaultValue(3.14))
            )),
            "Should parse a default meta property for Long"
        );

        assert_eq!(
            super::double_meta_property("range = [0.0, 42.0]"),
            Ok((
                "",
                super::MetaProperty::Range(super::NumberRange::Double(DoubleRange {
                    start: Some(0.0),
                    end: Some(42.0)
                }))
            )),
            "Should parse a range meta property for Long"
        );
    }

    #[test]
    fn test_long_meta() {
        assert_eq!(
            super::long_meta_property("default = 3147483647"),
            Ok((
                "",
                super::MetaProperty::Default(super::DefaultValue::LongDefaultValue(3147483647))
            )),
            "Should parse a default meta property for Long"
        );

        assert_eq!(
            super::long_meta_property("range = [0, 42]"),
            Ok((
                "",
                super::MetaProperty::Range(super::NumberRange::Long(LongRange {
                    start: Some(0),
                    end: Some(42)
                }))
            )),
            "Should parse a range meta property for Long"
        );
    }

    #[test]
    fn test_string_meta() {
        assert_eq!(
            super::string_meta_property("default = 'abc.*'"),
            Ok((
                "",
                super::MetaProperty::Default(super::DefaultValue::StringDefaultValue(
                    "abc.*".to_string()
                ))
            )),
            "Should parse a default string meta property"
        );

        assert_eq!(
            super::string_meta_property("length = [0, 42]"),
            Ok((
                "",
                super::MetaProperty::Length(LengthLimits {
                    start: Some(0),
                    end: Some(42),
                })
            )),
            "Should parse a default string meta property"
        );
        assert_eq!(
            super::string_meta_property("regex=/abc.*/"),
            Ok(("", super::MetaProperty::Regex("abc.*".to_string()))),
            "Should parse a regex meta property"
        );
    }
}
