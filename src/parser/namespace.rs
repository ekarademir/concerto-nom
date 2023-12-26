use nom::{
    bytes::complete::tag,
    character::complete::space1,
    error::{context, ContextError, ParseError, VerboseError},
    sequence::separated_pair,
    IResult, Parser,
};

use super::common::token_parser;
use super::version::{version_parser, SemanticVersion};
use super::BuiltIn;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct NamespaceVersion((String, SemanticVersion));

fn namespace_version_parser<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, NamespaceVersion, E> {
    context(
        "NamespaceVersion",
        separated_pair(token_parser, tag("@"), version_parser::<E>).map(
            |(parsed_token, parsed_version)| {
                NamespaceVersion((parsed_token.to_string(), parsed_version))
            },
        ),
    )(input)
}

pub fn namespace_parser<'a>(input: &'a str) -> IResult<&'a str, BuiltIn, VerboseError<&'a str>> {
    let mut parser = separated_pair(
        tag("namespace"),
        space1::<_, VerboseError<&'a str>>,
        namespace_version_parser::<VerboseError<&'a str>>,
    );

    let (rest, (_namespace_tag, parsed_fqn)) = parser(input)?;

    return Ok((rest, BuiltIn::Namespace(parsed_fqn)));
}

#[cfg(test)]
mod test {
    use super::{BuiltIn, NamespaceVersion, SemanticVersion};
    use nom::error::VerboseError;

    #[test]
    fn test_namespace_version() {
        assert_eq!(
            super::namespace_version_parser::<VerboseError<&str>>("test@12.13.14"),
            Ok((
                "",
                NamespaceVersion((
                    "test".to_string(),
                    SemanticVersion::Version((12, 13, 14).into())
                ))
            )),
        );
        assert_eq!(
            super::namespace_version_parser::<VerboseError<&str>>("test@12.13.14-pre"),
            Ok((
                "",
                NamespaceVersion((
                    "test".to_string(),
                    SemanticVersion::VersionWithRelease((12, 13, 14).into(), "pre".to_string())
                ))
            ))
        );
    }

    #[test]
    fn test_namespace() {
        assert_eq!(
            super::namespace_parser("namespace  test@1.0.2"),
            Ok((
                "",
                BuiltIn::Namespace(NamespaceVersion((
                    "test".to_string(),
                    SemanticVersion::Version((1, 0, 2).into())
                )))
            ))
        );
        assert_eq!(
            super::namespace_parser("namespace  test@1.0.2-beta"),
            Ok((
                "",
                BuiltIn::Namespace(NamespaceVersion((
                    "test".to_string(),
                    SemanticVersion::VersionWithRelease((1, 0, 2).into(), "beta".to_string())
                )))
            ))
        );
    }
}
