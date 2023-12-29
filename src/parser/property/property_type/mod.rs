mod primitive_type;

use nom::{branch::alt, error::context, Parser};

use crate::parser::CResult;

pub(crate) use primitive_type::{primitive_type, PrimitiveType};

#[derive(Debug, Eq, PartialEq, Clone)]
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

#[cfg(test)]
mod test {

    #[test]
    fn test_property_type() {
        assert!(true);
    }
}
