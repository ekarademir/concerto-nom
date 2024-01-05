use std::fmt::format;

use nom::{
    branch::alt,
    bytes::complete::{tag, take_while},
    character::{
        complete::{alpha1, digit1, u128},
        is_alphanumeric,
    },
    combinator::{eof, not, recognize},
    error::context,
    sequence::{pair, preceded, tuple},
    Parser,
};

use crate::parser::CResult;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct VersionNumber {
    major: u128,
    minor: u128,
    patch: u128,
}

impl From<&VersionNumber> for String {
    fn from(value: &VersionNumber) -> Self {
        format!("{}.{}.{}", value.major, value.minor, value.patch)
    }
}

impl From<(u128,)> for VersionNumber {
    fn from(value: (u128,)) -> Self {
        VersionNumber {
            major: value.0,
            minor: 0,
            patch: 0,
        }
    }
}

impl From<(u128, u128)> for VersionNumber {
    fn from(value: (u128, u128)) -> Self {
        VersionNumber {
            major: value.0,
            minor: value.1,
            patch: 0,
        }
    }
}

impl From<(u128, u128, u128)> for VersionNumber {
    fn from(value: (u128, u128, u128)) -> Self {
        VersionNumber {
            major: value.0,
            minor: value.1,
            patch: value.2,
        }
    }
}

/// Representation of semantic version
/// It can have a pre-release tag attached or not
#[derive(Debug, Eq, PartialEq, Clone)]
pub enum SemanticVersion {
    Version(VersionNumber),
    VersionWithRelease(VersionNumber, String),
}

impl From<&SemanticVersion> for String {
    fn from(value: &SemanticVersion) -> Self {
        match value {
            SemanticVersion::Version(v) => format!("{}", String::from(v)),
            SemanticVersion::VersionWithRelease(v, r) => format!("{}-{}", String::from(v), r),
        }
    }
}

fn major_only_version<'a>(input: &'a str) -> CResult<&'a str, VersionNumber> {
    context(
        "VersionMajorOnly",
        digit1.and_then(u128).map(|m| (m,).into()),
    )(input)
}

fn major_minor_version<'a>(input: &'a str) -> CResult<&'a str, VersionNumber> {
    context(
        "VersionMajorMinor",
        tuple((u128, tag("."), u128)).map(|(maj, _, min)| (maj, min).into()),
    )(input)
}

fn major_minor_patch_version<'a>(input: &'a str) -> CResult<&'a str, VersionNumber> {
    context(
        "VersionMajorMinorPatch",
        tuple((u128, tag("."), u128, tag("."), u128))
            .map(|(maj, _, min, _, pat)| (maj, min, pat).into()),
    )(input)
}

/// Parses a semantic version, without the pre-release part
pub fn version_number<'a>(input: &'a str) -> CResult<&'a str, VersionNumber> {
    context(
        "Version",
        alt((
            major_minor_patch_version,
            major_minor_version,
            major_only_version,
        )),
    )(input)
}

fn pre_release_allowed<'a>(input: &'a str) -> CResult<&'a str, &'a str> {
    let allowed = ".-";
    take_while::<_, _, _>(|c: char| is_alphanumeric(c as u8) || allowed.contains(c))(input)
}

/// Parses hyphen followed by at least one alpha numeric character, and dots and dashes.
/// Numeric idenfifiers MUST NOT include leading zeros, single zero is fine.
/// https://semver.org/#spec-item-9
pub(crate) fn pre_release_token<'a>(input: &'a str) -> CResult<&'a str, &'a str> {
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
    let leading_single_zero = context("PreReleaseLeadingZero", pair(tag("0"), not(tag("0"))));

    // Either start with one zero and follow by allowed characters, or start with non zero.
    let combined = alt((
        recognize(pair(leading_no_zero, pre_release_allowed)),
        recognize(pair(leading_single_zero, pre_release_allowed)),
    ));

    context("PreReleaseToken", combined)(input)
}

fn pre_release<'a>(input: &'a str) -> CResult<&'a str, &'a str> {
    context("PreRelease", preceded(tag("-"), pre_release_token))(input)
}

/// A version can be provided as major, major.minor, major.minor.patch and
/// each with a pre-release tag attached with an hyphen
pub fn version_identifier<'a>(input: &'a str) -> CResult<&'a str, SemanticVersion> {
    let (remains, (ver, maybe_pre)) =
        context("Version", version_number.and(alt((pre_release, eof))))(input)?;

    match maybe_pre.len() {
        0 => Ok((remains, SemanticVersion::Version(ver))),
        _ => Ok((
            remains,
            SemanticVersion::VersionWithRelease(ver, maybe_pre.to_string()),
        )),
    }
}

#[cfg(test)]
mod test {
    use super::SemanticVersion;

    #[test]
    fn test_pre_release() {
        assert!(
            super::pre_release("pr123").is_err(),
            "Should not parse if tag doesn't start with hyphen"
        );
        assert_eq!(
            super::pre_release("-pr123"),
            Ok(("", "pr123")),
            "Should parse prerelease tag with letters and numbers"
        );
        assert_eq!(
            super::pre_release("-0.1.pr123"),
            Ok(("", "0.1.pr123")),
            "Should parse prerelease tag with letters and numbers and dots"
        );
        assert_eq!(
            super::pre_release("-alpha"),
            Ok(("", "alpha")),
            "Should parse prerelease tag with letters only"
        );
        assert_eq!(
            super::pre_release("-alpha.1"),
            Ok(("", "alpha.1")),
            "Should parse prerelease tag with letters and numbers separated by dots"
        );
        assert!(super::pre_release("-001").is_err());
        assert_eq!(
            super::pre_release("-0.3.7"),
            Ok(("", "0.3.7")),
            "Should parse prerelease tag with numbers and dots"
        );
        assert_eq!(
            super::pre_release("-x.7.z.92"),
            Ok(("", "x.7.z.92")),
            "Should the example prerelease tag from semver.org"
        );
        assert_eq!(
            super::pre_release("-x-y-z.--"),
            Ok(("", "x-y-z.--")),
            "Should the example prerelease tag from semver.org"
        );
    }

    #[test]
    fn test_version() {
        assert_eq!(
            super::version_identifier("12"),
            Ok(("", SemanticVersion::Version((12,).into()))),
            "Should parse major only version_identifier",
        );
        assert_eq!(
            super::version_identifier("12-pre"),
            Ok((
                "",
                SemanticVersion::VersionWithRelease((12,).into(), "pre".to_string()),
            )),
            "Should parse major only version_identifier with pre-release tag",
        );
        assert_eq!(
            super::version_identifier("12.13"),
            Ok(("", SemanticVersion::Version((12, 13).into()))),
            "Should parse major.minor version_identifier",
        );
        assert_eq!(
            super::version_identifier("12.13-pre"),
            Ok((
                "",
                SemanticVersion::VersionWithRelease((12, 13).into(), "pre".to_string())
            )),
            "Should parse major.minor version_identifier with pre-release tag",
        );
        assert_eq!(
            super::version_identifier("12.13.14"),
            Ok(("", SemanticVersion::Version((12, 13, 14).into()))),
            "Should parse major.minor.patch version_identifier",
        );
        assert_eq!(
            super::version_identifier("12.13.14-0.1.pr123"),
            Ok((
                "",
                SemanticVersion::VersionWithRelease((12, 13, 14).into(), "0.1.pr123".to_string())
            )),
            "Should parse major.minor.patch version_identifier with pre-release tag",
        );
        assert_eq!(
            super::version_identifier("1.0.0-alpha"),
            Ok((
                "",
                SemanticVersion::VersionWithRelease((1, 0, 0).into(), "alpha".to_string())
            )),
            "Should parse major.minor.patch version_identifier with pre-release tag when tag is all letters",
        );
        assert_eq!(
            super::version_identifier("1.0.0-alpha.1"),
            Ok((
                "",
                SemanticVersion::VersionWithRelease((1, 0, 0).into(), "alpha.1".to_string())
            )),
            "Should parse major.minor.patch version_identifier with pre-release tag when tag has dots",
        );
    }
}
