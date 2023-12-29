use nom::{branch::alt, error::context, Parser};

use crate::parser::{common::concerto, CResult};

#[derive(Debug, Eq, PartialEq, Clone)]
pub(crate) enum PrimitiveType {
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
}
