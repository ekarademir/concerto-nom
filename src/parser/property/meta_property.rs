use nom::{
    branch::alt,
    character::complete::{char, space0},
    combinator::into,
    error::context,
    sequence::{delimited, preceded, separated_pair, terminated, tuple},
    Parser,
};

use super::default_value::{default_metaproperty_parser, string_default, DefaultValue};
use super::CResult;
use crate::parser::common::{keywords, numeric::positive_integer_value, regex_value};

#[derive(Debug, PartialEq, Clone)]
pub struct IntegerRanged {
    pub start: Option<i32>,
    pub end: Option<i32>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum MetaProperty {
    Default(DefaultValue),
    Regex(String),
    Length(IntegerRanged),
}

impl From<DefaultValue> for MetaProperty {
    fn from(value: DefaultValue) -> Self {
        Self::Default(value)
    }
}

pub(super) fn meta_property<'a>(input: &'a str) -> CResult<&'a str, MetaProperty> {
    context(
        "MetaProperty",
        alt((
            into(default_metaproperty_parser),
            regex_meta_property,
            default_string_meta_property,
            length_meta_property,
        )),
    )(input)
}

pub(super) fn length_meta_property<'a>(input: &'a str) -> CResult<&'a str, MetaProperty> {
    let only_start = context(
        "LengthOnlyStart",
        terminated(positive_integer_value, tuple((space0, char(','), space0))),
    )
    .map(|start: i32| IntegerRanged {
        start: Some(start),
        end: None,
    });

    let only_end = context(
        "LengthOnlyEnd",
        preceded(tuple((space0, char(','), space0)), positive_integer_value),
    )
    .map(|end: i32| IntegerRanged {
        end: Some(end),
        start: None,
    });

    let full = context(
        "LengthFull",
        separated_pair(
            positive_integer_value,
            tuple((space0, char(','), space0)),
            positive_integer_value,
        ),
    )
    .map(|(start, end): (i32, i32)| IntegerRanged {
        start: Some(start),
        end: Some(end),
    });

    context(
        "LengthMetaProperty",
        delimited(
            tuple((
                keywords::length,
                space0,
                char('='),
                space0,
                char('['),
                space0,
            )),
            alt((full, only_start, only_end)),
            tuple((space0, char(']'))),
        )
        .map(|int_range: IntegerRanged| MetaProperty::Length(int_range)),
    )(input)
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

pub(super) fn default_string_meta_property<'a>(input: &'a str) -> CResult<&'a str, MetaProperty> {
    context(
        "DefaultStringMetaProperty",
        alt((into(string_default), regex_meta_property)),
    )(input)
}

#[cfg(test)]
mod test {
    use crate::parser::property::meta_property::IntegerRanged;

    #[test]
    fn test_regex_meta() {
        assert_eq!(
            super::regex_meta_property("regex=/abc.*/"),
            Ok(("", super::MetaProperty::Regex("abc.*".to_string()))),
            "Should parse a regex meta property"
        );
    }

    #[test]
    fn test_default_string_meta() {
        assert_eq!(
            super::default_string_meta_property("default = 'abc.*'"),
            Ok((
                "",
                super::MetaProperty::Default(super::DefaultValue::StringDefaultValue(
                    "abc.*".to_string()
                ))
            )),
            "Should parse a default string meta property"
        );
    }

    #[test]
    fn test_length_meta() {
        assert_eq!(
            super::length_meta_property("length = [ 100, ]"),
            Ok((
                "",
                super::MetaProperty::Length(IntegerRanged {
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
                super::MetaProperty::Length(IntegerRanged {
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
                super::MetaProperty::Length(IntegerRanged {
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
