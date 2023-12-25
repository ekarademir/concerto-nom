use nom::{
    branch::alt,
    bytes::complete::{tag, take_while},
    character::{
        complete::{alpha1, alphanumeric0, space1, u128},
        is_alphanumeric,
    },
    combinator::{not, recognize},
    error::{ParseError, VerboseError},
    sequence::{pair, preceded, separated_pair, terminated},
    IResult, Parser,
};

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Version((u128, u128, u128, Option<String>));

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct FullyQualifiedName((String, Version));

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum BuiltIn {
    Namespace(FullyQualifiedName),
}

fn version_parser<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, u128, E> {
    // Parse a number followed by a dot, then discard the dot
    terminated(u128, tag("."))(input)
}

fn pre_release_parser<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, &'a str, E> {
    // Parse hyphen followed by at least one alpha numeric character, and dots and dashes.
    // Numeric idenfifiers MUST NOT include leading zeros, single zero is fine.
    // https://semver.org/#spec-item-9

    fn pre_release_allowed_parser<'a, E: ParseError<&'a str>>(
        input: &'a str,
    ) -> IResult<&'a str, &'a str, E> {
        let allowed = ".-";
        take_while::<_, _, _>(|c: char| is_alphanumeric(c as u8) || allowed.contains(c))(input)
    }

    let leading_no_zero = alt((
        tag("1"),
        tag("2"),
        tag("3"),
        tag("4"),
        tag("5"),
        tag("6"),
        tag("7"),
        tag("8"),
        tag("9"),
        tag("-"),
        tag("."),
        alpha1,
    ));
    let leading_single_zero = pair(tag::<&'a str, &'a str, E>("0"), not(tag("0")));

    let combined = alt((
        recognize(pair(leading_no_zero, pre_release_allowed_parser)),
        recognize(pair(leading_single_zero, pre_release_allowed_parser)),
    ));

    preceded(tag("-"), combined)(input)
}

fn version<'a>(input: &'a str) -> IResult<&'a str, Version, VerboseError<&'a str>> {
    let (remains, ((major_version, minor_version), patch)) =
        version_parser.and(version_parser).and(u128).parse(input)?;

    match pre_release_parser::<VerboseError<&'a str>>(remains) {
        Ok((remains, pre_release)) => Ok((
            remains,
            Version((
                major_version,
                minor_version,
                patch,
                Some(pre_release.to_string()),
            )),
        )),
        Err(_) => Ok((
            remains,
            Version((major_version, minor_version, patch, None)),
        )),
    }
}

fn token<'a>(input: &'a str) -> IResult<&'a str, String, VerboseError<&'a str>> {
    let mut token_parser = pair(alpha1, alphanumeric0);
    let (rest, (a, b)) = token_parser(input)?;
    return Ok((rest, a.to_string() + b));
}

fn fqn_parser<'a>(input: &'a str) -> IResult<&'a str, FullyQualifiedName, VerboseError<&'a str>> {
    let mut parser = separated_pair(token, tag("@"), version);
    let (rest, (parsed_token, parsed_version)) = parser(input)?;
    return Ok((rest, FullyQualifiedName((parsed_token, parsed_version))));
}

pub fn namespace<'a>(input: &'a str) -> IResult<&'a str, BuiltIn, VerboseError<&'a str>> {
    let mut namespace_parser = separated_pair(
        tag("namespace"),
        space1::<_, VerboseError<&'a str>>,
        fqn_parser,
    );

    let (rest, (_namespace_tag, parsed_fqn)) = namespace_parser(input)?;

    return Ok((rest, BuiltIn::Namespace(parsed_fqn)));
}

#[cfg(test)]
mod test {
    use super::{BuiltIn, FullyQualifiedName, Version};

    #[test]
    fn test_token() {
        assert_eq!(super::token("a123"), Ok(("", String::from("a123"))));
    }

    #[test]
    fn test_namespace() {
        assert_eq!(
            super::namespace("namespace  test@1.0.2"),
            Ok((
                "",
                BuiltIn::Namespace(FullyQualifiedName((
                    "test".to_string(),
                    Version((1, 0, 2, None))
                )))
            ))
        );
        assert_eq!(
            super::namespace("namespace  test@1.0.2-beta"),
            Ok((
                "",
                BuiltIn::Namespace(FullyQualifiedName((
                    "test".to_string(),
                    Version((1, 0, 2, Some("beta".to_string())))
                )))
            ))
        );
    }

    #[test]
    fn test_pre_release() {
        assert_eq!(
            super::pre_release_parser::<nom::error::VerboseError<&str>>("-pr123"),
            Ok(("", "pr123"))
        );
        assert_eq!(
            super::pre_release_parser::<nom::error::VerboseError<&str>>("-0.1.pr123"),
            Ok(("", "0.1.pr123"))
        );
        assert_eq!(
            super::pre_release_parser::<nom::error::VerboseError<&str>>("-alpha"),
            Ok(("", "alpha"))
        );
        assert_eq!(
            super::pre_release_parser::<nom::error::VerboseError<&str>>("-alpha.1"),
            Ok(("", "alpha.1"))
        );
        assert!(super::pre_release_parser::<nom::error::VerboseError<&str>>("-001").is_err());
        assert_eq!(
            super::pre_release_parser::<nom::error::VerboseError<&str>>("-0.3.7"),
            Ok(("", "0.3.7"))
        );
        assert_eq!(
            super::pre_release_parser::<nom::error::VerboseError<&str>>("-x.7.z.92"),
            Ok(("", "x.7.z.92"))
        );
        assert_eq!(
            super::pre_release_parser::<nom::error::VerboseError<&str>>("-x-y-z.--"),
            Ok(("", "x-y-z.--"))
        );
    }

    #[test]
    fn test_version() {
        assert_eq!(
            super::version("12.13.14"),
            Ok(("", Version((12, 13, 14, None))))
        );
        assert_eq!(
            super::version("12.13.14-0.1.pr123"),
            Ok(("", Version((12, 13, 14, Some("0.1.pr123".to_string())))))
        );
        assert_eq!(
            super::version("1.0.0-alpha"),
            Ok(("", Version((1, 0, 0, Some("alpha".to_string())))))
        );
        assert_eq!(
            super::version("1.0.0-alpha.1"),
            Ok(("", Version((1, 0, 0, Some("alpha.1".to_string())))))
        );
    }

    #[test]
    fn test_fqn() {
        assert_eq!(
            super::fqn_parser("test@12.13.14"),
            Ok((
                "",
                FullyQualifiedName(("test".to_string(), Version((12, 13, 14, None))))
            ))
        );
    }
}
