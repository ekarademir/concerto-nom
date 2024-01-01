use nom::{
    branch::alt,
    character::complete::{char, space0},
    combinator::into,
    error::context,
    sequence::{delimited, preceded, separated_pair, terminated, tuple},
    Parser,
};

use super::default_value::{
    boolean_default, double_default, integer_default, long_default, string_default, DefaultValue,
};
use super::CResult;
use crate::parser::{
    common::{
        double_value, integer_value, keywords, long_value, numeric::positive_integer_value,
        regex_value,
    },
    error::CError,
};

struct Ranged<T> {
    start: Option<T>,
    end: Option<T>,
}

fn ranged_parser<
    'a,
    T,
    P: Parser<&'a str, T, CError<&'a str>> + Copy,
    KV: Parser<&'a str, &'a str, CError<&'a str>>,
>(
    input: &'a str,
    keyword: KV,
    parser: P,
) -> CResult<&'a str, Ranged<T>> {
    let only_start = context(
        "LengthOnlyStart",
        terminated(parser, tuple((space0, char(','), space0))),
    )
    .map(|start: T| Ranged {
        start: Some(start),
        end: None,
    });

    let only_end = context(
        "LengthOnlyEnd",
        preceded(tuple((space0, char(','), space0)), parser),
    )
    .map(|end: T| Ranged {
        end: Some(end),
        start: None,
    });

    let full = context(
        "LengthFull",
        separated_pair(parser, tuple((space0, char(','), space0)), parser),
    )
    .map(|(start, end): (T, T)| Ranged {
        start: Some(start),
        end: Some(end),
    });

    context(
        "LengthMetaProperty",
        delimited(
            tuple((keyword, space0, char('='), space0, char('['), space0)),
            alt((full, only_start, only_end)),
            tuple((space0, char(']'))),
        ),
    )(input)
}

#[derive(Debug, PartialEq, Clone)]
pub enum NumberRange {
    Long(LongRange),
    Integer(IntegerRange),
    Double(DoubleRange),
}

#[derive(Debug, PartialEq, Clone)]
pub struct LengthLimits {
    pub start: Option<i32>,
    pub end: Option<i32>,
}

impl From<Ranged<i32>> for LengthLimits {
    fn from(value: Ranged<i32>) -> Self {
        Self {
            start: value.start,
            end: value.end,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct LongRange {
    pub start: Option<i64>,
    pub end: Option<i64>,
}

impl From<Ranged<i64>> for NumberRange {
    fn from(value: Ranged<i64>) -> Self {
        Self::Long(LongRange {
            start: value.start,
            end: value.end,
        })
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct IntegerRange {
    pub start: Option<i32>,
    pub end: Option<i32>,
}

impl From<Ranged<i32>> for NumberRange {
    fn from(value: Ranged<i32>) -> Self {
        Self::Integer(IntegerRange {
            start: value.start,
            end: value.end,
        })
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct DoubleRange {
    pub start: Option<f64>,
    pub end: Option<f64>,
}

impl From<Ranged<f64>> for NumberRange {
    fn from(value: Ranged<f64>) -> Self {
        Self::Double(DoubleRange {
            start: value.start,
            end: value.end,
        })
    }
}

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

pub(super) fn length_meta_property<'a>(input: &'a str) -> CResult<&'a str, MetaProperty> {
    match ranged_parser(input, keywords::length, positive_integer_value) {
        Err(e) => Err(e),
        Ok((remains, ranged)) => Ok((remains, MetaProperty::Length(ranged.into()))),
    }
}

pub(super) fn integer_range<'a>(input: &'a str) -> CResult<&'a str, MetaProperty> {
    match ranged_parser(input, keywords::range, integer_value) {
        Err(e) => Err(e),
        Ok((remains, ranged)) => Ok((remains, MetaProperty::Range(ranged.into()))),
    }
}

pub(super) fn double_range<'a>(input: &'a str) -> CResult<&'a str, MetaProperty> {
    match ranged_parser(input, keywords::range, double_value) {
        Err(e) => Err(e),
        Ok((remains, ranged)) => Ok((remains, MetaProperty::Range(ranged.into()))),
    }
}

pub(super) fn long_range<'a>(input: &'a str) -> CResult<&'a str, MetaProperty> {
    match ranged_parser(input, keywords::range, long_value) {
        Err(e) => Err(e),
        Ok((remains, ranged)) => Ok((remains, MetaProperty::Range(ranged.into()))),
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
                super::MetaProperty::Range(super::NumberRange::Double(super::DoubleRange {
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
                super::MetaProperty::Range(super::NumberRange::Long(super::LongRange {
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

    #[test]
    fn test_length_meta() {
        assert_eq!(
            super::length_meta_property("length = [ 100, ]"),
            Ok((
                "",
                super::MetaProperty::Length(LengthLimits {
                    start: Some(100),
                    end: None
                })
            )),
            "Should parse a length meta with start only"
        );

        assert_eq!(
            super::length_meta_property("length = [ , 100 ]"),
            Ok((
                "",
                super::MetaProperty::Length(LengthLimits {
                    end: Some(100),
                    start: None
                })
            )),
            "Should parse a length meta with end only"
        );

        assert_eq!(
            super::length_meta_property("length = [ 1, 100 ]"),
            Ok((
                "",
                super::MetaProperty::Length(LengthLimits {
                    start: Some(1),
                    end: Some(100),
                })
            )),
            "Should parse a length meta with full range"
        );

        assert!(
            super::length_meta_property("length = [ - 1, 100 ]").is_err(),
            "Should not parse a length meta negative numbers"
        );

        assert!(
            super::length_meta_property("length = [ , -100 ]").is_err(),
            "Should not parse a length meta negative numbers"
        );

        assert!(
            super::length_meta_property("length = [ ]").is_err(),
            "Should not parse a length meta with empty definition"
        );
    }
}
