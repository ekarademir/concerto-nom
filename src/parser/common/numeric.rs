use nom::{
    branch::alt,
    character::complete::{char, digit1},
    combinator::{map_res, recognize},
    error::{context, ContextError, Error, ErrorKind, ParseError},
    sequence::pair,
    Err as NomErr, IResult,
};

/// Parse an optional sign followed by a number of digits.
pub(crate) fn decimal_parser<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, &'a str, E> {
    context(
        "Decimal",
        alt((
            context("NegativeDecimal", recognize(pair(char('-'), digit1))),
            context(
                "ExplicitlyPositiveDecimal",
                recognize(pair(char('+'), digit1)),
            ),
            context("PositiveDecimal", digit1),
        )),
    )(input)
}

/// Parse a decimal into i32
pub(crate) fn integer_parser<'a, E: ParseError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, i32, E> {
    let maybe_i32 = map_res(decimal_parser::<Error<&'a str>>, |s: &str| {
        i32::from_str_radix(s, 10)
    })(input);

    let res: IResult<&'a str, i32, E> = match maybe_i32 {
        Ok((rest, parsed)) => Ok((rest, parsed)),
        _ => Err(NomErr::Error(E::from_error_kind(input, ErrorKind::Digit))),
    };
    res
}

/// Parse a decimal into i64
pub(crate) fn long_parser<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, i64, E> {
    let maybe_i64 = map_res(decimal_parser::<Error<&'a str>>, |s: &str| {
        i64::from_str_radix(s, 10)
    })(input);

    let res: IResult<&'a str, i64, E> = match maybe_i64 {
        Ok((rest, parsed)) => Ok((rest, parsed)),
        _ => Err(NomErr::Error(E::from_error_kind(input, ErrorKind::Digit))),
    };
    res
}

#[cfg(test)]
mod test {
    use nom::error::VerboseError;
    #[test]
    fn test_decimal() {
        assert_eq!(
            super::decimal_parser::<VerboseError<&str>>("-345763874568374568"),
            Ok(("", "-345763874568374568")),
            "Should parse negative decimal"
        );
        assert_eq!(
            super::decimal_parser::<VerboseError<&str>>("+345763874568374568"),
            Ok(("", "+345763874568374568")),
            "Should parse explicitly positive decimal"
        );
        assert_eq!(
            super::decimal_parser::<VerboseError<&str>>("345763874568374568"),
            Ok(("", "345763874568374568")),
            "Should parse positive decimal"
        );
    }

    #[test]
    fn test_integer() {
        assert_eq!(
            super::integer_parser::<VerboseError<&str>>("-147483647"),
            Ok(("", -147483647 as i32)),
            "Should parse negative integer"
        );
        assert_eq!(
            super::integer_parser::<VerboseError<&str>>("147483647"),
            Ok(("", 147483647 as i32)),
            "Should parse positive integer"
        );
        assert_eq!(
            super::integer_parser::<VerboseError<&str>>("+147483647"),
            Ok(("", 147483647 as i32)),
            "Should parse explicitly positive integer"
        );
        assert!(
            super::integer_parser::<VerboseError<&str>>("-3147483647").is_err(),
            "Should not parse negative long"
        );
        assert!(
            super::integer_parser::<VerboseError<&str>>("3147483647").is_err(),
            "Should not parse positive long"
        );
        assert!(
            super::integer_parser::<VerboseError<&str>>("+3147483647").is_err(),
            "Should not parse explicitly positive long"
        );
    }

    #[test]
    fn test_long() {
        assert_eq!(
            super::long_parser::<VerboseError<&str>>("-3147483647"),
            Ok(("", -3147483647 as i64)),
            "Should parse negative long"
        );
        assert_eq!(
            super::long_parser::<VerboseError<&str>>("3147483647"),
            Ok(("", 3147483647 as i64)),
            "Should parse positive long"
        );
        assert_eq!(
            super::long_parser::<VerboseError<&str>>("+3147483647"),
            Ok(("", 3147483647 as i64)),
            "Should parse explicitly positive long"
        );
    }
}
