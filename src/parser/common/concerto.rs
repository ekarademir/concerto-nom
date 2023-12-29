use nom::{bytes::complete::tag, error::context};

use crate::parser::CResult;

pub fn string<'a>(input: &'a str) -> CResult<&'a str, &'a str> {
    context("StringToken", tag("String"))(input)
}

pub fn boolean<'a>(input: &'a str) -> CResult<&'a str, &'a str> {
    context("BooleanToken", tag("Boolean"))(input)
}

pub fn long<'a>(input: &'a str) -> CResult<&'a str, &'a str> {
    context("LongToken", tag("Long"))(input)
}

pub fn double<'a>(input: &'a str) -> CResult<&'a str, &'a str> {
    context("DoubleToken", tag("Double"))(input)
}

pub fn integer<'a>(input: &'a str) -> CResult<&'a str, &'a str> {
    context("IntegerToken", tag("Integer"))(input)
}

pub fn datetime<'a>(input: &'a str) -> CResult<&'a str, &'a str> {
    context("DateTimeToken", tag("DateTime"))(input)
}
