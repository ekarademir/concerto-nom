use nom::{
    bytes::complete::tag,
    character::complete::space1,
    error::{context, ContextError, ParseError},
    sequence::{pair, preceded, separated_pair},
    IResult, Parser,
};

use super::common::token_parser;
use super::version::{version_parser, SemanticVersion};

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

fn namespace_version_parser<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, (String, SemanticVersion), E> {
    context(
        "Namespace",
        separated_pair(token_parser, tag("@"), version_parser::<E>)
            .map(|(name, ver)| (name.to_string(), ver)),
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
