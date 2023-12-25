mod common;
mod version;

use nom::{
    bytes::complete::tag, character::complete::space1, error::VerboseError,
    sequence::separated_pair, IResult,
};

pub use common::token_parser;
pub use version::{version_number_parser, version_parser, SemanticVersion, VersionNumber};

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct FullyQualifiedName((String, SemanticVersion));

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum BuiltIn {
    Namespace(FullyQualifiedName),
}

fn fqn_parser<'a>(input: &'a str) -> IResult<&'a str, FullyQualifiedName, VerboseError<&'a str>> {
    let mut parser = separated_pair(
        token_parser,
        tag("@"),
        version_parser::<VerboseError<&'a str>>,
    );
    let (rest, (parsed_token, parsed_version)) = parser(input)?;
    return Ok((
        rest,
        FullyQualifiedName((parsed_token.to_string(), parsed_version)),
    ));
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
    use super::{BuiltIn, FullyQualifiedName, SemanticVersion};

    #[test]
    fn test_fqn() {
        assert_eq!(
            super::fqn_parser("test@12.13.14"),
            Ok((
                "",
                FullyQualifiedName((
                    "test".to_string(),
                    SemanticVersion::Version((12, 13, 14).into())
                ))
            )),
        );
        assert_eq!(
            super::fqn_parser("test@12.13.14-pre"),
            Ok((
                "",
                FullyQualifiedName((
                    "test".to_string(),
                    SemanticVersion::VersionWithRelease((12, 13, 14).into(), "pre".to_string())
                ))
            ))
        );
    }

    #[test]
    fn test_namespace() {
        assert_eq!(
            super::namespace("namespace  test@1.0.2"),
            Ok((
                "",
                BuiltIn::Namespace(FullyQualifiedName((
                    "test".to_string(),
                    SemanticVersion::Version((1, 0, 2).into())
                )))
            ))
        );
        assert_eq!(
            super::namespace("namespace  test@1.0.2-beta"),
            Ok((
                "",
                BuiltIn::Namespace(FullyQualifiedName((
                    "test".to_string(),
                    SemanticVersion::VersionWithRelease((1, 0, 2).into(), "beta".to_string())
                )))
            ))
        );
    }
}
