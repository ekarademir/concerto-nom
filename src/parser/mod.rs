pub mod common;
pub mod error;
pub mod namespace;
pub mod property;
pub mod version;

pub use namespace::namespace_definition_parser;

use crate::parser::error::CError;
use nom::IResult;

/// Concerto parse result type
pub type CResult<I, O> = IResult<I, O, CError<I>>;

#[cfg(test)]
mod test {
    #[test]
    fn test_a() {
        assert!(true);
    }
}
