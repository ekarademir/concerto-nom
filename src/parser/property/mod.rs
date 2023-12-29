mod default_value;
mod property_type;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{space0, space1},
    error::context,
    multi::fold_many0,
    sequence::{pair, preceded, separated_pair},
    Parser,
};

use super::common::token_parser;
use super::CResult;
use default_value::DefaultValue;
use property_type::{property_type_parser, PropertyType};

#[derive(Debug, PartialEq, Clone)]
pub enum MetaProperty {
    Default(DefaultValue),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Property {
    name: String,
    type_name: PropertyType,
    meta_properties: Vec<MetaProperty>,
}

fn meta_property_parser<'a>(input: &'a str) -> CResult<&'a str, MetaProperty> {
    context(
        "MetaProperty",
        alt((default_value::default_metaproperty_parser.map(|d| MetaProperty::Default(d)),)),
    )(input)
}

fn property_nometa_parser<'a>(input: &'a str) -> CResult<&'a str, (PropertyType, String)> {
    context(
        "PropertyNoMeta",
        separated_pair(property_type_parser, space1, token_parser)
            .map(|(property_type, property_name)| (property_type, property_name.to_string())),
    )(input)
}

fn property_meta_parser<'a>(input: &'a str) -> CResult<&'a str, Vec<MetaProperty>> {
    context(
        "PropertyhMeta",
        fold_many0(
            meta_property_parser,
            Vec::new,
            |mut acc: Vec<_>, property| {
                acc.push(property);
                acc
            },
        ),
    )(input)
}

pub fn property_parser<'a>(input: &'a str) -> CResult<&'a str, Property> {
    let no_meta_parser = context("NoMeta", space0.map(|_| Vec::new()));
    let body_parser = context(
        "PropertyBody",
        alt((
            separated_pair(property_nometa_parser, space1, property_meta_parser),
            separated_pair(property_nometa_parser, space0, no_meta_parser),
        ))
        .map(|((type_name, name), meta_properties)| Property {
            name,
            type_name,
            meta_properties,
        }),
    );

    context("Property", preceded(pair(tag("o"), space0), body_parser))(input)
}

#[cfg(test)]
mod test {
    #[test]
    fn test_property_meta() {
        assert_eq!(
            super::property_meta_parser("default=\"Hello World\""),
            Ok((
                "",
                vec![super::MetaProperty::Default(
                    super::DefaultValue::StringDefaultValue("Hello World".to_string())
                )]
            )),
        );
    }

    #[test]
    fn test_property_parser() {
        assert_eq!(
            super::property_parser("o String foo"),
            Ok((
                "",
                super::Property {
                    name: String::from("foo"),
                    type_name: super::PropertyType::Primitive(
                        super::property_type::PrimitiveType::StringPropertyType
                    ),
                    meta_properties: Vec::new(),
                }
            ))
        );
        assert_eq!(
            super::property_parser("o Boolean bar123"),
            Ok((
                "",
                super::Property {
                    name: String::from("bar123"),
                    type_name: super::PropertyType::Primitive(
                        super::property_type::PrimitiveType::BooleanPropertyType
                    ),
                    meta_properties: Vec::new(),
                }
            ))
        );

        assert_eq!(
            super::property_parser("o String baz default=\"Hello World\""),
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
            ))
        );
    }
}
