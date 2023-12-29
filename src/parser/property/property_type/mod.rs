mod primitive_type;

use nom::{
    branch::alt,
    error::{context, ContextError, ParseError},
    IResult, Parser,
};

pub(crate) use primitive_type::{primitive_type_parser, PrimitiveType};

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
