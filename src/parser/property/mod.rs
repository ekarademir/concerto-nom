mod default_value;
mod internal;
mod meta_property;
mod property_type;

// After refactor
pub mod boolean_property;
pub mod string_property;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, space0, space1},
    combinator::{into, recognize},
    error::context,
    multi::fold_many0,
    sequence::{delimited, pair, preceded, separated_pair, tuple},
    Parser,
};

use super::common::token;
use super::CResult;
use property_type::{property_type, PrimitiveType, PropertyType};

pub use default_value::DefaultValue;
use meta_property::{
    meta_property, ranged::length_meta_property, regex_meta_property, string_meta_property,
    MetaProperty,
};

#[derive(Debug, PartialEq, Clone)]
pub struct Property {
    name: String,
    type_name: PropertyType,
    meta_properties: Vec<MetaProperty>,
}

fn property_nometa<'a>(input: &'a str) -> CResult<&'a str, (PropertyType, String)> {
    context(
        "PropertyNoMeta",
        separated_pair(property_type, space1, token)
            .map(|(property_type, property_name)| (property_type, property_name.to_string())),
    )(input)
}

fn property_meta<'a>(input: &'a str) -> CResult<&'a str, Vec<MetaProperty>> {
    context(
        "PropertyhMeta",
        fold_many0(
            delimited(space0, meta_property, space0),
            Vec::new,
            |mut acc: Vec<_>, property| {
                acc.push(property);
                acc
            },
        ),
    )(input)
}

pub fn no_meta_parser<'a>(input: &'a str) -> CResult<&'a str, Vec<MetaProperty>> {
    context("NoMeta", space0.map(|_| Vec::new()))(input)
}

pub fn string_primitive_property<'a>(input: &'a str) -> CResult<&'a str, Property> {
    let string_property_meta = context(
        "StringPropertyhMeta",
        fold_many0(
            alt((
                delimited(space0, string_meta_property, space0),
                delimited(space0, regex_meta_property, space0),
                delimited(space0, length_meta_property, space0),
            )),
            Vec::new,
            |mut acc: Vec<_>, property| {
                acc.push(property);
                acc
            },
        ),
    );

    context(
        "StringPrimitiveProperty",
        separated_pair(property_nometa, space1, string_property_meta).map(
            |((type_name, name), meta_properties)| Property {
                name,
                type_name,
                meta_properties,
            },
        ),
    )(input)
}

pub fn generic_property<'a>(input: &'a str) -> CResult<&'a str, Property> {
    context(
        "PropertyBody",
        alt((
            separated_pair(property_nometa, space1, property_meta),
            separated_pair(property_nometa, space0, no_meta_parser),
        ))
        .map(|((type_name, name), meta_properties)| Property {
            name,
            type_name,
            meta_properties,
        }),
    )(input)
}

// Remove
pub fn property_definition<'a>(input: &'a str) -> CResult<&'a str, Property> {
    let body_parser = context(
        "PropertyBody",
        alt((string_primitive_property, generic_property)),
    );

    context("Property", preceded(pair(tag("o"), space0), body_parser))(input)
}

#[cfg(test)]
mod test {
    use crate::parser::property::meta_property::ranged::LengthLimits;
    #[test]
    fn test_string_property() {
        assert_eq!(
            super::property_definition("o String foo"),
            Ok((
                "",
                super::Property {
                    name: String::from("foo"),
                    type_name: super::PropertyType::Primitive(
                        super::property_type::PrimitiveType::StringPropertyType
                    ),
                    meta_properties: Vec::new(),
                }
            )),
            "Should parse string with no meta properties"
        );

        assert_eq!(
            super::property_definition("o String baz default=\"Hello World\""),
            Ok((
                "",
                super::Property {
                    name: String::from("baz"),
                    type_name: super::PropertyType::Primitive(
                        super::property_type::PrimitiveType::StringPropertyType
                    ),
                    meta_properties: vec![super::MetaProperty::Default(
                        super::DefaultValue::StringDefaultValue("Hello World".to_string())
                    )],
                }
            )),
            "Should parse string with default value only"
        );

        assert_eq!(
            super::property_definition("o String baz regex=/abc.*/"),
            Ok((
                "",
                super::Property {
                    name: String::from("baz"),
                    type_name: super::PropertyType::Primitive(
                        super::property_type::PrimitiveType::StringPropertyType
                    ),
                    meta_properties: vec![super::MetaProperty::Regex("abc.*".to_string())],
                }
            )),
            "Should parse string with regex value only"
        );

        assert_eq!(
            super::property_definition("o String baz length=[0, 10]"),
            Ok((
                "",
                super::Property {
                    name: String::from("baz"),
                    type_name: super::PropertyType::Primitive(
                        super::property_type::PrimitiveType::StringPropertyType
                    ),
                    meta_properties: vec![super::MetaProperty::Length(LengthLimits {
                        start: Some(0),
                        end: Some(10),
                    })],
                }
            )),
            "Should parse string with length only"
        );

        assert_eq!(
            super::property_definition(
                "o String baz length=[,100] regex=/abc.*/ default=\"Hello World\""
            ),
            Ok((
                "",
                super::Property {
                    name: String::from("baz"),
                    type_name: super::PropertyType::Primitive(
                        super::property_type::PrimitiveType::StringPropertyType
                    ),
                    meta_properties: vec![
                        super::MetaProperty::Length(LengthLimits {
                            start: None,
                            end: Some(100)
                        }),
                        super::MetaProperty::Regex("abc.*".to_string()),
                        super::MetaProperty::Default(super::DefaultValue::StringDefaultValue(
                            "Hello World".to_string()
                        )),
                    ],
                }
            )),
            "Should parse string with both default and regex and length"
        );
    }

    #[test]
    fn test_property_parser() {
        assert_eq!(
            super::property_definition("o Boolean bar123"),
            Ok((
                "",
                super::Property {
                    name: String::from("bar123"),
                    type_name: super::PropertyType::Primitive(
                        super::property_type::PrimitiveType::BooleanPropertyType
                    ),
                    meta_properties: Vec::new(),
                }
            )),
            "Should parse boolean with no meta properties"
        );
    }
}
