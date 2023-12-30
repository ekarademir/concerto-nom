use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::space0,
    combinator::into,
    error::context,
    sequence::{preceded, tuple},
    Parser,
};

use super::default_value::{self, DefaultValue};
use super::CResult;
use crate::parser::common::{keywords, regex_value};

#[derive(Debug, PartialEq, Clone)]
pub enum MetaProperty {
    Default(DefaultValue),
    Regex(String),
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
            into(default_value::default_metaproperty_parser),
            regex_meta_property,
            string_meta_property,
        )),
    )(input)
}

pub(super) fn regex_meta_property<'a>(input: &'a str) -> CResult<&'a str, MetaProperty> {
    context(
        "RegexMetaProperty",
        preceded(
            tuple((keywords::regex, space0, tag("="), space0)),
            regex_value,
        )
        .map(|s| MetaProperty::Regex(s)),
    )(input)
}

pub(super) fn string_meta_property<'a>(input: &'a str) -> CResult<&'a str, MetaProperty> {
    context(
        "StringMetaProperty",
        alt((into(default_value::string_default), regex_meta_property)),
    )(input)
}

#[cfg(test)]
mod test {
    #[test]
    fn test_regex_meta() {
        assert_eq!(
            super::regex_meta_property("regex=/abc.*/"),
            Ok(("", super::MetaProperty::Regex("abc.*".to_string()))),
            "Should parse a regex meta property"
        );
    }
}
