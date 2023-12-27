use nom::{
    bytes::complete::tag,
    error::{context, ContextError, ParseError},
    IResult,
};

pub fn default<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, &'a str, E> {
    context("DefaultKeyword", tag("default"))(input)
}

pub fn namespace<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, &'a str, E> {
    context("NamespaceKeyword", tag("namespace"))(input)
}
