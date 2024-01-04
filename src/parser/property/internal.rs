use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, space0},
    error::context,
    sequence::{delimited, preceded, separated_pair, terminated, tuple},
    Parser,
};

use crate::parser::{common::token, error::CError, CResult};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum PrimitiveType {
    StringPropertyType,
    BooleanPropertyType,
    LongPropertyType,
    DoublePropertyType,
    IntegerPropertyType,
    DateTimePropertyType,
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
            _ => unreachable!(),
        }
    }
}

impl<'a> From<PrimitiveType> for &'a str {
    fn from(value: PrimitiveType) -> Self {
        use PrimitiveType::*;
        match value {
            StringPropertyType => "String",
            BooleanPropertyType => "Boolean",
            LongPropertyType => "Long",
            DoublePropertyType => "Double",
            IntegerPropertyType => "Integer",
            DateTimePropertyType => "DateTime",
        }
    }
}

enum AnnotatedType {
    Single,
    Array,
}

/// Parses provided primitive type then returns (the name of the defined type, is array) tuple
pub fn primitive_property<'a>(
    primitive_type: PrimitiveType,
) -> impl Fn(&'a str) -> CResult<&'a str, (&'a str, bool)> {
    move |input: &'a str| {
        let type_tag: &'a str = primitive_type.into();
        let single_type = tuple((space0, char('o'), space0, tag(type_tag), space0))
            .map(|_| AnnotatedType::Single);
        let array_type = tuple((
            space0,
            char('o'),
            space0,
            tag(type_tag),
            space0,
            char('['),
            space0,
            char(']'),
            space0,
        ))
        .map(|_| AnnotatedType::Array);

        context(
            "PrimitiveProperty",
            tuple((alt((array_type, single_type)), token)).map(
                |(annotated, name)| match annotated {
                    AnnotatedType::Array => (name, true),
                    AnnotatedType::Single => (name, false),
                },
            ),
        )(input)
    }
}

pub(crate) struct Ranged<T> {
    pub(crate) start: Option<T>,
    pub(crate) end: Option<T>,
}

pub(crate) fn ranged_parser<
    'a,
    T,
    P: Parser<&'a str, T, CError<&'a str>> + Copy,
    KV: Parser<&'a str, &'a str, CError<&'a str>>,
>(
    input: &'a str,
    keyword: KV,
    parser: P,
) -> CResult<&'a str, Ranged<T>> {
    let only_start = context(
        "RangedOnlyStart",
        terminated(parser, tuple((space0, char(','), space0))),
    )
    .map(|start: T| Ranged {
        start: Some(start),
        end: None,
    });

    let only_end = context(
        "RangedOnlyEnd",
        preceded(tuple((space0, char(','), space0)), parser),
    )
    .map(|end: T| Ranged {
        end: Some(end),
        start: None,
    });

    let full = context(
        "RangedFull",
        separated_pair(parser, tuple((space0, char(','), space0)), parser),
    )
    .map(|(start, end): (T, T)| Ranged {
        start: Some(start),
        end: Some(end),
    });

    context(
        "RangedMetaProperty",
        delimited(
            tuple((keyword, space0, char('='), space0, char('['), space0)),
            alt((full, only_start, only_end)),
            tuple((space0, char(']'))),
        ),
    )(input)
}
