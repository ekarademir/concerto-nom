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
    common::{datetime::datetime_value, keywords},
    property::internal::{primitive_property, PrimitiveType},
    CResult,
};

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct DateTimeProperty {
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
}

enum DateTimeMetaProperty {
    Default(String),
    Optional,
}

pub fn datetime_property<'a>(input: &'a str) -> CResult<&'a str, DateTimeProperty> {
    let default =
        preceded(space1, datetime_default_value).map(|x| DateTimeMetaProperty::Default(x));
    let optional = preceded(space1, keywords::optional).map(|_| DateTimeMetaProperty::Optional);

    let property_meta = context("PropertyMeta", alt((default, optional)));

    context(
        "DateTimeProperty",
        primitive_property(PrimitiveType::DateTimePropertyType)
            .and(fold_many_m_n(
                0,
                2,
                property_meta,
                Vec::new,
                |mut acc: Vec<_>, meta_prop| {
                    acc.push(meta_prop);
                    acc
                },
            ))
            .map(|((property_name, is_array), meta_props)| {
                let mut prop = DateTimeProperty {
                    class: String::from("DateTimeProperty"),
                    name: property_name.to_string(),
                    default_value: None,
                    is_optional: false,
                    is_array,
                };

                for meta_prop in meta_props {
                    use DateTimeMetaProperty::*;
                    match meta_prop {
                        Default(x) => prop.default_value = Some(x),
                        Optional => prop.is_optional = true,
                    }
                }

                prop
            }),
    )(input)
}

pub fn datetime_default_value<'a>(input: &'a str) -> CResult<&'a str, String> {
    into(context(
        "DateTimeDefaultValue",
        preceded(
            tuple((keywords::default, space0, char('='), space0)),
            datetime_value,
        ),
    ))(input)
}

#[cfg(test)]
mod test {
    #[test]
    fn test_datetime_property() {
        assert_eq!(
            super::datetime_property("o DateTime foo"),
            Ok((
                "",
                super::DateTimeProperty {
                    name: String::from("foo"),
                    class: String::from("DateTimeProperty"),
                    default_value: None,
                    is_optional: false,
                    is_array: false,
                }
            )),
            "Should parse datetime with no meta properties"
        );

        assert_eq!(
            super::datetime_property("o DateTime baz default=2024-01-04T18:39:55+02:30"),
            Ok((
                "",
                super::DateTimeProperty {
                    name: String::from("baz"),
                    class: String::from("DateTimeProperty"),
                    default_value: Some(String::from("2024-01-04T18:39:55+02:30")),
                    is_optional: false,
                    is_array: false,
                }
            )),
            "Should parse datetime with default value"
        );

        assert_eq!(
            super::datetime_property("o DateTime baz default=2024-01-04T18:39:55+02:30 optional"),
            Ok((
                "",
                super::DateTimeProperty {
                    name: String::from("baz"),
                    class: String::from("DateTimeProperty"),
                    default_value: Some(String::from("2024-01-04T18:39:55+02:30")),
                    is_optional: true,
                    is_array: false,
                }
            )),
            "Should parse datetime with optional flag"
        );

        assert_eq!(
            super::datetime_property("o DateTime[] baz default=2024-01-04T18:39:55+02:30 optional"),
            Ok((
                "",
                super::DateTimeProperty {
                    name: String::from("baz"),
                    class: String::from("DateTimeProperty"),
                    default_value: Some(String::from("2024-01-04T18:39:55+02:30")),
                    is_optional: true,
                    is_array: true,
                }
            )),
            "Should parse datetime with array flag"
        );

        assert_eq!(
            super::datetime_property("o DateTime baz default=42"),
            Ok((
                " default=42",
                super::DateTimeProperty {
                    name: String::from("baz"),
                    class: String::from("DateTimeProperty"),
                    default_value: None,
                    is_optional: false,
                    is_array: false,
                }
            )),
            "Should not parse datetime with wring default value"
        );
    }
}
