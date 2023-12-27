use nom::{
    branch::alt,
    error::{context, ContextError, ParseError},
    IResult, Parser,
};

use crate::parser::common::concerto;

#[derive(Debug, Eq, PartialEq, Clone)]
pub(super) enum PrimitiveType {
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

fn primitive_type_parser<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, PrimitiveType, E> {
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

#[derive(Debug, Eq, PartialEq, Clone)]
pub(super) enum PropertyType {
    Primitive(PrimitiveType),
}

impl From<PrimitiveType> for PropertyType {
    fn from(value: PrimitiveType) -> Self {
        Self::Primitive(value)
    }
}

pub(super) fn property_type_parser<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, PropertyType, E> {
    Parser::into(context("PropertyType", alt((primitive_type_parser,)))).parse(input)
}

#[cfg(test)]
mod test {

    #[test]
    fn test_property_type() {
        assert!(true);
    }
}
