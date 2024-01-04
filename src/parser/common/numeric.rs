use nom::{
    branch::alt,
    bytes::complete::tag_no_case,
    character::complete::{char, digit1, one_of},
    combinator::{map_res, opt, recognize},
    error::{context, ErrorKind, ParseError},
    sequence::{pair, preceded, tuple},
    Err as NomErr,
};
use std::str::FromStr;

use crate::parser::CResult;

/// Parse an optional sign followed by a number of digits.
pub(crate) fn positive_decimal_value<'a>(input: &'a str) -> CResult<&'a str, &'a str> {
    context(
        "PositiveDecimal",
        alt((
            context(
                "ExplicitlyPositiveDecimal",
                recognize(pair(char('+'), digit1)),
            ),
            context("PositiveDecimal", digit1),
        )),
    )(input)
}

/// Parse an optional sign followed by a number of digits.
pub(crate) fn negative_decimal_value<'a>(input: &'a str) -> CResult<&'a str, &'a str> {
    context("NegativeDecimal", recognize(pair(char('-'), digit1)))(input)
}

/// Parse an optional sign followed by a number of digits.
pub(crate) fn decimal_value<'a>(input: &'a str) -> CResult<&'a str, &'a str> {
    context(
        "Decimal",
        alt((negative_decimal_value, positive_decimal_value)),
    )(input)
}

fn floating_point_value<'a>(input: &'a str) -> CResult<&'a str, &'a str> {
    // In addition to the recipe in nom
    // https://doc.rust-lang.org/std/primitive.f64.html#impl-FromStr-for-f64
    context(
        "FloatingPointValue",
        alt((
            context(
                ".42e42",
                recognize(tuple((
                    char('.'),
                    digit1,
                    opt(tuple((one_of("eE"), decimal_value))),
                ))),
            ),
            context(
                "42e42 or 42.42e42",
                recognize(tuple((
                    decimal_value,
                    opt(preceded(char('.'), digit1)),
                    one_of("eE"),
                    decimal_value,
                ))),
            ),
            context(
                "42. or 42.42",
                recognize(tuple((decimal_value, char('.'), opt(digit1)))),
            ),
            context(
                "infinity",
                recognize(tuple((
                    opt(one_of("+-")),
                    alt((tag_no_case("infinity"), tag_no_case("inf"))),
                ))),
            ),
            // This works but not testable
            context("nan", recognize(tag_no_case("nan"))),
        )),
    )(input)
}

/// Parse a decimal guarantied to be positive, into i32
pub(crate) fn positive_integer_value<'a>(input: &'a str) -> CResult<&'a str, i32> {
    let maybe_i32 = map_res(positive_decimal_value, |s: &str| i32::from_str_radix(s, 10))(input);

    let res: CResult<&'a str, i32> = match maybe_i32 {
        Ok((rest, parsed)) => Ok((rest, parsed)),
        _ => Err(NomErr::Error(ParseError::from_error_kind(
            input,
            ErrorKind::Digit,
        ))),
    };
    res
}

// /// Parse a decimal guarantied to be negative, into i32
// pub(crate) fn negative_integer_value<'a>(input: &'a str) -> CResult<&'a str, i32> {
//     let maybe_i32 = map_res(negative_decimal_value, |s: &str| i32::from_str_radix(s, 10))(input);

//     let res: CResult<&'a str, i32> = match maybe_i32 {
//         Ok((rest, parsed)) => Ok((rest, parsed)),
//         _ => Err(NomErr::Error(ParseError::from_error_kind(
//             input,
//             ErrorKind::Digit,
//         ))),
//     };
//     res
// }

/// Parse a decimal into i32
pub(crate) fn integer_value<'a>(input: &'a str) -> CResult<&'a str, i32> {
    let maybe_i32 = map_res(decimal_value, |s: &str| i32::from_str_radix(s, 10))(input);

    let res: CResult<&'a str, i32> = match maybe_i32 {
        Ok((rest, parsed)) => Ok((rest, parsed)),
        _ => Err(NomErr::Error(ParseError::from_error_kind(
            input,
            ErrorKind::Digit,
        ))),
    };
    res
}

/// Parse a decimal into i64
pub(crate) fn long_value<'a>(input: &'a str) -> CResult<&'a str, i64> {
    let maybe_i64 = map_res(decimal_value, |s: &str| i64::from_str_radix(s, 10))(input);

    let res: CResult<&'a str, i64> = match maybe_i64 {
        Ok((rest, parsed)) => Ok((rest, parsed)),
        _ => Err(NomErr::Error(ParseError::from_error_kind(
            input,
            ErrorKind::Digit,
        ))),
    };
    res
}

/// Parse a floating point string into f64
pub(crate) fn double_value<'a>(input: &'a str) -> CResult<&'a str, f64> {
    let maybe_i64 = map_res(floating_point_value, |s: &str| f64::from_str(s))(input);

    let res: CResult<&'a str, f64> = match maybe_i64 {
        Ok((rest, parsed)) => Ok((rest, parsed)),
        _ => Err(NomErr::Error(ParseError::from_error_kind(
            input,
            ErrorKind::Digit,
        ))),
    };
    res
}

#[cfg(test)]
mod test {
    #[test]
    fn test_double_value() {
        assert_eq!(
            super::double_value(".42"),
            Ok(("", 0.42)),
            "Should parse .42"
        );
        assert_eq!(
            super::double_value(".42e43"),
            Ok(("", 0.42e43)),
            "Should parse .42e43"
        );
        assert_eq!(
            super::double_value(".42E43"),
            Ok(("", 0.42E43)),
            "Should parse .42E43"
        );
        assert_eq!(
            super::double_value(".42e-43"),
            Ok(("", 0.42e-43)),
            "Should parse .42e-43"
        );
        assert_eq!(
            super::double_value("42.42"),
            Ok(("", 42.42)),
            "Should parse 42.42"
        );
        assert_eq!(
            super::double_value("42.42e43"),
            Ok(("", 42.42e43)),
            "Should parse 42.42e43"
        );
        assert_eq!(
            super::double_value("42.42E43"),
            Ok(("", 42.42E43)),
            "Should parse 42.42E43"
        );
        assert_eq!(
            super::double_value("42.42e-43"),
            Ok(("", 42.42e-43)),
            "Should parse 42.42e-43"
        );
        assert_eq!(
            super::double_value("inf"),
            Ok(("", f64::INFINITY)),
            "Should parse inf"
        );
        assert_eq!(
            super::double_value("-infIniTY"),
            Ok(("", f64::NEG_INFINITY)),
            "Should parse inf"
        );
        assert_eq!(
            super::double_value("Infinity"),
            Ok(("", f64::INFINITY)),
            "Should parse Infinity"
        );
    }

    #[test]
    fn test_decimal() {
        assert_eq!(
            super::decimal_value("-345763874568374568"),
            Ok(("", "-345763874568374568")),
            "Should parse negative decimal"
        );
        assert_eq!(
            super::decimal_value("+345763874568374568"),
            Ok(("", "+345763874568374568")),
            "Should parse explicitly positive decimal"
        );
        assert_eq!(
            super::decimal_value("345763874568374568"),
            Ok(("", "345763874568374568")),
            "Should parse positive decimal"
        );
    }

    #[test]
    fn test_integer() {
        assert_eq!(
            super::integer_value("-147483647"),
            Ok(("", -147483647 as i32)),
            "Should parse negative integer"
        );
        assert_eq!(
            super::integer_value("147483647"),
            Ok(("", 147483647 as i32)),
            "Should parse positive integer"
        );
        assert_eq!(
            super::integer_value("+147483647"),
            Ok(("", 147483647 as i32)),
            "Should parse explicitly positive integer"
        );
        assert!(
            super::integer_value("-3147483647").is_err(),
            "Should not parse negative long"
        );
        assert!(
            super::integer_value("3147483647").is_err(),
            "Should not parse positive long"
        );
        assert!(
            super::integer_value("+3147483647").is_err(),
            "Should not parse explicitly positive long"
        );
    }

    #[test]
    fn test_long() {
        assert_eq!(
            super::long_value("-3147483647"),
            Ok(("", -3147483647 as i64)),
            "Should parse negative long"
        );
        assert_eq!(
            super::long_value("3147483647"),
            Ok(("", 3147483647 as i64)),
            "Should parse positive long"
        );
        assert_eq!(
            super::long_value("+3147483647"),
            Ok(("", 3147483647 as i64)),
            "Should parse explicitly positive long"
        );
    }
}
