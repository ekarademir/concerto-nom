use nom::{branch::alt, error::context, Parser};

use crate::parser::{common::concerto, CResult};

#[derive(Debug, PartialEq, Clone)]
pub(super) enum PropertyType {
    Primitive(PrimitiveType),
}

impl From<PrimitiveType> for PropertyType {
    fn from(value: PrimitiveType) -> Self {
        Self::Primitive(value)
    }
}

pub(super) fn property_type<'a>(input: &'a str) -> CResult<&'a str, PropertyType> {
    Parser::into(context("PropertyType", alt((primitive_type,)))).parse(input)
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum PrimitiveType {
    StringPropertyType,
    BooleanPropertyType,
    LongPropertyType,
    DoublePropertyType,
    IntegerPropertyType,
    DateTimePropertyType,
    None,
}

impl<'a> From<&'a str> for PrimitiveType {
    fn from(value: &'a str) -> Self {
        match value {
            "String" => Self::StringPropertyType,
            "Boolean" => Self::BooleanPropertyType,
            "Long" => Self::LongPropertyType,
            "Double" => Self::DoublePropertyType,
            "Integer" => Self::IntegerPropertyType,
            "DateTime" => Self::DateTimePropertyType,
            _ => Self::None,
        }
    }
}

impl<'a> From<PrimitiveType> for &'a str {
    fn from(value: PrimitiveType) -> Self {
        match value {
            PrimitiveType::StringPropertyType => "String",
            PrimitiveType::BooleanPropertyType => "Boolean",
            PrimitiveType::LongPropertyType => "Long",
            PrimitiveType::DoublePropertyType => "Double",
            PrimitiveType::IntegerPropertyType => "Integer",
            PrimitiveType::DateTimePropertyType => "DateTime",
            _ => "",
        }
    }
}

pub(crate) fn primitive_type<'a>(input: &'a str) -> CResult<&'a str, PrimitiveType> {
    Parser::into(context(
        "PrimitiveType",
        alt((
            concerto::string,
            concerto::boolean,
            concerto::long,
            concerto::double,
            concerto::integer,
            concerto::datetime,
        )),
    ))
    .parse(input)
}

#[cfg(test)]
mod test {

    #[test]
    fn test_primitive_type() {
        assert!(true);
    }

    #[test]
    fn test_property_type() {
        assert!(true);
    }
}
