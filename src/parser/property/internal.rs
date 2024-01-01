use nom::{
    bytes::complete::tag,
    character::complete::{char, space0},
    error::context,
    sequence::{preceded, tuple},
};

pub use crate::parser::property::property_type::PrimitiveType;
use crate::parser::{common::token, CResult};

/// Parses provided primitive type then returns the name of the defined type
pub fn primitive_property<'a>(
    primitive_type: PrimitiveType,
) -> impl Fn(&'a str) -> CResult<&'a str, &'a str> {
    move |input: &'a str| {
        let type_tag: &'a str = primitive_type.into();
        context(
            "PrimitiveProperty",
            preceded(
                tuple((space0, char('o'), space0, tag(type_tag), space0)),
                token,
            ),
        )(input)
    }
}
