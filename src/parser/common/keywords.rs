use nom::{bytes::complete::tag, error::context};

use crate::parser::CResult;

pub fn default<'a>(input: &'a str) -> CResult<&'a str, &'a str> {
    context("DefaultKeyword", tag("default"))(input)
}

pub fn length<'a>(input: &'a str) -> CResult<&'a str, &'a str> {
    context("LengthKeyword", tag("length"))(input)
}

pub fn range<'a>(input: &'a str) -> CResult<&'a str, &'a str> {
    context("RangeKeyword", tag("range"))(input)
}

pub fn regex<'a>(input: &'a str) -> CResult<&'a str, &'a str> {
    context("RegexKeyword", tag("regex"))(input)
}

pub fn namespace<'a>(input: &'a str) -> CResult<&'a str, &'a str> {
    context("NamespaceKeyword", tag("namespace"))(input)
}

pub fn optional<'a>(input: &'a str) -> CResult<&'a str, &'a str> {
    context("OptionalKeyword", tag("optional"))(input)
}

pub fn concept<'a>(input: &'a str) -> CResult<&'a str, &'a str> {
    context("ConceptKeyword", tag("concept"))(input)
}

pub fn abstrakt<'a>(input: &'a str) -> CResult<&'a str, &'a str> {
    context("AbstractKeyword", tag("abstract"))(input)
}
