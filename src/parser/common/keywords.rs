use nom::{bytes::complete::tag, error::context};

use crate::parser::CResult;

pub fn default<'a>(input: &'a str) -> CResult<&'a str, &'a str> {
    context("DefaultKeyword", tag("default"))(input)
}

pub fn regex<'a>(input: &'a str) -> CResult<&'a str, &'a str> {
    context("RegexKeyword", tag("regex"))(input)
}

pub fn namespace<'a>(input: &'a str) -> CResult<&'a str, &'a str> {
    context("NamespaceKeyword", tag("namespace"))(input)
}
