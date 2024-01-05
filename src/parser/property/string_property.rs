use nom::{
    branch::alt,
    character::complete::{char, space0, space1},
    combinator::into,
    error::context,
    multi::fold_many_m_n,
    sequence::{preceded, tuple},
    Parser,
};
use serde_derive::Serialize;

use crate::parser::{
    common::{
        keywords,
        numeric::positive_integer_value,
        string::{regex_value, string_value},
    },
    property::internal::{primitive_property, ranged_parser, PrimitiveType, Ranged},
    CResult,
};

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct StringProperty {
    #[serde(rename = "$class")]
    pub class: String,
    pub name: String,
    #[serde(rename = "isOptional")]
    pub is_optional: bool,
    #[serde(rename = "isArray")]
    pub is_array: bool,
    #[serde(rename = "default")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String>,
    #[serde(rename = "regex")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regex_validator: Option<StringRegexValidator>,
    #[serde(rename = "length")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub length_validator: Option<StringLengthValidator>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct StringRegexValidator {
    pub pattern: String,
    pub flags: String,
}

impl serde::Serialize for StringRegexValidator {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&String::from(self))
    }
}

impl From<&StringRegexValidator> for String {
    fn from(value: &StringRegexValidator) -> Self {
        value.pattern.clone()
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct StringLengthValidator {
    pub min_length: Option<i32>,
    pub max_length: Option<i32>,
}

impl serde::Serialize for StringLengthValidator {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&String::from(self))
    }
}

impl From<&StringLengthValidator> for String {
    fn from(value: &StringLengthValidator) -> Self {
        match (value.min_length, value.max_length) {
            (None, None) => Self::from(""),
            (Some(lower), Some(upper)) => format!("[{}, {}]", lower, upper),
            (None, Some(upper)) => format!("[, {}]", upper),
            (Some(lower), None) => format!("[{},]", lower),
        }
    }
}

impl From<Ranged<i32>> for StringLengthValidator {
    fn from(value: Ranged<i32>) -> Self {
        Self {
            max_length: value.end,
            min_length: value.start,
        }
    }
}
enum StringMetaProperty {
    Regex(StringRegexValidator),
    Default(String),
    Length(StringLengthValidator),
    Optional,
}

/// Parses a primitive StringProperty with its default meta properties.
/// If a meta property is defined twice, second one will overwrite the first.
/// Meta property parser will only run four times.
pub fn string_property<'a>(input: &'a str) -> CResult<&'a str, StringProperty> {
    let length = context(
        "StringLengthValidator",
        preceded(space1, string_length_validator),
    )
    .map(|x| StringMetaProperty::Length(x));
    let regex = preceded(space1, string_regex_validator).map(|x| StringMetaProperty::Regex(x));
    let default = preceded(space1, string_default_value).map(|x| StringMetaProperty::Default(x));
    let optional = preceded(space1, keywords::optional).map(|_| StringMetaProperty::Optional);

    let property_meta = context("PropertyMeta", alt((length, regex, default, optional)));

    context(
        "StringProperty",
        primitive_property(PrimitiveType::StringPropertyType)
            .and(fold_many_m_n(
                0,
                4,
                property_meta,
                Vec::new,
                |mut acc: Vec<_>, meta_prop| {
                    acc.push(meta_prop);
                    acc
                },
            ))
            .map(|((property_name, is_array), meta_props)| {
                let mut prop = StringProperty {
                    class: String::from("StringProperty"),
                    name: property_name.to_string(),
                    default_value: None,
                    regex_validator: None,
                    length_validator: None,
                    is_optional: false,
                    is_array,
                };

                for meta_prop in meta_props {
                    use StringMetaProperty::*;
                    match meta_prop {
                        Default(x) => prop.default_value = Some(x),
                        Regex(x) => prop.regex_validator = Some(x),
                        Length(x) => prop.length_validator = Some(x),
                        Optional => prop.is_optional = true,
                    }
                }

                prop
            }),
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
                    class: String::from("StringProperty"),
                    name: String::from("foo"),
                    default_value: None,
                    regex_validator: None,
                    length_validator: None,
                    is_optional: false,
                    is_array: false,
                }
            )),
            "Should parse string with no meta properties"
        );

        assert_eq!(
            super::string_property("o String foo optional"),
            Ok((
                "",
                super::StringProperty {
                    class: String::from("StringProperty"),
                    name: String::from("foo"),
                    default_value: None,
                    regex_validator: None,
                    length_validator: None,
                    is_optional: true,
                    is_array: false,
                }
            )),
            "Should parse string with optional flag"
        );

        assert_eq!(
            super::string_property("o String baz default=\"Hello World\""),
            Ok((
                "",
                super::StringProperty {
                    class: String::from("StringProperty"),
                    name: String::from("baz"),
                    default_value: Some(String::from("Hello World")),
                    regex_validator: None,
                    length_validator: None,
                    is_optional: false,
                    is_array: false,
                }
            )),
            "Should parse string with default value only"
        );

        assert_eq!(
            super::string_property("o String baz   regex = /abc.*/"),
            Ok((
                "",
                super::StringProperty {
                    class: String::from("StringProperty"),
                    name: String::from("baz"),
                    default_value: None,
                    regex_validator: Some(super::StringRegexValidator {
                        pattern: String::from("abc.*"),
                        flags: String::from("")
                    }),
                    length_validator: None,
                    is_optional: false,
                    is_array: false,
                }
            )),
            "Should parse string with regex value only"
        );

        assert_eq!(
            super::string_property("o String []   baz   regex = /abc.*/"),
            Ok((
                "",
                super::StringProperty {
                    class: String::from("StringProperty"),
                    name: String::from("baz"),
                    default_value: None,
                    regex_validator: Some(super::StringRegexValidator {
                        pattern: String::from("abc.*"),
                        flags: String::from("")
                    }),
                    length_validator: None,
                    is_optional: false,
                    is_array: true,
                }
            )),
            "Should parse string with array flag"
        );

        assert_eq!(
            super::string_property("o String baz    length   = [ 0 , 10  ]"),
            Ok((
                "",
                super::StringProperty {
                    class: String::from("StringProperty"),
                    name: String::from("baz"),
                    default_value: None,
                    regex_validator: None,
                    length_validator: Some(super::StringLengthValidator {
                        min_length: Some(0),
                        max_length: Some(10)
                    }),
                    is_optional: false,
                    is_array: false,
                }
            )),
            "Should parse string with length only"
        );

        assert_eq!(
            super::string_property(
                "o String baz regex  =\t/abc.*/ \tdefault  =   \"Hello World\"    length=[,100]"
            ),
            Ok((
                "",
                super::StringProperty {
                    class: String::from("StringProperty"),
                    name: String::from("baz"),
                    default_value: Some(String::from("Hello World")),
                    regex_validator: Some(super::StringRegexValidator {
                        pattern: String::from("abc.*"),
                        flags: String::from("")
                    }),
                    length_validator: Some(super::StringLengthValidator {
                        min_length: None,
                        max_length: Some(100)
                    }),
                    is_optional: false,
                    is_array: false,
                }
            )),
            "Should parse string with both default and regex and length"
        );

        assert_eq!(
            super::string_property(
                "o String baz regex  =\t/abc.*/ length=[,  100 ] \tdefault  =   \"Hello World\""
            ),
            Ok((
                "",
                super::StringProperty {
                    class: String::from("StringProperty"),
                    name: String::from("baz"),
                    default_value: Some(String::from("Hello World")),
                    regex_validator: Some(super::StringRegexValidator {
                        pattern: String::from("abc.*"),
                        flags: String::from("")
                    }),
                    length_validator: Some(super::StringLengthValidator {
                        min_length: None,
                        max_length: Some(100)
                    }),
                    is_optional: false,
                    is_array: false,
                }
            )),
            "Should parse string with both default and regex and length in a different order"
        );
    }

    #[test]
    fn test_serialize() {
        let a = super::StringProperty {
            class: String::from("StringProperty"),
            name: String::from("aProperty"),
            is_array: true,
            is_optional: false,
            default_value: Some("Hello world".into()),
            regex_validator: Some(super::StringRegexValidator {
                pattern: "abc.*".into(),
                flags: "".into(),
            }),
            length_validator: None,
        };

        assert_eq!(
            serde_json::json!({
              "$class": "StringProperty",
              "name": "aProperty",
              "isArray": true,
              "isOptional": false,
              "default": "Hello world",
              "regex": "abc.*"
            }),
            serde_json::to_value(a).unwrap(),
        )
    }
}
