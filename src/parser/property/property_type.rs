use nom::{
    branch::alt,
    bytes::complete::tag,
    error::{context, ContextError, ParseError},
    IResult, Parser,
};

#[derive(Debug, Eq, PartialEq, Clone)]
pub(super) enum PrimitiveType {
    StringPropertyType,
    BooleanPropertyType,
    LongPropertyType,
    DoublePropertyType,
    IntegerPropertyType,
    DateTimePropertyType,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub(super) enum PropertyType {
    Primitive(PrimitiveType),
}

fn primitive_type_parser<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, PrimitiveType, E> {
    let string_type_parser =
        tag::<&'a str, &'a str, E>("String").map(|_| PrimitiveType::StringPropertyType);
    let boolean_type_parser =
        tag::<&'a str, &'a str, E>("Boolean").map(|_| PrimitiveType::BooleanPropertyType);
    let long_type_parser =
        tag::<&'a str, &'a str, E>("Long").map(|_| PrimitiveType::LongPropertyType);
    let double_type_parser =
        tag::<&'a str, &'a str, E>("Double").map(|_| PrimitiveType::DoublePropertyType);
    let integer_type_parser =
        tag::<&'a str, &'a str, E>("Integer").map(|_| PrimitiveType::IntegerPropertyType);
    let datetime_type_parser =
        tag::<&'a str, &'a str, E>("DateTime").map(|_| PrimitiveType::DateTimePropertyType);

    context(
        "PrimitiveType",
        alt((
            string_type_parser,
            boolean_type_parser,
            long_type_parser,
            double_type_parser,
            integer_type_parser,
            datetime_type_parser,
        )),
    )(input)
}

pub(super) fn property_type_parser<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, PropertyType, E> {
    context(
        "PropertyType",
        alt((primitive_type_parser.map(|primitive| PropertyType::Primitive(primitive)),)),
    )(input)
}

#[cfg(test)]
mod test {

    #[test]
    fn test_property_type() {
        assert!(true);
    }
}
