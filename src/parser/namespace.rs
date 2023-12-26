use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{anychar, space1},
    combinator::peek,
    error::{context, ContextError, ParseError},
    multi::many_till,
    sequence::{pair, preceded, separated_pair, terminated, tuple},
    IResult, Parser,
};

use super::common::token_parser;
use super::version::{
    pre_release_token_parser, version_number_parser, version_parser, SemanticVersion,
};

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

fn namespace_version_parser<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, (String, SemanticVersion), E> {
    context(
        "Namespace",
        separated_pair(token_parser, tag("@"), version_parser::<E>)
            .map(|(name, ver)| (name.to_string(), ver)),
    )(input)
}

fn fqn_no_prerelease_parser<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, FullyQualifiedName, E> {
    context(
        "FQNNoPrerelease",
        tuple((
            token_parser,
            tag("@"),
            version_number_parser,
            tag("."),
            token_parser,
        ))
        .map(|(namespace_name, _, version_number, _, type_name)| {
            (
                namespace_name.to_string(),
                SemanticVersion::Version(version_number),
                type_name.to_string(),
            )
                .into()
        }),
    )(input)
}

/// Try to pick up the last dot delimited bit of the pre-release
/// since dots are valid prerelease characters.
fn prerelease_dot_token_parser<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, (&'a str, &'a str), E> {
    // First try to pick up the last bit separated by a dot
    let (_, (_, (_, token))) = context(
        "PrereleaseDotToken::Token",
        many_till(
            anychar,
            pair(tag::<&'a str, &'a str, E>("."), token_parser::<E>),
        ),
    )(input)?;

    // Then parse the prerelease, which will also consume the dot
    let (rest, pre_with_token) =
        context("PrereleaseDotToken::PreRelease", pre_release_token_parser)(input)?;

    // Finally calculate the end of the pre (without the lat doted bit)
    let end_of_pre = pre_with_token.len() - token.len() - 1; // -1 for "dot"

    Ok((rest, (&(pre_with_token[..end_of_pre]), token)))
}

fn fqn_with_prerelease_parser<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, FullyQualifiedName, E> {
    context(
        "FQNWithPrerelease",
        tuple((
            token_parser,
            tag("@"),
            version_number_parser,
            tag("-"),
            prerelease_dot_token_parser,
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

pub fn fqn_parser<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, FullyQualifiedName, E> {
    context(
        "FullyQualifiedName",
        alt((fqn_with_prerelease_parser, fqn_no_prerelease_parser)),
    )(input)
}

pub fn namespace_parser<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, Namespace, E> {
    context(
        "Namespace",
        preceded(pair(tag("namespace"), space1), namespace_version_parser)
            .map(|parsed_nv| parsed_nv.into()),
    )(input)
}

#[cfg(test)]
mod test {
    use super::SemanticVersion;
    use nom::error::VerboseError;

    #[test]
    fn test_prerelease_and_token() {
        assert_eq!(
            super::prerelease_dot_token_parser::<VerboseError<&str>>("pre.bar123"),
            Ok(("", ("pre", "bar123")))
        )
    }

    #[test]
    fn test_fqn() {
        assert_eq!(
            super::fqn_parser::<VerboseError<&str>>("test@12.13.14.Foo"),
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
            super::fqn_parser::<VerboseError<&str>>("test@12.13.14-pre.bar123"),
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
            super::fqn_parser::<VerboseError<&str>>("test@12.13.14-pre.0.1.bar123"),
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
            super::namespace_version_parser::<VerboseError<&str>>("test@12.13.14"),
            Ok((
                "",
                (
                    "test".to_string(),
                    SemanticVersion::Version((12, 13, 14).into())
                )
            )),
        );
        assert_eq!(
            super::namespace_version_parser::<VerboseError<&str>>("test@12.13.14-pre"),
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
            super::namespace_parser::<VerboseError<&str>>("namespace  test@1.0.2"),
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
            super::namespace_parser::<VerboseError<&str>>("namespace  test@1.0.2-beta"),
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
