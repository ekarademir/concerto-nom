use nom::{
    character::complete::{alpha1, alphanumeric0},
    combinator::recognize,
    error::{context, ContextError, ParseError},
    sequence::pair,
    IResult,
};

/// A token_parser starts with a letter and includes alphanumerical characters
pub fn token_parser<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, &'a str, E> {
    context("Token", recognize(pair(alpha1, alphanumeric0)))(input)
}

#[cfg(test)]
mod test {
    use nom::error::VerboseError;

    #[test]
    fn test_token() {
        assert_eq!(
            super::token_parser::<VerboseError<&str>>("a123"),
            Ok(("", "a123"))
        );
    }
}
