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
    common::{boolean_value, keywords},
    property::internal::{primitive_property, PrimitiveType},
    CResult,
};

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct BooleanProperty {
    #[serde(rename = "$class")]
    pub class: String,
    pub name: String,
    #[serde(rename = "isOptional")]
    pub is_optional: bool,
    #[serde(rename = "isArray")]
    pub is_array: bool,
    #[serde(rename = "default")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_value: Option<bool>,
}

enum BooleanMetaProperty {
    Default(bool),
    Optional,
}

pub fn boolean_property<'a>(input: &'a str) -> CResult<&'a str, BooleanProperty> {
    let default = preceded(space1, boolean_default_value).map(|x| BooleanMetaProperty::Default(x));
    let optional = preceded(space1, keywords::optional).map(|_| BooleanMetaProperty::Optional);

    let property_meta = context("PropertyMeta", alt((default, optional)));

    context(
        "BooleanProperty",
        primitive_property(PrimitiveType::BooleanPropertyType)
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
                let mut prop = BooleanProperty {
                    class: String::from("BooleanProperty"),
                    name: property_name.to_string(),
                    default_value: None,
                    is_optional: false,
                    is_array,
                };

                for meta_prop in meta_props {
                    use BooleanMetaProperty::*;
                    match meta_prop {
                        Default(x) => prop.default_value = Some(x),
                        Optional => prop.is_optional = true,
                    }
                }

                prop
            }),
    )(input)
}

pub fn boolean_default_value<'a>(input: &'a str) -> CResult<&'a str, bool> {
    into(context(
        "BooleanDefaultValue",
        preceded(
            tuple((keywords::default, space0, char('='), space0)),
            boolean_value,
        ),
    ))(input)
}

#[cfg(test)]
mod test {

    #[test]
    fn test_serialize_without_default() {
        let a = super::BooleanProperty {
            class: String::from("BooleanProperty"),
            name: String::from("aProperty"),
            is_array: false,
            is_optional: true,
            default_value: None,
        };

        assert_eq!(
            serde_json::json!({
              "$class": "BooleanProperty",
              "name": "aProperty",
              "isArray": false,
              "isOptional": true,
            }),
            serde_json::to_value(a).unwrap(),
        )
    }

    #[test]
    fn test_serialize_with_default() {
        let a = super::BooleanProperty {
            class: String::from("BooleanProperty"),
            name: String::from("aProperty"),
            is_array: false,
            is_optional: true,
            default_value: Some(false),
        };

        assert_eq!(
            serde_json::json!({
              "$class": "BooleanProperty",
              "name": "aProperty",
              "isArray": false,
              "isOptional": true,
              "default": false,
            }),
            serde_json::to_value(a).unwrap(),
        )
    }

    #[test]
    fn test_boolean_property() {
        assert_eq!(
            super::boolean_property("o Boolean foo"),
            Ok((
                "",
                super::BooleanProperty {
                    name: String::from("foo"),
                    class: String::from("BooleanProperty"),
                    default_value: None,
                    is_optional: false,
                    is_array: false,
                }
            )),
            "Should parse boolean with no meta properties"
        );

        assert_eq!(
            super::boolean_property("o Boolean[] foo"),
            Ok((
                "",
                super::BooleanProperty {
                    name: String::from("foo"),
                    class: String::from("BooleanProperty"),
                    default_value: None,
                    is_optional: false,
                    is_array: true,
                }
            )),
            "Should parse boolean with array flag"
        );

        assert_eq!(
            super::boolean_property("o Boolean baz default=false"),
            Ok((
                "",
                super::BooleanProperty {
                    name: String::from("baz"),
                    class: String::from("BooleanProperty"),
                    default_value: Some(false),
                    is_optional: false,
                    is_array: false,
                }
            )),
            "Should parse boolean with false default value"
        );

        assert_eq!(
            super::boolean_property("o Boolean baz default=true"),
            Ok((
                "",
                super::BooleanProperty {
                    name: String::from("baz"),
                    class: String::from("BooleanProperty"),
                    default_value: Some(true),
                    is_optional: false,
                    is_array: false,
                }
            )),
            "Should parse boolean with true default value"
        );

        assert_eq!(
            super::boolean_property("o Boolean baz optional default=true"),
            Ok((
                "",
                super::BooleanProperty {
                    name: String::from("baz"),
                    class: String::from("BooleanProperty"),
                    default_value: Some(true),
                    is_optional: true,
                    is_array: false,
                }
            )),
            "Should parse boolean with optional flag"
        );

        assert_eq!(
            super::boolean_property("o Boolean baz default=42"),
            Ok((
                " default=42",
                super::BooleanProperty {
                    name: String::from("baz"),
                    class: String::from("BooleanProperty"),
                    default_value: None,
                    is_optional: false,
                    is_array: false,
                }
            )),
            "Should not parse boolean with wrong default value"
        );
    }
}
