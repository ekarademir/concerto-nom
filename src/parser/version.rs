use nom::{
    branch::alt,
    bytes::complete::{tag, take_while},
    character::{
        complete::{alpha1, digit1, u128},
        is_alphanumeric,
    },
    combinator::{eof, not, recognize},
    error::{context, ContextError, ParseError},
    sequence::{pair, preceded, tuple},
    IResult, Parser,
};

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct ParsedVersion {
    major: u128,
    minor: u128,
    patch: u128,
}

impl From<(u128,)> for ParsedVersion {
    fn from(value: (u128,)) -> Self {
        ParsedVersion {
            major: value.0,
            minor: 0,
            patch: 0,
        }
    }
}

impl From<(u128, u128)> for ParsedVersion {
    fn from(value: (u128, u128)) -> Self {
        ParsedVersion {
            major: value.0,
            minor: value.1,
            patch: 0,
        }
    }
}

impl From<(u128, u128, u128)> for ParsedVersion {
    fn from(value: (u128, u128, u128)) -> Self {
        ParsedVersion {
            major: value.0,
            minor: value.1,
            patch: value.2,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum ModelVersion {
    Version(ParsedVersion),
    VersionWithRelease(ParsedVersion, String),
}

fn major_only_version_parser<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, ParsedVersion, E> {
    context(
        "VersionMajorOnly",
        digit1::<&'a str, E>.and_then(u128).map(|m| (m,).into()),
    )(input)
}

fn major_minor_version_parser<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, ParsedVersion, E> {
    context(
        "VersionMajorMinor",
        tuple((u128, tag("."), u128)).map(|(maj, _, min)| (maj, min).into()),
    )(input)
}

fn major_minor_patch_version_parser<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, ParsedVersion, E> {
    context(
        "VersionMajorMinorPatch",
        tuple((u128, tag("."), u128, tag("."), u128))
            .map(|(maj, _, min, _, pat)| (maj, min, pat).into()),
    )(input)
}

/// Parses a semantic version_parser, without the pre-release part
fn version_number_parser<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, ParsedVersion, E> {
    context(
        "Version",
        alt((
            major_minor_patch_version_parser,
            major_minor_version_parser,
            major_only_version_parser,
        )),
    )(input)
}

/// Parses hyphen followed by at least one alpha numeric character, and dots and dashes.
/// Numeric idenfifiers MUST NOT include leading zeros, single zero is fine.
/// https://semver.org/#spec-item-9
fn pre_release_parser<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, &'a str, E> {
    fn pre_release_allowed_parser<'a, E: ParseError<&'a str>>(
        input: &'a str,
    ) -> IResult<&'a str, &'a str, E> {
        let allowed = ".-";
        take_while::<_, _, _>(|c: char| is_alphanumeric(c as u8) || allowed.contains(c))(input)
    }

    let leading_no_zero = context(
        "PreReleaseNoLeadingZero",
        alt((
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
        )),
    );
    let leading_single_zero = context(
        "PreReleaseLeadingZero",
        pair(tag::<&'a str, &'a str, E>("0"), not(tag("0"))),
    );

    // Either start with one zero and follow by allowed characters, or start with non zero.
    let combined = alt((
        recognize(pair(leading_no_zero, pre_release_allowed_parser)),
        recognize(pair(leading_single_zero, pre_release_allowed_parser)),
    ));

    context("PreRelease", preceded(tag("-"), combined))(input)
}

pub fn version_parser<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, ModelVersion, E> {
    let (remains, (ver, maybe_pre)) = context(
        "Version",
        version_number_parser.and(alt((pre_release_parser, eof))),
    )(input)?;

    match maybe_pre.len() {
        0 => Ok((remains, ModelVersion::Version(ver))),
        _ => Ok((
            remains,
            ModelVersion::VersionWithRelease(ver, maybe_pre.to_string()),
        )),
    }
}

#[cfg(test)]
mod test {
    use super::ModelVersion;
    use nom::error::VerboseError;

    #[test]
    fn test_pre_release() {
        assert!(
            super::pre_release_parser::<VerboseError<&str>>("pr123").is_err(),
            "Should not parse if tag doesn't start with hyphen"
        );
        assert_eq!(
            super::pre_release_parser::<VerboseError<&str>>("-pr123"),
            Ok(("", "pr123")),
            "Should parse prerelease tag with letters and numbers"
        );
        assert_eq!(
            super::pre_release_parser::<VerboseError<&str>>("-0.1.pr123"),
            Ok(("", "0.1.pr123")),
            "Should parse prerelease tag with letters and numbers and dots"
        );
        assert_eq!(
            super::pre_release_parser::<VerboseError<&str>>("-alpha"),
            Ok(("", "alpha")),
            "Should parse prerelease tag with letters only"
        );
        assert_eq!(
            super::pre_release_parser::<VerboseError<&str>>("-alpha.1"),
            Ok(("", "alpha.1")),
            "Should parse prerelease tag with letters and numbers separated by dots"
        );
        assert!(super::pre_release_parser::<VerboseError<&str>>("-001").is_err());
        assert_eq!(
            super::pre_release_parser::<VerboseError<&str>>("-0.3.7"),
            Ok(("", "0.3.7")),
            "Should parse prerelease tag with numbers and dots"
        );
        assert_eq!(
            super::pre_release_parser::<VerboseError<&str>>("-x.7.z.92"),
            Ok(("", "x.7.z.92")),
            "Should the example prerelease tag from semver.org"
        );
        assert_eq!(
            super::pre_release_parser::<VerboseError<&str>>("-x-y-z.--"),
            Ok(("", "x-y-z.--")),
            "Should the example prerelease tag from semver.org"
        );
    }

    #[test]
    fn test_version() {
        assert_eq!(
            super::version_parser::<VerboseError<&str>>("12"),
            Ok(("", ModelVersion::Version((12,).into()))),
            "Should parse major only version_parser",
        );
        assert_eq!(
            super::version_parser::<VerboseError<&str>>("12-pre"),
            Ok((
                "",
                ModelVersion::VersionWithRelease((12,).into(), "pre".to_string()),
            )),
            "Should parse major only version_parser with pre-release tag",
        );
        assert_eq!(
            super::version_parser::<VerboseError<&str>>("12.13"),
            Ok(("", ModelVersion::Version((12, 13).into()))),
            "Should parse major.minor version_parser",
        );
        assert_eq!(
            super::version_parser::<VerboseError<&str>>("12.13-pre"),
            Ok((
                "",
                ModelVersion::VersionWithRelease((12, 13).into(), "pre".to_string())
            )),
            "Should parse major.minor version_parser with pre-release tag",
        );
        assert_eq!(
            super::version_parser::<VerboseError<&str>>("12.13.14"),
            Ok(("", ModelVersion::Version((12, 13, 14).into()))),
            "Should parse major.minor.patch version_parser",
        );
        assert_eq!(
            super::version_parser::<VerboseError<&str>>("12.13.14-0.1.pr123"),
            Ok((
                "",
                ModelVersion::VersionWithRelease((12, 13, 14).into(), "0.1.pr123".to_string())
            )),
            "Should parse major.minor.patch version_parser with pre-release tag",
        );
        assert_eq!(
            super::version_parser::<VerboseError<&str>>("1.0.0-alpha"),
            Ok((
                "",
                ModelVersion::VersionWithRelease((1, 0, 0).into(), "alpha".to_string())
            )),
            "Should parse major.minor.patch version_parser with pre-release tag when tag is all letters",
        );
        assert_eq!(
            super::version_parser::<VerboseError<&str>>("1.0.0-alpha.1"),
            Ok((
                "",
                ModelVersion::VersionWithRelease((1, 0, 0).into(), "alpha.1".to_string())
            )),
            "Should parse major.minor.patch version_parser with pre-release tag when tag has dots",
        );
    }
}
