use nom::{
    bytes::complete::tag,
    error::{context, ContextError, ParseError},
    IResult,
};

pub fn string<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, &'a str, E> {
    context("StringToken", tag("String"))(input)
}

pub fn boolean<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, &'a str, E> {
    context("BooleanToken", tag("Boolean"))(input)
}

pub fn long<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, &'a str, E> {
    context("LongToken", tag("Long"))(input)
}

pub fn double<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, &'a str, E> {
    context("DoubleToken", tag("Double"))(input)
}

pub fn integer<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, &'a str, E> {
    context("IntegerToken", tag("Integer"))(input)
}

pub fn datetime<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, &'a str, E> {
    context("DateTimeToken", tag("DateTime"))(input)
}
