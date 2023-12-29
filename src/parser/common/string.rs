/// This parser is pretty much the example from nom
/// https://github.com/rust-bakery/nom/blob/7.1.3/examples/string.rs
/// Differences are:
/// - Doesn't propagate `FromExternalError` when parsing integers that form a unicode character,
///   instead, it converts it to a `ParserError`.
/// - In addition to double quoted strings, it also accepts and escapes single quotes strings.
use nom::{
    branch::alt,
    bytes::streaming::{is_not, take_while_m_n},
    character::complete::{char, multispace1},
    combinator::{map, map_opt, map_res, value, verify},
    error::{context, ErrorKind, ParseError},
    multi::fold_many0,
    sequence::{delimited, preceded},
    Err as NomErr,
};

use crate::parser::CResult;

/// Collects hex digits within u{XXXX}
fn delimited_hex<'a>(input: &'a str) -> CResult<&'a str, &'a str> {
    // Collect all hex digits
    let hex = take_while_m_n(1, 6, |c: char| c.is_ascii_hexdigit());

    // Collect hex digits within u{XXXX}
    preceded(char('u'), delimited(char('{'), hex, char('}')))(input)
}

/// Converts hex digits to integers, different from the example, it emits a `ParseError`
/// when en external error is encountered`, instead of propagating `FromExternalError`.
fn u32_value<'a>(input: &'a str) -> CResult<&'a str, u32> {
    let maybe_u32 = map_res(delimited_hex, move |h| u32::from_str_radix(h, 16))(input);

    let res: CResult<&'a str, u32> = match maybe_u32 {
        Ok((rest, parsed)) => Ok((rest, parsed)),
        _ => Err(NomErr::Error(ParseError::from_error_kind(
            input,
            ErrorKind::Digit,
        ))),
    };
    res
}

/// Parses characters that start wuth `u` and followed by 3 to 6 integers
fn unicode_char<'a>(input: &'a str) -> CResult<&'a str, char> {
    // Convert them back to character, validating unicode character
    let u32_validate = context(
        "U32Validate",
        map_opt(u32_value, |val| std::char::from_u32(val)),
    );

    context("UnicodeCharacter", u32_validate)(input)
}

/// Parses escaped characters
fn escaped_char<'a>(input: &'a str) -> CResult<&'a str, char> {
    context(
        "EscapedCharacter",
        preceded(
            char('\\'),
            alt((
                unicode_char,
                value('\n', char('n')),
                value('\r', char('r')),
                value('\t', char('t')),
                value('\u{08}', char('b')), // Unicode backspace
                value('\u{0C}', char('f')), // Unicode form feed
                value('\\', char('\\')),
                value('/', char('/')),
                value('"', char('"')),
                value('\'', char('\'')),
            )),
        ),
    )(input)
}

/// Parse escaped whitespace, trusting the wisdom of the example
fn escaped_whitespace<'a>(input: &'a str) -> CResult<&'a str, &'a str> {
    context("EscapedWhitespace", preceded(char('\\'), multispace1))(input)
}

/// Parse non-escaped characters
fn literal<'a>(input: &'a str) -> CResult<&'a str, &'a str> {
    let not_quote_slash = context("NotQuoteSlash", is_not("\'\"\\"));
    context("Literal", verify(not_quote_slash, |s: &str| !s.is_empty()))(input)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum StringFragment<'a> {
    /// Non-escaped chunks of string
    Literal(&'a str),
    /// Escaped chars
    Escaped(char),
    /// Escaped whitespace
    EscapedWS,
}

/// Parse parts of string
fn fragment<'a>(input: &'a str) -> CResult<&'a str, StringFragment<'a>> {
    context(
        "Fragment",
        alt((
            map(literal, StringFragment::Literal),
            map(escaped_char, StringFragment::Escaped),
            value(StringFragment::EscapedWS, escaped_whitespace),
        )),
    )(input)
}

fn build_string<'a>(input: &'a str) -> CResult<&'a str, String> {
    context(
        "BuildString",
        fold_many0(fragment, String::new, |mut acc, fragment| {
            match fragment {
                StringFragment::Escaped(c) => acc.push(c),
                StringFragment::EscapedWS => {}
                StringFragment::Literal(s) => acc.push_str(s),
            };
            acc
        }),
    )(input)
}

pub(crate) fn string_value<'a>(input: &'a str) -> CResult<&'a str, String> {
    let single_quoted = context(
        "SingleQuoted",
        delimited(char('\''), *(&build_string), char('\'')),
    );

    let double_quoted = context(
        "DoubleQuoted",
        delimited(char('"'), *(&build_string), char('"')),
    );

    context("String", alt((single_quoted, double_quoted)))(input)
}

#[cfg(test)]
mod test {
    #[test]
    fn test_simple_string() {
        assert_eq!(
            super::string_value("\"a simple string\""),
            Ok(("", String::from("a simple string"))),
            "Should parse a string with double quotes"
        );

        assert_eq!(
            super::string_value("'a simple string'"),
            Ok(("", String::from("a simple string"))),
            "Should parse a string with single quotes"
        );
    }

    #[test]
    fn test_string_with_escaped() {
        assert_eq!(
            super::string_value("\"an escaped \\\" and \\t and \\\' string\""),
            Ok(("", String::from("an escaped \" and \t and ' string"))),
            "Should parse an escaped string with single quotes"
        );

        assert_eq!(
            super::string_value("'an escaped \\\" and \\t and \\\' string'"),
            Ok(("", String::from("an escaped \" and \t and ' string"))),
            "Should parse an escaped string with single quotes"
        );
    }

    #[test]
    fn test_nom_example() {
        assert_eq!(
            super::string_value(
                "\"tab:\\tafter tab, newline:\\nnew line, quote: \\\", emoji: \\u{1F602}, newline:\\nescaped whitespace: \\    abc\""
            ),
            Ok(("", String::from("tab:\tafter tab, newline:\nnew line, quote: \", emoji: 😂, newline:\nescaped whitespace: abc"))),
            "Should parse nom example with single quotes"
        );

        assert_eq!(
            super::string_value("'tab:\\tafter tab, newline:\\nnew line, quote: \\\", emoji: \\u{1F602}, newline:\\nescaped whitespace: \\    abc'"),
            Ok(("", String::from("tab:\tafter tab, newline:\nnew line, quote: \", emoji: 😂, newline:\nescaped whitespace: abc"))),
            "Should parse nom example with single quotes"
        );
    }
}
