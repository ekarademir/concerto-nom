use nom::{
    branch::{alt, permutation},
    character::complete::{char, space0, space1},
    combinator::{into, opt},
    error::context,
    sequence::{preceded, separated_pair, terminated, tuple},
    Parser,
};

use crate::parser::{
    common::{keywords, numeric::positive_integer_value, regex_value, string_value},
    property::{
        internal::{primitive_property, PrimitiveType},
        meta_property::ranged::{ranged_parser, Ranged},
    },
    CResult,
};

#[derive(Debug, PartialEq, Clone)]
pub struct StringProperty {
    pub name: String,
    pub default_value: Option<String>,
    pub regex_validator: Option<StringRegexValidator>,
    pub length_validator: Option<StringLengthValidator>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct StringRegexValidator {
    pub pattern: String,
    pub flags: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct StringLengthValidator {
    pub min_length: Option<i32>,
    pub max_length: Option<i32>,
}

impl From<Ranged<i32>> for StringLengthValidator {
    fn from(value: Ranged<i32>) -> Self {
        Self {
            max_length: value.end,
            min_length: value.start,
        }
    }
}

pub fn string_property<'a>(input: &'a str) -> CResult<&'a str, StringProperty> {
    let property_with_meta = separated_pair(
        primitive_property(PrimitiveType::StringPropertyType),
        space1,
        permutation((
            opt(string_default_value),
            opt(string_regex_validator),
            opt(context("StringLengthValidator", string_length_validator)),
        )),
    );
    let property_without_meta = terminated(
        primitive_property(PrimitiveType::StringPropertyType),
        space0,
    )
    .map(|property_name: &'a str| (property_name, (None, None, None)));

    context(
        "StringProperty",
        alt((property_with_meta, property_without_meta)).map(
            |(property_name, (default_value, regex_validator, length_validator))| StringProperty {
                name: String::from(property_name),
                default_value,
                regex_validator,
                length_validator,
            },
        ),
    )(input)
}

pub fn string_default_value<'a>(input: &'a str) -> CResult<&'a str, String> {
    into(context(
        "StringDefaultValue",
        preceded(
            tuple((keywords::default, space0, char('='), space0)),
            string_value,
        ),
    ))(input)
}

pub fn string_regex_validator<'a>(input: &'a str) -> CResult<&'a str, StringRegexValidator> {
    context(
        "StringRegexValidator",
        preceded(
            tuple((keywords::regex, space0, char('='), space0)),
            regex_value,
        )
        .map(|s| StringRegexValidator {
            pattern: s,
            flags: "".to_string(),
        }),
    )(input)
}

pub fn string_length_validator<'a>(input: &'a str) -> CResult<&'a str, StringLengthValidator> {
    match ranged_parser(input, keywords::length, positive_integer_value) {
        Err(e) => Err(e),
        Ok((remains, ranged)) => Ok((remains, ranged.into())),
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_string_property() {
        assert_eq!(
            super::string_property("o String foo"),
            Ok((
                "",
                super::StringProperty {
                    name: String::from("foo"),
                    default_value: None,
                    regex_validator: None,
                    length_validator: None
                }
            )),
            "Should parse string with no meta properties"
        );

        assert_eq!(
            super::string_property("o String baz default=\"Hello World\""),
            Ok((
                "",
                super::StringProperty {
                    name: String::from("baz"),
                    default_value: Some(String::from("Hello World")),
                    regex_validator: None,
                    length_validator: None
                }
            )),
            "Should parse string with default value only"
        );

        assert_eq!(
            super::string_property("o String baz regex=/abc.*/"),
            Ok((
                "",
                super::StringProperty {
                    name: String::from("baz"),
                    default_value: None,
                    regex_validator: Some(super::StringRegexValidator {
                        pattern: String::from("abc.*"),
                        flags: String::from("")
                    }),
                    length_validator: None
                }
            )),
            "Should parse string with regex value only"
        );

        assert_eq!(
            super::string_property("o String baz length=[0, 10]"),
            Ok((
                "",
                super::StringProperty {
                    name: String::from("baz"),
                    default_value: None,
                    regex_validator: None,
                    length_validator: Some(super::StringLengthValidator {
                        min_length: Some(0),
                        max_length: Some(10)
                    })
                }
            )),
            "Should parse string with length only"
        );

        assert_eq!(
            super::string_property(
                "o String baz length=[,100] regex=/abc.*/ default=\"Hello World\""
            ),
            Ok((
                "",
                super::StringProperty {
                    name: String::from("baz"),
                    default_value: Some(String::from("Hello World")),
                    regex_validator: Some(super::StringRegexValidator {
                        pattern: String::from("abc.*"),
                        flags: String::from("")
                    }),
                    length_validator: Some(super::StringLengthValidator {
                        min_length: None,
                        max_length: Some(100)
                    })
                }
            )),
            "Should parse string with both default and regex and length"
        );
    }
}
