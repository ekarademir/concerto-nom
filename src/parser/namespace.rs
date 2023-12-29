use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{anychar, space1},
    combinator::{into, recognize},
    error::context,
    multi::{many_till, separated_list1},
    sequence::{pair, preceded, separated_pair, tuple},
    Parser,
};

use super::common::token;
use super::version::{pre_release_token, version_identifier, version_number, SemanticVersion};
use crate::parser::{common::keywords, CResult};

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Namespace {
    name: String,
    version: SemanticVersion,
}

impl From<(String, SemanticVersion)> for Namespace {
    fn from(value: (String, SemanticVersion)) -> Self {
        Namespace {
            name: value.0,
            version: value.1,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct FullyQualifiedName {
    name: String,
    version: SemanticVersion,
    type_name: String,
}

impl From<(String, SemanticVersion, String)> for FullyQualifiedName {
    fn from(value: (String, SemanticVersion, String)) -> Self {
        FullyQualifiedName {
            name: value.0,
            version: value.1,
            type_name: value.2,
        }
    }
}

/// Namespaces are tokens and can be dot separated
fn namespace_name<'a>(input: &'a str) -> CResult<&'a str, &'a str> {
    context(
        "NamespaceToken",
        recognize(separated_list1(tag("."), token)),
    )(input)
}

fn namespace_version<'a>(input: &'a str) -> CResult<&'a str, (String, SemanticVersion)> {
    context(
        "Namespace",
        separated_pair(namespace_name, tag("@"), version_identifier)
            .map(|(name, ver)| (name.to_string(), ver)),
    )(input)
}

fn fqn_no_prerelease<'a>(input: &'a str) -> CResult<&'a str, FullyQualifiedName> {
    context(
        "FQNNoPrerelease",
        tuple((token, tag("@"), version_number, tag("."), token)).map(
            |(namespace_name, _, version_number, _, type_name)| {
                (
                    namespace_name.to_string(),
                    SemanticVersion::Version(version_number),
                    type_name.to_string(),
                )
                    .into()
            },
        ),
    )(input)
}

/// Try to pick up the last dot delimited bit of the pre-release
/// since dots are valid prerelease characters.
fn prerelease_dot_token<'a>(input: &'a str) -> CResult<&'a str, (&'a str, &'a str)> {
    // First try to pick up the last bit separated by a dot
    let (_, (_, (_, token))) = context(
        "PrereleaseDotToken::Token",
        many_till(anychar, pair(tag("."), token)),
    )(input)?;

    // Then parse the prerelease, which will also consume the dot
    let (rest, pre_with_token) =
        context("PrereleaseDotToken::PreRelease", pre_release_token)(input)?;

    // Finally calculate the end of the pre (without the lat doted bit)
    let end_of_pre = pre_with_token.len() - token.len() - 1; // -1 for "dot"

    Ok((rest, (&(pre_with_token[..end_of_pre]), token)))
}

fn fqn_with_prerelease<'a>(input: &'a str) -> CResult<&'a str, FullyQualifiedName> {
    context(
        "FQNWithPrerelease",
        tuple((
            token,
            tag("@"),
            version_number,
            tag("-"),
            prerelease_dot_token,
        ))
        .map(
            |(namespace_name, _, version_number, _, (pre_release, type_name))| {
                (
                    namespace_name.to_string(),
                    SemanticVersion::VersionWithRelease(version_number, pre_release.to_string()),
                    type_name.to_string(),
                )
                    .into()
            },
        ),
    )(input)
}

pub fn fqn<'a>(input: &'a str) -> CResult<&'a str, FullyQualifiedName> {
    context(
        "FullyQualifiedName",
        alt((fqn_with_prerelease, fqn_no_prerelease)),
    )(input)
}

pub fn namespace_identifier<'a>(input: &'a str) -> CResult<&'a str, Namespace> {
    context(
        "NamespaceDefinition",
        preceded(pair(keywords::namespace, space1), into(namespace_version)),
    )(input)
}

#[cfg(test)]
mod test {
    use super::SemanticVersion;

    #[test]
    fn test_prerelease_and_token() {
        assert_eq!(
            super::prerelease_dot_token("pre.bar123"),
            Ok(("", ("pre", "bar123")))
        )
    }

    #[test]
    fn test_fqn() {
        assert_eq!(
            super::fqn("test@12.13.14.Foo"),
            Ok((
                "",
                (
                    "test".to_string(),
                    SemanticVersion::Version((12, 13, 14).into()),
                    "Foo".to_string(),
                )
                    .into()
            )),
            "Should parse fully qualified name"
        );
        assert_eq!(
            super::fqn("test@12.13.14-pre.bar123"),
            Ok((
                "",
                (
                    "test".to_string(),
                    SemanticVersion::VersionWithRelease((12, 13, 14).into(), "pre".to_string()),
                    "bar123".to_string(),
                )
                    .into()
            )),
            "Should parse fully qualified name with pre-release"
        );
        assert_eq!(
            super::fqn("test@12.13.14-pre.0.1.bar123"),
            Ok((
                "",
                (
                    "test".to_string(),
                    SemanticVersion::VersionWithRelease((12, 13, 14).into(), "pre.0.1".to_string()),
                    "bar123".to_string(),
                )
                    .into()
            )),
            "Should parse fully qualified name with pre-release with dots"
        );
    }

    #[test]
    fn test_namespace_version() {
        assert_eq!(
            super::namespace_version("test@12.13.14"),
            Ok((
                "",
                (
                    "test".to_string(),
                    SemanticVersion::Version((12, 13, 14).into())
                )
            )),
        );
        assert_eq!(
            super::namespace_version("test@12.13.14-pre"),
            Ok((
                "",
                (
                    "test".to_string(),
                    SemanticVersion::VersionWithRelease((12, 13, 14).into(), "pre".to_string())
                )
            ))
        );
    }

    #[test]
    fn test_namespace() {
        assert_eq!(
            super::namespace_identifier("namespace  test@1.0.2"),
            Ok((
                "",
                (
                    "test".to_string(),
                    SemanticVersion::Version((1, 0, 2).into())
                )
                    .into()
            ))
        );
        assert_eq!(
            super::namespace_identifier("namespace  test@1.0.2-beta"),
            Ok((
                "",
                (
                    "test".to_string(),
                    SemanticVersion::VersionWithRelease((1, 0, 2).into(), "beta".to_string())
                )
                    .into()
            ))
        );
    }
}
