use nom::{
    branch::alt,
    character::complete::{char, space0},
    error::context,
    sequence::{delimited, preceded, separated_pair, terminated, tuple},
    Parser,
};

use super::CResult;

use crate::parser::{
    common::{double_value, integer_value, keywords, long_value, numeric::positive_integer_value},
    error::CError,
};

use super::MetaProperty;

struct Ranged<T> {
    start: Option<T>,
    end: Option<T>,
}

fn ranged_parser<
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

#[derive(Debug, PartialEq, Clone)]
pub enum NumberRange {
    Long(LongRange),
    Integer(IntegerRange),
    Double(DoubleRange),
}

#[derive(Debug, PartialEq, Clone)]
pub struct LengthLimits {
    pub start: Option<i32>,
    pub end: Option<i32>,
}

impl From<Ranged<i32>> for LengthLimits {
    fn from(value: Ranged<i32>) -> Self {
        Self {
            start: value.start,
            end: value.end,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct LongRange {
    pub start: Option<i64>,
    pub end: Option<i64>,
}

impl From<Ranged<i64>> for NumberRange {
    fn from(value: Ranged<i64>) -> Self {
        Self::Long(LongRange {
            start: value.start,
            end: value.end,
        })
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct IntegerRange {
    pub start: Option<i32>,
    pub end: Option<i32>,
}

impl From<Ranged<i32>> for NumberRange {
    fn from(value: Ranged<i32>) -> Self {
        Self::Integer(IntegerRange {
            start: value.start,
            end: value.end,
        })
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct DoubleRange {
    pub start: Option<f64>,
    pub end: Option<f64>,
}

impl From<Ranged<f64>> for NumberRange {
    fn from(value: Ranged<f64>) -> Self {
        Self::Double(DoubleRange {
            start: value.start,
            end: value.end,
        })
    }
}

pub(crate) fn length_meta_property<'a>(input: &'a str) -> CResult<&'a str, MetaProperty> {
    match ranged_parser(input, keywords::length, positive_integer_value) {
        Err(e) => Err(e),
        Ok((remains, ranged)) => Ok((remains, MetaProperty::Length(ranged.into()))),
    }
}

pub(crate) fn integer_range<'a>(input: &'a str) -> CResult<&'a str, MetaProperty> {
    match ranged_parser(input, keywords::range, integer_value) {
        Err(e) => Err(e),
        Ok((remains, ranged)) => Ok((remains, MetaProperty::Range(ranged.into()))),
    }
}

pub(crate) fn double_range<'a>(input: &'a str) -> CResult<&'a str, MetaProperty> {
    match ranged_parser(input, keywords::range, double_value) {
        Err(e) => Err(e),
        Ok((remains, ranged)) => Ok((remains, MetaProperty::Range(ranged.into()))),
    }
}

pub(crate) fn long_range<'a>(input: &'a str) -> CResult<&'a str, MetaProperty> {
    match ranged_parser(input, keywords::range, long_value) {
        Err(e) => Err(e),
        Ok((remains, ranged)) => Ok((remains, MetaProperty::Range(ranged.into()))),
    }
}

#[cfg(test)]
mod test {

    use super::LengthLimits;

    #[test]
    fn test_length_meta() {
        assert_eq!(
            super::length_meta_property("length = [ 100, ]"),
            Ok((
                "",
                super::MetaProperty::Length(LengthLimits {
                    start: Some(100),
                    end: None
                })
            )),
            "Should parse a length meta with start only"
        );

        assert_eq!(
            super::length_meta_property("length = [ , 100 ]"),
            Ok((
                "",
                super::MetaProperty::Length(LengthLimits {
                    end: Some(100),
                    start: None
                })
            )),
            "Should parse a length meta with end only"
        );

        assert_eq!(
            super::length_meta_property("length = [ 1, 100 ]"),
            Ok((
                "",
                super::MetaProperty::Length(LengthLimits {
                    start: Some(1),
                    end: Some(100),
                })
            )),
            "Should parse a length meta with full range"
        );

        assert!(
            super::length_meta_property("length = [ - 1, 100 ]").is_err(),
            "Should not parse a length meta negative numbers"
        );

        assert!(
            super::length_meta_property("length = [ , -100 ]").is_err(),
            "Should not parse a length meta negative numbers"
        );

        assert!(
            super::length_meta_property("length = [ ]").is_err(),
            "Should not parse a length meta with empty definition"
        );
    }
}
